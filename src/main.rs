use directories::ProjectDirs;
use input::Libinput;
use shortcut_parser::DesktopFile;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::io::Write;
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::mpsc::{self, RecvTimeoutError};
use std::thread;
use std::time::{Duration, Instant};
use utils::find_binary;

use crate::libinput_events::{InputState, Interface, KeyCode};
use crate::shortcut_parser::get_shortcuts;

mod libinput_events;
mod shortcut_parser;
mod utils;

#[derive(Debug)]
enum EventType {
    MenuUp,
    MenuDown,
    Scroll(i32),
}

fn main() {
    let proj_dirs = ProjectDirs::from("", "", "aeonium-menu").expect("No home directory found");
    let config_dir = proj_dirs.config_dir();
    println!("INFO: {config_dir:?}");

    let modifiers: HashSet<KeyCode> = vec![KeyCode::KEY_LEFTCTRL, KeyCode::KEY_LEFTSHIFT]
        .into_iter()
        .collect();

    let menu_control_keys: HashMap<&str, KeyCode> =
        HashMap::from([("up", KeyCode::KEY_F10), ("down", KeyCode::KEY_F9)]);

    let shortcut_files = get_shortcuts(config_dir);
    let segments = shortcut_files.len();

    let (tx, rx) = mpsc::channel();

    // Spawn input checker thread
    thread::spawn(move || {
        let mut libinput = Libinput::new_with_udev(Interface);
        libinput.udev_assign_seat("seat0").unwrap();
        let mut state = InputState::new();

        loop {
            state.update(&mut libinput);

            if state.key_bind_pressed(&modifiers, menu_control_keys["up"]) {
                if let Err(e) = tx.send(EventType::MenuUp) {
                    eprintln!("ERR: Failed to send MenuUp event: {e}");
                    break;
                }
            }
            if state.key_bind_pressed(&modifiers, menu_control_keys["down"]) {
                if let Err(e) = tx.send(EventType::MenuDown) {
                    eprintln!("ERR: Failed to send MenuDown event: {e}");
                    break;
                }
            }

            let delta = state.scrolled(&modifiers);
            if delta != 0 {
                if let Err(e) = tx.send(EventType::Scroll(delta)) {
                    eprintln!("ERR: Failed to send Scroll event: {e}");
                    break;
                }
            }
        }
    });

    let mut gui_process: Option<Child> = None;
    let mut gui_stdin: Option<ChildStdin> = None;
    let mut highlight_idx: Option<usize> = None;
    let mut idle_duration: Option<Instant> = None;

    loop {
        match rx.recv_timeout(Duration::from_millis(100)) {
            Ok(event) => {
                handle_event(
                    event,
                    segments,
                    &shortcut_files,
                    &mut gui_process,
                    &mut gui_stdin,
                    &mut highlight_idx,
                    &mut idle_duration,
                );

                if let (Some(stdin), Some(idx)) = (gui_stdin.as_mut(), highlight_idx) {
                    if let Err(e) = writeln!(stdin, "HIGHLIGHT {}", idx) {
                        eprintln!("ERR: Failed to write GUI stdin: {e}");
                        break;
                    }

                    if let Err(e) = stdin.flush() {
                        eprintln!("ERR: Failed to flush stdin: {e}");
                        break;
                    }
                }
            }

            Err(RecvTimeoutError::Timeout) => {
                if let (Some(start), Some(stdin)) = (idle_duration, gui_stdin.as_mut()) {
                    if start.elapsed() > Duration::from_secs(1) {
                        if let Err(e) = writeln!(stdin, "QUIT") {
                            eprintln!("ERR: Failed to write GUI stdin: {e}");
                        }
                        idle_duration = None;

                        if let Some(mut child) = gui_process.take() {
                            let status = child.wait().expect("ERR: GUI process wasn't runnning");
                            eprintln!("INFO: GUI exited with status: {status}");
                        }
                        gui_stdin = None;
                    }
                }
            }

            Err(RecvTimeoutError::Disconnected) => {
                eprintln!("Err: Input checker thread broken");
                break;
            }
        }
    }
}

#[inline]
fn handle_event(
    event: EventType,
    segments: usize,
    shortcut_files: &[DesktopFile],
    gui_process: &mut Option<Child>,
    gui_stdin: &mut Option<ChildStdin>,
    highlight_idx: &mut Option<usize>,
    idle_duration: &mut Option<Instant>,
) {
    match event {
        EventType::MenuUp | EventType::MenuDown | EventType::Scroll(_) if gui_process.is_none() => {
            let gui_exe_path = find_binary("gui");

            let mut cmd = Command::new(gui_exe_path);
            cmd.arg(segments.to_string());

            for desktop_file in shortcut_files {
                if let Some(icon_path) = &desktop_file.icon {
                    cmd.arg(icon_path);
                } else {
                    cmd.arg("default");
                }
            }

            let mut child = cmd
                .stdin(Stdio::piped())
                .spawn()
                .expect("Failed to run GUI");

            *idle_duration = Some(Instant::now());
            *gui_stdin = Some(child.stdin.take().unwrap());
            *gui_process = Some(child);

            *highlight_idx = Some(match event {
                EventType::MenuUp => segments - 1,
                EventType::MenuDown => 0,
                EventType::Scroll(d) if d < 0 => segments - 1,
                EventType::Scroll(d) if d > 0 => 0,
                _ => unreachable!(),
            })
        }

        EventType::MenuUp => {
            *idle_duration = Some(Instant::now());
            *highlight_idx = Some(match *highlight_idx {
                Some(val) => (val + 1) % segments,
                None => 0,
            });
        }

        EventType::MenuDown => {
            *idle_duration = Some(Instant::now());
            *highlight_idx = Some(match *highlight_idx {
                Some(val) => (val + segments - 1) % segments,
                None => segments - 1,
            });
        }

        EventType::Scroll(scroll_delta) => {
            *idle_duration = Some(Instant::now());
            match scroll_delta.cmp(&0) {
                Ordering::Greater => {
                    *highlight_idx = Some(match *highlight_idx {
                        Some(val) => (val + 1) % segments,
                        None => 0,
                    })
                }
                Ordering::Less => {
                    *highlight_idx = Some(match *highlight_idx {
                        Some(val) => (val + segments - 1) % segments,
                        None => segments - 1,
                    });
                }
                Ordering::Equal => {}
            }
        }
    }
}
