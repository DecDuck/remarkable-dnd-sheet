// Build script for the cxx-qt based reMarkable D&D character sheet app.
//
// Three parts:
//   1. The C++ ink rendering (InkCanvas : QQuickPaintedItem and the
//      EPScreenModeItem ABI stub) is compiled with a plain cc::Build, with moc
//      run on the InkCanvas header. We link the private libqsgepaper.so for the
//      EPScreenModeItem symbols.
//   2. Optional PDF sheet embedding: if sheet.pdf exists in the project root,
//      tools/extract_fields.py is run to render a PNG background and extract
//      AcroForm field positions. The results are compiled into the binary as
//      Qt resources (qrc:/charsheet/bg.png and qrc:/charsheet/fields.json) via
//      rcc, and a Q_INIT_RESOURCE call in ink_bridge.cpp forces them to load.
//   3. The cxx-qt QML module exposes the Rust CharacterSheet and PenInput
//      QObjects and registers the QML file.

use cxx_qt_build::{CxxQtBuilder, QmlModule};
use std::env;
use std::path::PathBuf;
use std::process::Command;

fn qmake_query(qmake: &str, var: &str) -> String {
    let out = Command::new(qmake)
        .args(["-query", var])
        .output()
        .unwrap_or_else(|e| panic!("failed to run {qmake}: {e}"));
    String::from_utf8_lossy(&out.stdout).trim().to_string()
}

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    let qmake = env::var("QMAKE").unwrap_or_else(|_| "qmake6".to_string());
    let qt_headers = PathBuf::from(qmake_query(&qmake, "QT_INSTALL_HEADERS"));
    let qt_libs = PathBuf::from(qmake_query(&qmake, "QT_INSTALL_LIBS"));
    let host_libexec = PathBuf::from(qmake_query(&qmake, "QT_HOST_LIBEXECS"));
    let moc = host_libexec.join("moc");

    // --- moc the InkCanvas header (it declares Q_OBJECT) ---
    let ink_header = manifest_dir.join("cpp/inkcanvas.h");
    let moc_out = out_dir.join("moc_inkcanvas.cpp");
    let status = Command::new(&moc)
        .arg(&ink_header)
        .arg("-o")
        .arg(&moc_out)
        .status()
        .expect("failed to run moc");
    assert!(status.success(), "moc failed on inkcanvas.h");

    // --- moc ink_bridge.cpp (ScreenRefresher has Q_OBJECT inline) ---
    // The .cpp file includes "ink_bridge.moc" at the bottom; moc must be run
    // on the .cpp source so it sees the class definition.
    let bridge_src = manifest_dir.join("cpp/ink_bridge.cpp");
    let moc_bridge_out = out_dir.join("ink_bridge.moc");
    let status = Command::new(&moc)
        .arg(&bridge_src)
        .arg("-o")
        .arg(&moc_bridge_out)
        .status()
        .expect("failed to run moc on ink_bridge.cpp");
    assert!(status.success(), "moc failed on ink_bridge.cpp");

    // --- compile the C++ ink sources (deferred until after rcc runs below) ---
    // The cc::Build is constructed here but .compile() is called after the QRC
    // step so that charsheet_rcc.cpp can be added to the same archive.
    let qt_modules = ["QtCore", "QtGui", "QtQml", "QtQuick"];
    let mut build = cc::Build::new();
    build.cpp(true).std("c++17").flag_if_supported("-fPIC");
    build.include(&qt_headers);
    for m in qt_modules {
        build.include(qt_headers.join(m));
    }
    build.include(manifest_dir.join("cpp"));
    // out_dir must be on the include path so `#include "ink_bridge.moc"` resolves.
    build.include(&out_dir);
    build.file(manifest_dir.join("cpp/inkcanvas.cpp"));
    build.file(manifest_dir.join("cpp/ink_bridge.cpp"));
    build.file(&moc_out);

    println!("cargo:rerun-if-changed=cpp/inkcanvas.cpp");
    println!("cargo:rerun-if-changed=cpp/inkcanvas.h");
    println!("cargo:rerun-if-changed=cpp/ink_bridge.cpp");
    println!("cargo:rerun-if-changed=cpp/epscreenmodeitem.h");
    println!("cargo:rerun-if-changed=qml/main.qml");

    // --- optional PDF sheet embedding ---
    // Place sheet.pdf in the project root; `cargo build` does the rest.
    println!("cargo:rerun-if-changed=sheet.pdf");
    println!("cargo:rerun-if-changed=tools/extract_fields.py");

    let sheet_pdf = manifest_dir.join("sheet.pdf");
    let assets_dir = out_dir.join("charsheet_assets");
    let bg_png = assets_dir.join("charsheet_bg.png");
    let fields_json = assets_dir.join("charsheet_fields.json");

    let assets_ok = if sheet_pdf.exists() {
        std::fs::create_dir_all(&assets_dir).expect("failed to create charsheet_assets dir");

        // Use the python3 that ships with the host OS, not whatever the SDK
        // put on PATH (the SDK may override python3 with a cross-build variant
        // that lacks the host's site-packages like PyMuPDF).
        // Resolve the real host python3 by consulting /usr/bin directly, then
        // fall back to a bare "python3" search as last resort.
        let python = ["/usr/bin/python3", "/usr/local/bin/python3", "python3", "python"]
            .iter()
            .find(|p| {
                Command::new(p)
                    .arg("-c")
                    .arg("import fitz")
                    .output()
                    .map(|o| o.status.success())
                    .unwrap_or(false)
            })
            .copied()
            .unwrap_or_else(|| {
                panic!(
                    "No python3 with PyMuPDF (fitz) found. \
                     Install it with: sudo dnf install python3-pymupdf  \
                     or: pip install pymupdf"
                )
            });

        let script = manifest_dir.join("tools/extract_fields.py");
        let result = Command::new(python)
            .arg(&script)
            .arg(&sheet_pdf)
            .arg("--out-dir")
            .arg(&assets_dir)
            .output();

        match result {
            Ok(out) if out.status.success() && bg_png.exists() && fields_json.exists() => true,
            Ok(out) => {
                let stdout = String::from_utf8_lossy(&out.stdout);
                let stderr = String::from_utf8_lossy(&out.stderr);
                panic!(
                    "extract_fields.py failed (exit {}).\n\
                     stdout:\n{}\nstderr:\n{}",
                    out.status, stdout, stderr
                );
            }
            Err(e) => {
                panic!("Could not run python ({e}). Ensure python3 is on PATH with PyMuPDF installed.");
            }
        }
    } else {
        false
    };

    // Always write + compile a QRC so that qrc:/charsheet/* URLs exist.
    // When no sheet is provided the bundle is empty; Image and XHR fail
    // gracefully and the placeholder is shown instead.
    let qrc_path = out_dir.join("charsheet.qrc");

    // ── sheet_bindings.json ── optional field-ID → binding-key map ──────────
    // Place this file in the project root (alongside Cargo.toml) to enable
    // auto-calculation of values in specific PDF fields.  Use
    //   python3 tools/dump_fields.py --write-template
    // to generate a ready-to-edit template after extracting a PDF.
    println!("cargo:rerun-if-changed=sheet_bindings.json");
    let bindings_path = manifest_dir.join("sheet_bindings.json");
    let bindings_js = if bindings_path.exists() {
        let raw = std::fs::read_to_string(&bindings_path)
            .expect("failed to read sheet_bindings.json");
        // Validate it is a JSON object, then re-emit as a JS object literal.
        match serde_json::from_str::<serde_json::Value>(&raw) {
            Ok(serde_json::Value::Object(_)) => {
                // Compact single-line for embedding
                let compact = serde_json::to_string(
                    &serde_json::from_str::<serde_json::Value>(&raw).unwrap(),
                )
                .expect("failed to re-serialize bindings");
                println!(
                    "cargo:warning=sheet_bindings.json loaded ({} bytes)",
                    raw.len()
                );
                format!("var BINDINGS = {};\n", compact)
            }
            _ => {
                println!(
                    "cargo:warning=sheet_bindings.json is not a JSON object — bindings disabled"
                );
                "var BINDINGS = {};\n".to_string()
            }
        }
    } else {
        println!(
            "cargo:warning=No sheet_bindings.json found. \
             Run: python3 tools/dump_fields.py --write-template \
             to generate a template, then rebuild."
        );
        "var BINDINGS = {};\n".to_string()
    };

    // Generate a JS file with the field data embedded as a literal array.
    // Importing a JS module in QML is reliable across all Qt builds; using
    // XMLHttpRequest to read a qrc:// URL is not (Qt's XHR implementation
    // silently returns status 0 and an empty body for qrc:// on some builds).
    let fields_js_path = out_dir.join("charsheet_fields.js");
    let fields_js_content = if assets_ok {
        let raw = std::fs::read_to_string(&fields_json).expect("failed to read fields.json");
        format!(
            ".pragma library\nvar FIELDS = {};\n{}",
            raw.trim(),
            bindings_js,
        )
    } else {
        format!(".pragma library\nvar FIELDS = [];\n{}", bindings_js)
    };
    std::fs::write(&fields_js_path, &fields_js_content)
        .expect("failed to write charsheet_fields.js");

    let qrc_content = if assets_ok {
        format!(
            "<!DOCTYPE RCC>\n<RCC version=\"1.0\">\n  \
             <qresource prefix=\"/charsheet\">\n    \
             <file alias=\"bg.png\">{bg}</file>\n    \
             <file alias=\"fields.js\">{js}</file>\n  \
             </qresource>\n</RCC>\n",
            bg = bg_png.display(),
            js = fields_js_path.display(),
        )
    } else {
        // Always embed the JS so the QML import never fails.
        format!(
            "<!DOCTYPE RCC>\n<RCC version=\"1.0\">\n  \
             <qresource prefix=\"/charsheet\">\n    \
             <file alias=\"fields.js\">{js}</file>\n  \
             </qresource>\n</RCC>\n",
            js = fields_js_path.display(),
        )
    };
    std::fs::write(&qrc_path, &qrc_content).expect("failed to write charsheet.qrc");

    let rcc = host_libexec.join("rcc");
    let rcc_cpp = out_dir.join("charsheet_rcc.cpp");
    let status = Command::new(&rcc)
        .arg("-name")
        .arg("charsheet")
        .arg(&qrc_path)
        .arg("-o")
        .arg(&rcc_cpp)
        .status()
        .expect("failed to run rcc");
    assert!(status.success(), "rcc failed on charsheet.qrc");

    // Add the rcc output to the SAME archive as ink_bridge.cpp.
    // This is the critical step: qInitResources_charsheet (defined in
    // charsheet_rcc.cpp) and the Q_INIT_RESOURCE(charsheet) call in
    // ink_bridge.cpp must live in the same static lib. If they are in separate
    // archives, bfd-ld on the SDK toolchain cannot resolve the reference
    // because it processes each archive only once in left-to-right order.
    build.file(&rcc_cpp);
    build.compile("inkcpp");

    // Link directives printed AFTER compile() so that -linkcpp appears before
    // -lqsgepaper in the final link line.  With -Wl,--as-needed (injected by
    // rustc) a shared lib is dropped if no undefined references exist at the
    // point it is scanned; placing qsgepaper after inkcpp ensures the
    // EPScreenModeItem references from inkcpp are visible when qsgepaper is
    // encountered, so the SO is kept and the symbols resolve.
    let plugin_dir = qt_libs.join("plugins/scenegraph");
    println!("cargo:rustc-link-search=native={}", plugin_dir.display());
    println!("cargo:rustc-link-search=native={}", qt_libs.display());
    println!("cargo:rustc-link-lib=dylib=qsgepaper");
    println!("cargo:rustc-link-arg=-Wl,-rpath,/usr/lib/plugins/scenegraph");
    CxxQtBuilder::new_qml_module(
        QmlModule::new("com.remarkable.calc").qml_file("qml/main.qml"),
    )
    .qt_module("Gui")
    .qt_module("Qml")
    .qt_module("Quick")
    .files(["src/charsheet.rs", "src/peninput.rs"])
    .build();
}
