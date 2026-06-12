//! Pen (marker) input QObject implemented in Rust.
//!
//! The reMarkable epaper platform plugin does not forward marker events into
//! the QML mouse/touch pipeline, so we read the digitizer directly from evdev.
//! A dedicated background thread blocks on the device and, once per SYN_REPORT
//! frame, marshals the new position onto the Qt thread via the cxx-qt threading
//! queue, where the QObject properties are updated and `penChanged` is emitted.
#![allow(non_snake_case)]

#[cxx_qt::bridge]
pub mod qobject {
    extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qproperty(f64, penX)]
        #[qproperty(f64, penY)]
        #[qproperty(f64, pressure)]
        #[qproperty(bool, penDown)]
        #[qproperty(bool, active)]
        #[qproperty(f64, screenWidth)]
        #[qproperty(f64, screenHeight)]
        type PenInput = super::PenInputRust;

        #[qsignal]
        #[cxx_name = "penChanged"]
        fn pen_changed(self: Pin<&mut PenInput>);

        #[qinvokable]
        fn open(self: Pin<&mut PenInput>);
    }

    impl cxx_qt::Threading for PenInput {}
}

use core::pin::Pin;
use cxx_qt::Threading;

#[derive(Default)]
pub struct PenInputRust {
    penX: f64,
    penY: f64,
    pressure: f64,
    penDown: bool,
    active: bool,
    screenWidth: f64,
    screenHeight: f64,
}

// One pen sample, already normalised to [0, 1] on each axis by the reader
// thread. Mapping to screen pixels happens on the Qt thread (where the current
// screen size lives).
struct Frame {
    norm_x: f64,
    norm_y: f64,
    pressure: f64,
    pen_down: bool,
}

impl qobject::PenInput {
    /// Detect the marker device and start the background reader thread.
    pub fn open(mut self: Pin<&mut Self>) {
        if *self.as_ref().active() {
            return;
        }

        let device = match find_pen_device() {
            Some(d) => d,
            None => {
                eprintln!("PenInput: no marker device found under /dev/input");
                return;
            }
        };

        self.as_mut().set_active(true);

        let qt_thread = self.qt_thread();
        std::thread::spawn(move || {
            reader_loop(device, qt_thread);
        });
    }
}

fn reader_loop(mut device: evdev::Device, qt_thread: cxx_qt::CxxQtThread<qobject::PenInput>) {
    use evdev::{AbsoluteAxisType, InputEventKind, Key};

    // Axis ranges, queried once up front.
    let (min_x, max_x, min_y, max_y, min_p, max_p) = match device.get_abs_state() {
        Ok(abs) => {
            let x = abs[AbsoluteAxisType::ABS_X.0 as usize];
            let y = abs[AbsoluteAxisType::ABS_Y.0 as usize];
            let p = abs[AbsoluteAxisType::ABS_PRESSURE.0 as usize];
            (
                x.minimum, x.maximum.max(x.minimum + 1),
                y.minimum, y.maximum.max(y.minimum + 1),
                p.minimum, p.maximum.max(p.minimum + 1),
            )
        }
        Err(_) => (0, 1, 0, 1, 0, 1),
    };

    let mut raw_x = 0i32;
    let mut raw_y = 0i32;
    let mut raw_p = 0i32;
    let mut pen_down = false;

    loop {
        let events = match device.fetch_events() {
            Ok(ev) => ev,
            Err(_) => break,
        };

        for event in events {
            match event.kind() {
                InputEventKind::AbsAxis(axis) => {
                    if axis == AbsoluteAxisType::ABS_X {
                        raw_x = event.value();
                    } else if axis == AbsoluteAxisType::ABS_Y {
                        raw_y = event.value();
                    } else if axis == AbsoluteAxisType::ABS_PRESSURE {
                        raw_p = event.value();
                    }
                }
                InputEventKind::Key(key) => {
                    // BTN_TOUCH = tip in contact (drawing). BTN_TOOL_PEN is only
                    // hover/proximity and must not count as a stroke.
                    if key == Key::BTN_TOUCH {
                        pen_down = event.value() != 0;
                    }
                }
                InputEventKind::Synchronization(_) => {
                    let norm_x = ((raw_x - min_x) as f64 / (max_x - min_x) as f64).clamp(0.0, 1.0);
                    let norm_y = ((raw_y - min_y) as f64 / (max_y - min_y) as f64).clamp(0.0, 1.0);
                    let pressure = ((raw_p - min_p) as f64 / (max_p - min_p) as f64).clamp(0.0, 1.0);

                    let frame = Frame { norm_x, norm_y, pressure, pen_down };

                    // Apply on the Qt thread; ignore if the object is gone.
                    let _ = qt_thread.queue(move |qobject| {
                        apply_frame(qobject, frame);
                    });
                }
                _ => {}
            }
        }
    }
}

fn apply_frame(mut qobject: Pin<&mut qobject::PenInput>, frame: Frame) {
    let w = *qobject.as_ref().screenWidth();
    let h = *qobject.as_ref().screenHeight();

    qobject.as_mut().set_penX(frame.norm_x * w);
    qobject.as_mut().set_penY(frame.norm_y * h);
    qobject.as_mut().set_pressure(frame.pressure);
    qobject.as_mut().set_penDown(frame.pen_down);
    qobject.as_mut().pen_changed();
}

/// Scan /dev/input for the marker: a device exposing ABS_X/ABS_Y and a pen or
/// stylus button.
fn find_pen_device() -> Option<evdev::Device> {
    use evdev::{AbsoluteAxisType, Key};

    for (_path, device) in evdev::enumerate() {
        let has_abs = device.supported_absolute_axes().map_or(false, |axes| {
            axes.contains(AbsoluteAxisType::ABS_X) && axes.contains(AbsoluteAxisType::ABS_Y)
        });
        let is_pen = device.supported_keys().map_or(false, |keys| {
            keys.contains(Key::BTN_TOOL_PEN) || keys.contains(Key::BTN_STYLUS)
        });
        if has_abs && is_pen {
            return Some(device);
        }
    }
    None
}
