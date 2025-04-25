use raylib::prelude::*;
use std::collections::HashSet;

pub fn key_bind_pressed(modifier_keys: &HashSet<KeyboardKey>, main_key: KeyboardKey, d: &RaylibDrawHandle) -> bool {
    modifier_keys.iter().all(|&key| d.is_key_down(key)) && d.is_key_pressed(main_key)
}

pub fn mouse_wheel_scrolled(d: &RaylibDrawHandle) -> i32 {
    let wheel_movement = d.get_mouse_wheel_move();
    match wheel_movement {
        w if w > 0.0 => 1,
        w if w < 0.0 => -1,
        _ => 0
    }
}

