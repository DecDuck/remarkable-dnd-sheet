//! reMarkable Paper Pro — D&D 5e character sheet + ink — Rust + cxx-qt.
//!
//! The UI is QML; the character sheet state lives in the Rust `CharacterSheet`
//! QObject, and the marker/pen reader in `PenInput`. The ink surface is the C++
//! `InkCanvas` (QQuickPaintedItem, the epaper backend's supported draw path),
//! registered via `register_ink_types()`.

pub mod charsheet;
pub mod peninput;
pub mod dnd_data;
pub mod player;

use cxx_qt_lib::{QGuiApplication, QQmlApplicationEngine, QUrl};

extern "C" {
    // Defined in cpp/ink_bridge.cpp; registers InkCanvas under "InkTools 1.0".
    fn register_ink_types();
    // Forces the charsheet QRC resources (PNG background + fields JSON) to be
    // registered with Qt's resource system before the QML engine is created.
    fn init_charsheet_resources();
}

fn main() {
    // The reMarkable device only ships the "epaper" QPA plugin; Qt otherwise
    // defaults to "linuxfb" and aborts. Default to epaper unless the user has
    // explicitly chosen a platform (e.g. QT_QPA_PLATFORM=offscreen for tests).
    if std::env::var_os("QT_QPA_PLATFORM").is_none() {
        std::env::set_var("QT_QPA_PLATFORM", "epaper");
    }

    // Select the reMarkable epaper Qt Quick scene-graph backend
    // (libqsgepaper.so). Without it, Qt falls back to the generic "software"
    // backend, which renders into an offscreen buffer that never reaches the
    // e-ink panel (window stays blank). The epaper backend presents via
    // EPFramebuffer and is also what enables the fast Pen waveform.
    if std::env::var_os("QT_QUICK_BACKEND").is_none() {
        std::env::set_var("QT_QUICK_BACKEND", "epaper");
    }

    let mut app = QGuiApplication::new();

    // Register the embedded Qt resources (charsheet PNG + fields JSON compiled
    // in by build.rs from sheet.pdf via rcc). Must happen before the engine
    // loads so qrc:/charsheet/* URLs resolve.
    unsafe { init_charsheet_resources() };

    // Must run before any QML that imports InkTools is loaded.
    unsafe { register_ink_types() };

    let mut engine = QQmlApplicationEngine::new();

    if let Some(engine) = engine.as_mut() {
        engine.load(&QUrl::from(
            "qrc:/qt/qml/com/remarkable/calc/qml/main.qml",
        ));
    }

    if let Some(app) = app.as_mut() {
        app.exec();
    }
}
