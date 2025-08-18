use input::Libinput;
use directories::ProjectDirs;
use std::collections::{HashMap, HashSet};
use std::ffi::OsString;
use std::io::Write;
use std::process::{Command, Stdio};
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
    let segments_str: OsString = segments.to_string().into();
    let mut highlight_idx: Option<usize> = None;

    let gui_exe_path = find_binary("gui");
    let mut cmd = Command::new(gui_exe_path);
    cmd.arg(segments_str);

    for desktop_file in shortcut_files {
        if let Some(icon_path) = &desktop_file.icon {
            cmd.arg(icon_path);
        } else {
            cmd.arg("default");
        }
    }

    let mut gui_process = cmd
        .stdin(Stdio::piped())
        .spawn()
        .expect("Failed to run GUI");

    let mut input = Libinput::new_with_udev(Interface);
    input.udev_assign_seat("seat0").unwrap();

    let mut state = InputState::new();
    let gui_stdin = gui_process.stdin.as_mut().expect("Failed to open stdin");

    loop {
        state.update(&mut input);

        let scroll_movement = state.scrolled(&modifiers);

        if state.key_bind_pressed(&modifiers, menu_control_keys["up"]) || scroll_movement == 1 {
            highlight_idx = match highlight_idx {
                Some(val) => Some((val + 1) % segments),
                None => Some(0),
            };
        }

        if state.key_bind_pressed(&modifiers, menu_control_keys["down"]) || scroll_movement == -1 {
            highlight_idx = match highlight_idx {
                Some(val) => Some((val + segments - 1) % segments),
                None => Some(segments - 1),
            };
        }

        if let Some(idx) = highlight_idx {
            writeln!(gui_stdin, "HIGHLIGHT {}", idx).ok();
            gui_stdin.flush().ok();
        }
    }
}
