use std::thread;
use std::time::Duration;

use arboard::Clipboard;
use crossbeam_channel::{unbounded, Receiver};

/// Message from input subsystem containing captured text and optional metadata.
#[derive(Debug, Clone)]
pub struct InputSnapshot {
    pub text: String,
    // future: add window title, pid, selection range, etc.
}

/// Start a clipboard watcher that sends new clipboard text on the returned receiver.
/// This is a simple polling-based prototype (X11-friendly).
pub fn start_clipboard_watcher(poll_interval: Duration) -> Receiver<InputSnapshot> {
    let (s, r) = unbounded();

    thread::spawn(move || {
        let mut clipboard = Clipboard::new().ok();
        let mut last = String::new();
        loop {
            if clipboard.is_none() {
                clipboard = Clipboard::new().ok();
            }
            if let Some(cb) = &mut clipboard {
                match cb.get_text() {
                    Ok(text) => {
                        if !text.is_empty() && text != last {
                            last = text.clone();
                            let snap = InputSnapshot { text };
                            // best-effort send
                            let _ = s.send(snap);
                        }
                    }
                    Err(_) => {
                        // ignore clipboard read errors
                    }
                }
            }
            thread::sleep(poll_interval);
        }
    });

    r
}

/// Start a global-hotkey listener (prototype) and return a receiver that emits an
/// `InputSnapshot` each time the hotkey is pressed. The implementation listens for
/// Ctrl+Shift+Space as the activation hotkey and will attempt to read the clipboard
/// at that moment.
pub fn start_hotkey_listener() -> Receiver<InputSnapshot> {
    use rdev::{Event, EventType, Key, listen};

    let (s, r) = unbounded();

    thread::spawn(move || {
        // We keep track of modifier state
        let mut ctrl = false;
        let mut shift = false;

        let callback = move |event: Event| {
            match event.event_type {
                EventType::KeyPress(key) => {
                    match key {
                        Key::ControlLeft | Key::ControlRight => ctrl = true,
                        Key::ShiftLeft | Key::ShiftRight => shift = true,
                        Key::Space => {
                            if ctrl && shift {
                                // read clipboard once and emit snapshot
                                if let Ok(mut cb) = Clipboard::new() {
                                    if let Ok(text) = cb.get_text() {
                                        let snap = InputSnapshot { text };
                                        let _ = s.send(snap);
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
                EventType::KeyRelease(key) => {
                    match key {
                        Key::ControlLeft | Key::ControlRight => ctrl = false,
                        Key::ShiftLeft | Key::ShiftRight => shift = false,
                        _ => {}
                    }
                }
                _ => {}
            }
        };

        // rdev's listen runs the callback in the current thread and blocks.
        // If it fails, we exit the thread.
        if let Err(err) = listen(callback) {
            eprintln!("hotkey listener error: {:?}", err);
        }
    });

    r
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn clipboard_watcher_smoke() {
        let r = start_clipboard_watcher(Duration::from_millis(200));
        // we cannot assert clipboard contents in CI, but ensure receiver exists
        drop(r);
    }
}
