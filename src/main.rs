use std::collections::{HashMap, HashSet};
use std::result::Result::Ok;
use std::sync::mpsc::{self, RecvTimeoutError};
use std::thread;
use std::time::Duration;

use directories::ProjectDirs;
use gui_state::GuiState;
use tracing::error;

use crate::gui_state::EventType;
use crate::libinput_events::KeyCode;
use crate::shortcut_parser::get_shortcuts;
use crate::config::Config;

mod gui_state;
mod libinput_events;
mod shortcut_parser;
mod utils;
mod config;

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let proj_dirs = ProjectDirs::from("", "", "aeonium").unwrap_or_else(|| {
            error!("Fatal error: Home directory not found");
            std::process::exit(1);
    });

    let config_vals = Config::parse(&proj_dirs).unwrap_or_else(|e| {
            error!("Fatal error: {:?}", e);
            std::process::exit(1);
    });

    let modifiers: HashSet<KeyCode> = vec![KeyCode::KEY_LEFTCTRL, KeyCode::KEY_LEFTSHIFT]
        .into_iter()
        .collect();

    let menu_control_keys: HashMap<&str, KeyCode> =
        HashMap::from([("up", KeyCode::KEY_F10), ("down", KeyCode::KEY_F9)]);

    let shortcut_files = get_shortcuts(&proj_dirs).unwrap_or_else(|e| {
            tracing::error!("Fatal Error: {:?}", e);
            std::process::exit(1);
    });
    let segments = shortcut_files.len();

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        if let Err(e) = libinput_events::run_input_checker(tx, &modifiers, menu_control_keys) {
            tracing::error!("Fatal Error: {:?}", e);
            std::process::exit(1);
        };
    });

    let mut gui_state = GuiState::new(&proj_dirs);

    loop {
        let event = match rx.recv_timeout(Duration::from_millis(config_vals.timeout)) {
            Ok(ev) => Some(ev),
            Err(RecvTimeoutError::Timeout) => None,
            Err(RecvTimeoutError::Disconnected) => {
                error!("Fatal Error: Input checker thread broken");
                std::process::exit(1);
            }
        };

        if let Err(e) = gui_state.tick(event, segments, &shortcut_files) {
            error!("Fatal error: {:?}", e);
            std::process::exit(1);
        };
    }
}
