use raylib::prelude::*;
use std::{collections::HashSet, env, path::PathBuf};

pub fn key_bind_pressed(modifier_keys: &HashSet<KeyboardKey>, main_key: KeyboardKey, d: &RaylibDrawHandle) -> bool {
    modifier_keys.iter().all(|&key| d.is_key_down(key)) && d.is_key_pressed(main_key)
}

pub fn mouse_wheel_scrolled(modifier_keys: &HashSet<KeyboardKey>, d: &RaylibDrawHandle) -> i32 {
    let wheel_movement = d.get_mouse_wheel_move();
    if modifier_keys.iter().all(|&key| d.is_key_down(key)) {
        match wheel_movement {
            w if w > 0.0 => 1,
            w if w < 0.0 => -1,
            _ => 0
        }
    } else {
        0
    }
}

pub fn find_binary(name: &str) -> PathBuf {
    let profile = env::var("PROFILE").unwrap_or_else(|_| "debug".into());

    let mut exe_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    exe_path.push("target");
    exe_path.push(&profile);
    exe_path.push(format!("{}{}", name, env::consts::EXE_SUFFIX));

    exe_path
}

