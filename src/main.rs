use std::collections::HashSet;

use input_check::key_bind_pressed;
use raylib::prelude::*;

mod input_check;

const WIN_W: i32 = 1920;
const WIN_H: i32 = 1080;

const COLOR_TRANSLUCENT_BLUE: Color = Color::new(100, 149, 237, 77);
const COLOR_DARK_BLUE: Color = Color::new(31, 102, 229, 220);

fn main() {
    let modifiers: HashSet<KeyboardKey> = vec![KeyboardKey::KEY_LEFT_SUPER].into_iter().collect();
    let menu_up_key: KeyboardKey = KeyboardKey::KEY_Z;
    let menu_down_key: KeyboardKey = KeyboardKey::KEY_X;

    let (mut rl, thread) = raylib::init()
        .size(WIN_W, WIN_H)
        .title("Hello World!")
        .transparent()
        .undecorated()
        .build();

    while !rl.window_should_close() {
        let screen_w = rl.get_screen_width() as f32;
        let screen_h = rl.get_screen_height() as f32;

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::new(0, 0, 0, 0));

        if key_bind_pressed(&modifiers, menu_up_key, &mut d) {
            println!("INFO: Move menu up!");
        }

        if key_bind_pressed(&modifiers, menu_down_key, &mut d) {
            println!("INFO: Move menu down!");
        }

        draw_ring_menu(&mut d, screen_h, screen_w, 5, Some(4));
    }
}

fn draw_ring_menu(
    d: &mut RaylibDrawHandle,
    screen_h: f32,
    screen_w: f32,
    segments: u32,
    highlight: Option<u32>,
) {

    let center = Vector2::new(screen_w / 2.0, screen_h / 2.0);
    let outer_radius = screen_h.min(screen_w) * 0.25;
    let inner_radius = outer_radius * 0.75;

    let gap_angle = 2.0;
    let total_gap = gap_angle * segments as f32;
    let angle_per_segment = (360.0 - total_gap) / segments as f32;

    let mut start_angle = -90.0;
    for idx in 0..segments {
        let end_angle = start_angle + angle_per_segment;

        let color = match highlight {
            Some(h_idx) => {
                assert!(
                    h_idx <= segments,
                    "hightlight index {} out of bounds for segments {}",
                    h_idx, segments
                );
                if h_idx == idx {
                    COLOR_DARK_BLUE
                } else {
                    COLOR_TRANSLUCENT_BLUE
                }
            },
            None => COLOR_TRANSLUCENT_BLUE
        };

        d.draw_ring(
            center,
            inner_radius,
            outer_radius,
            start_angle,
            end_angle,
            0,
            color
        );

        start_angle = end_angle + gap_angle;
    }
}

