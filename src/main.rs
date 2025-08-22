use std::collections::{HashMap, HashSet};
use std::result::Result::Ok;
use std::sync::mpsc::{self, RecvTimeoutError};
use std::time::Duration;
use std::thread;

use anyhow::Context;
use directories::ProjectDirs;
use gui_state::GuiState;
use input::Libinput;
use tracing::{error, info};

use crate::libinput_events::{InputState, Interface, KeyCode};
use crate::shortcut_parser::get_shortcuts;
use crate::gui_state::EventType;

mod libinput_events;
mod shortcut_parser;
mod gui_state;
mod utils;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let proj_dirs = ProjectDirs::from("", "", "aeonium-menu").expect("No home directory found");
    let config_dir = proj_dirs.config_dir();
    info!("Found config directory: {}", config_dir.display());

    let modifiers: HashSet<KeyCode> = vec![KeyCode::KEY_LEFTCTRL, KeyCode::KEY_LEFTSHIFT]
        .into_iter()
        .collect();

    let menu_control_keys: HashMap<&str, KeyCode> =
        HashMap::from([("up", KeyCode::KEY_F10), ("down", KeyCode::KEY_F9)]);

    let shortcut_files = get_shortcuts(config_dir)?;
    let segments = shortcut_files.len();

    let (tx, rx) = mpsc::channel();

    // Spawn input checker thread
    thread::spawn(move || -> anyhow::Result<()> {
        let mut libinput = Libinput::new_with_udev(Interface);
        libinput.udev_assign_seat("seat0").unwrap();
        let mut state = InputState::new();

        loop {
            state.update(&mut libinput);

            if state.key_bind_pressed(&modifiers, menu_control_keys["up"]) {
                tx.send(EventType::MenuUp)
                    .context("Failed to send MenuUp event")?;
            }

            if state.key_bind_pressed(&modifiers, menu_control_keys["down"]) {
                tx.send(EventType::MenuDown)
                    .context("Failed to send MenuDown event")?;
            }

            let delta = state.scrolled(&modifiers);
            if delta != 0 {
                tx.send(EventType::Scroll(delta))
                    .context(format!("Failed to send Scroll event with delta {}", delta))?;
            }
        }
    });

    let mut gui_state = GuiState::new();

    loop {
        let event = match rx.recv_timeout(Duration::from_millis(100)) {
            Ok(e) => Some(e),
            Err(RecvTimeoutError::Timeout) => None,
            Err(RecvTimeoutError::Disconnected) => {
                error!("Input checker thread broken");
                break;
            }
        };

        if !gui_state.tick(event, segments, &shortcut_files)? {
            break;
        }
    }

    Ok(())
}
