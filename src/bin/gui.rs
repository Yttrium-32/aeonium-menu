use raylib::ffi::{Image, LoadImageFromMemory, LoadTextureFromImage, UnloadImage};
use raylib::prelude::*;

use std::ffi::CString;
use std::io::BufRead;
use std::sync::mpsc;
use std::{env, io, thread};

use aeonium_menu::ring_menu;

const WIN_W: i32 = 1920;
const WIN_H: i32 = 1080;

static DEFAULT_ICON_DATA: &[u8] = include_bytes!("../../resources/default.png");

fn main() {
    let mut args = env::args_os().skip(1);
    let segments: usize = args
        .next()
        .expect("GUI: Expected argument for number of segments")
        .to_string_lossy()
        .parse()
        .expect("GUI: Failed to parse segments argument as usize");

    let mut highlight_idx: Option<usize> = None;

    let (mut rl, thread) = raylib::init()
        .size(WIN_W, WIN_H)
        .title("Aeonium-GUI")
        .transparent()
        .undecorated()
        .build();

    rl.set_target_fps(30);

    let icon_paths: Vec<String> = args.map(|s| s.to_string_lossy().into_owned()).collect();

    let mut icon_textures = Vec::new();

    for path in icon_paths {
        let texture = if path == "default" {
            load_default_icon(DEFAULT_ICON_DATA).expect("GUI: Failed to load built-in default icon")
        } else {
            match rl.load_texture(&thread, &path) {
                Ok(texture) => texture,
                Err(err) => {
                    eprintln!("WARNING: Failed to load icon `{}`: {}", path, err);
                    eprintln!("WARNING: Falling back to default icon");
                    load_default_icon(DEFAULT_ICON_DATA)
                        .expect("GUI: Failed to load built-in default icon")
                }
            }
        };

        icon_textures.push(texture);
    }

    let rx = input_checker_thread();

    while !rl.window_should_close() {
        if let Ok(idx) = rx.try_recv() {
            highlight_idx = idx;
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::new(0, 0, 0, 0));

        ring_menu::draw(
            &mut d,
            WIN_H as f32,
            WIN_W as f32,
            highlight_idx,
            segments,
            &icon_textures,
        );
    }
}

fn input_checker_thread() -> mpsc::Receiver<Option<usize>> {
    let (tx, rx) = mpsc::channel();
    let stdin = io::stdin();

    thread::spawn(move || {
        for line in stdin.lock().lines() {
            match line {
                Ok(text) => {
                    let trimmed = text.trim();
                    if let Some(idx_str) = trimmed.to_uppercase().strip_prefix("HIGHLIGHT ") {
                        let idx_str = idx_str.trim();
                        if let Ok(idx) = idx_str.parse::<usize>() {
                            println!("GUI: INFO: Highlight received {idx}");
                            let _ = tx.send(Some(idx));
                        } else {
                            eprintln!("GUI: WARN: Invalid index in `{trimmed}`");
                        }
                    } else {
                        eprintln!("GUI: WARN: Unexpected input `{trimmed}`");
                    }
                }
                Err(err) => {
                    eprint!("GUI: Error reading stdin: {err}");
                }
            }
        }
    });

    rx
}

pub fn load_default_icon(raw_icon_data: &[u8]) -> Result<Texture2D, String> {
    let extension = CString::new(".png")
        .map_err(|_| "Failed to convert file extension to CString".to_string())?;

    let image: Image = unsafe {
        LoadImageFromMemory(
            extension.as_ptr(),
            raw_icon_data.as_ptr(),
            raw_icon_data.len() as i32,
        )
    };

    if image.data.is_null() {
        return Err("Failed to load default icon from bytes".to_string());
    }

    unsafe {
        let raw_texture = LoadTextureFromImage(image);
        UnloadImage(image);
        Ok(Texture2D::from_raw(raw_texture))
    }
}
