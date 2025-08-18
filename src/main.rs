use input::Libinput;
use directories::ProjectDirs;
use std::collections::{HashMap, HashSet};
use std::io::Write;
use std::process::{Child, Command, Stdio};
use utils::find_binary;

use crate::libinput_events::{InputState, Interface, KeyCode};
use crate::shortcut_parser::get_shortcuts;

mod libinput_events;
mod shortcut_parser;
mod utils;

fn main() {
    let proj_dirs = ProjectDirs::from("", "", "aeonium-menu").expect("No home directory found");
    let config_dir = proj_dirs.config_dir();
    println!("INFO: {config_dir:?}");

    let modifiers: HashSet<KeyCode> = vec![
        KeyCode::KEY_LEFTCTRL,
        KeyCode::KEY_LEFTSHIFT
    ].into_iter().collect();

    let menu_control_keys: HashMap<&str, KeyCode> = HashMap::from([
            ("up", KeyCode::KEY_F10),
            ("down", KeyCode::KEY_F9)
    ]);

    let shortcut_files = get_shortcuts(config_dir);
    let segments = shortcut_files.len();

    let mut input = Libinput::new_with_udev(Interface);
    input.udev_assign_seat("seat0").unwrap();

    let mut state = InputState::new();
    let mut gui_process: Option<Child> = None;
    let mut gui_stdin: Option<std::process::ChildStdin> = None;
    let mut highlight_idx: Option<usize> = None;

    loop {
        state.update(&mut input);

        let scroll_movement = state.scrolled(&modifiers);
        let up = state.key_bind_pressed(&modifiers, menu_control_keys["up"]) || scroll_movement == 1;
        let down = state.key_bind_pressed(&modifiers, menu_control_keys["down"]) || scroll_movement == -1;

        if (up || down) && gui_process.is_none() {
            let gui_exe_path = find_binary("gui");

            let mut cmd = Command::new(gui_exe_path);
            cmd.arg(segments.to_string());

            for desktop_file in &shortcut_files {
                if let Some(icon_path) = &desktop_file.icon {
                    cmd.arg(icon_path);
                } else {
                    cmd.arg("default");
                }
            }

            let mut child = cmd.stdin(Stdio::piped()).spawn().expect("Failed to run GUI");
            gui_stdin = Some(child.stdin.take().unwrap());
            gui_process = Some(child);

            highlight_idx = if up {
                Some(segments - 1)
            } else {
                Some(0)
            };
        }

        if let Some(stdin) = gui_stdin.as_mut() {
            if up {
                highlight_idx = Some(match highlight_idx {
                    Some(val) => (val + 1) % segments,
                    None => 0,
                });
            }
            if down {
                highlight_idx = Some(match highlight_idx {
                    Some(val) => (val + segments - 1) % segments,
                    None => segments - 1,
                });
            }

            if let Some(idx) = highlight_idx {
                writeln!(stdin, "HIGHLIGHT {}", idx).ok();
                stdin.flush().ok();
            }
        }
    }
}
