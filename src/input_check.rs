use raylib::prelude::*;
use std::collections::HashSet;

pub fn key_bind_pressed(modifier_keys: &HashSet<KeyboardKey>, main_key: KeyboardKey, d: &mut RaylibDrawHandle) -> bool {
    modifier_keys.iter().all(|&key| d.is_key_down(key)) && d.is_key_pressed(main_key)
}

