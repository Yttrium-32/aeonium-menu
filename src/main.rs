use std::collections::{HashMap, HashSet};
use std::result::Result::Ok;
use std::sync::mpsc::{self, RecvTimeoutError};
use std::thread;
use std::time::Duration;

use directories::ProjectDirs;
use gui_state::GuiState;
use tracing::{error, info};

use crate::gui_state::EventType;
use crate::libinput_events::KeyCode;
use crate::shortcut_parser::get_shortcuts;

mod gui_state;
mod libinput_events;
mod shortcut_parser;
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

    thread::spawn(move || -> anyhow::Result<()> {
        libinput_events::run_input_checker(tx, &modifiers, menu_control_keys)?;
        Ok(())
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
            continue;
        }
    }

    Ok(())
}
