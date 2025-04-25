use std::collections::HashSet;
use raylib::prelude::*;

mod draw_call;
use draw_call::draw_ring_menu;

mod input_check;
use input_check::{key_bind_pressed, mouse_wheel_scrolled};

const WIN_W: i32 = 1920;
const WIN_H: i32 = 1080;

fn main() {
    let modifiers: HashSet<KeyboardKey> = vec![KeyboardKey::KEY_LEFT_CONTROL, KeyboardKey::KEY_LEFT_SHIFT].into_iter().collect();
    let menu_up_key: KeyboardKey = KeyboardKey::KEY_F10;
    let menu_down_key: KeyboardKey = KeyboardKey::KEY_F9;

    let mut wheel_idx: Option<u32> = None;
    let segments: u32 = 5;

    let (mut rl, thread) = raylib::init()
        .size(WIN_W, WIN_H)
        .title("Hello World!")
        .transparent()
        .undecorated()
        .build();

    rl.set_target_fps(30);

    while !rl.window_should_close() {
        let screen_w = rl.get_screen_width() as f32;
        let screen_h = rl.get_screen_height() as f32;

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::new(0, 0, 0, 0));

        let wheel_movement = mouse_wheel_scrolled(&modifiers, &d);

        if key_bind_pressed(&modifiers, menu_up_key, &d) || wheel_movement == -1 {
            wheel_idx = match wheel_idx {
                Some(val) => Some((val + 1) % segments),
                None => Some(0)
            };
            println!("INFO: Move menu up!");
        }

        if key_bind_pressed(&modifiers, menu_down_key, &d) || wheel_movement == 1 {
            wheel_idx = match wheel_idx {
                Some(val) => Some((val + segments - 1) % segments),
                None => Some(segments - 1),
            };
            println!("INFO: Move menu down!");
        }

        draw_ring_menu(&mut d, screen_h, screen_w, segments, wheel_idx);
    }
}

