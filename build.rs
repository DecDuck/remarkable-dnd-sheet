// Build script for the cxx-qt based reMarkable calculator + ink app.
//
// Two parts:
//   1. The C++ ink rendering (InkCanvas : QQuickPaintedItem and the
//      EPScreenModeItem ABI stub) is compiled with a plain cc::Build, with moc
//      run on the InkCanvas header. We link the private libqsgepaper.so for the
//      EPScreenModeItem symbols.
//   2. The cxx-qt QML module exposes the Rust Calculator and PenInput QObjects
//      and registers the QML file.

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

    // --- compile the C++ ink sources ---
    let qt_modules = ["QtCore", "QtGui", "QtQml", "QtQuick"];
    let mut build = cc::Build::new();
    build.cpp(true).std("c++17").flag_if_supported("-fPIC");
    build.include(&qt_headers);
    for m in qt_modules {
        build.include(qt_headers.join(m));
    }
    build.include(manifest_dir.join("cpp"));
    build.file(manifest_dir.join("cpp/inkcanvas.cpp"));
    build.file(manifest_dir.join("cpp/ink_bridge.cpp"));
    build.file(&moc_out);
    build.compile("inkcpp");

    // --- link the private epaper plugin for EPScreenModeItem symbols ---
    let plugin_dir = qt_libs.join("plugins/scenegraph");
    println!("cargo:rustc-link-search=native={}", plugin_dir.display());
    println!("cargo:rustc-link-search=native={}", qt_libs.display());
    println!("cargo:rustc-link-lib=dylib=qsgepaper");
    // Resolve the plugin from the on-device path at runtime.
    println!("cargo:rustc-link-arg=-Wl,-rpath,/usr/lib/plugins/scenegraph");

    println!("cargo:rerun-if-changed=cpp/inkcanvas.cpp");
    println!("cargo:rerun-if-changed=cpp/inkcanvas.h");
    println!("cargo:rerun-if-changed=cpp/ink_bridge.cpp");
    println!("cargo:rerun-if-changed=cpp/epscreenmodeitem.h");

    // --- cxx-qt QML module (Calculator + PenInput) ---
    CxxQtBuilder::new_qml_module(
        QmlModule::new("com.remarkable.calc").qml_file("qml/main.qml"),
    )
    .qt_module("Gui")
    .qt_module("Qml")
    .qt_module("Quick")
    .files(["src/calculator.rs", "src/peninput.rs"])
    .build();
}
