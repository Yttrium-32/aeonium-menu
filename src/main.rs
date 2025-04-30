use directories::ProjectDirs;
use std::collections::{HashMap, HashSet};
use raylib::prelude::*;

mod draw_call;
use draw_call::draw_ring_menu;

mod input_check;
use input_check::{key_bind_pressed, mouse_wheel_scrolled};

mod desktop_file;
use desktop_file::{get_shortcut_files, DesktopFile};

const WIN_W: i32 = 1920;
const WIN_H: i32 = 1080;

fn main() {
    let proj_dirs = ProjectDirs::from("", "", "aeonium-menu").expect("No home directory found");
    let config_dir = proj_dirs.config_dir();
    println!("INFO: {config_dir:?}");

    let modifiers: HashSet<KeyboardKey> = vec![KeyboardKey::KEY_LEFT_CONTROL, KeyboardKey::KEY_LEFT_SHIFT].into_iter().collect();
    let menu_control_keys: HashMap<&str, KeyboardKey> = HashMap::from([
        ("up", KeyboardKey::KEY_F10),
        ("down", KeyboardKey::KEY_F9)
    ]);

    let (mut rl, thread) = raylib::init()
        .size(WIN_W, WIN_H)
        .title("Aeonium")
        .transparent()
        .undecorated()
        .build();

    rl.set_target_fps(30);

    let shortcut_files = get_shortcut_files(config_dir, &mut rl, &thread);

    let segments = shortcut_files.len();
    let mut seg_highlight_idx: Option<usize> = None;

    while !rl.window_should_close() {
        render_loop(
            &mut rl,
            &thread,
            &modifiers,
            &menu_control_keys,
            &shortcut_files,
            segments,
            &mut seg_highlight_idx
        );
    }
}

#[inline]
fn render_loop(
    rl: &mut RaylibHandle,
    thread: &RaylibThread,
    modifiers: &HashSet<KeyboardKey>,
    menu_control_keys: &HashMap<&str, KeyboardKey>,
    shortcut_files: &[DesktopFile],
    segments: usize,
    seg_highlight_idx: &mut Option<usize>
) {
        let mut d = rl.begin_drawing(thread);
        d.clear_background(Color::new(0, 0, 0, 0));

        let wheel_movement = mouse_wheel_scrolled(modifiers, &d);

        if key_bind_pressed(modifiers, menu_control_keys["up"], &d) || wheel_movement == -1 {
            *seg_highlight_idx = match *seg_highlight_idx {
                Some(val) => Some((val + 1) % segments),
                None => Some(0),
            };
        }

        if key_bind_pressed(modifiers, menu_control_keys["down"], &d) || wheel_movement == 1 {
            *seg_highlight_idx = match *seg_highlight_idx {
                Some(val) => Some((val + segments - 1) % segments),
                None => Some(segments - 1),
            };
        }

        draw_ring_menu(&mut d, WIN_H as f32, WIN_W as f32, *seg_highlight_idx, shortcut_files);
}
