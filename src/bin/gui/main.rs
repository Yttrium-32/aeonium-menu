use anyhow::{Context, bail};
use raylib::ffi::{Image, LoadImageFromMemory, LoadTextureFromImage, UnloadImage};
use raylib::prelude::*;
use tracing::{debug, error, info, trace, warn};

use std::ffi::CString;
use std::io::BufRead;
use std::sync::mpsc;
use std::{env, io, thread};

mod ring_menu;

const WIN_W: i32 = 1920;
const WIN_H: i32 = 1080;

static DEFAULT_ICON_DATA: &[u8] = include_bytes!("../../../resources/default.png");

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let mut args = env::args_os().skip(1);
    let segments: usize = args
        .next()
        .context("GUI: Expected argument for number of segments")?
        .to_string_lossy()
        .parse()
        .context("GUI: Failed to parse segments argument as usize")?;

    let mut highlight_idx: Option<usize> = None;

    set_trace_log_callback(|lvl, msg| match lvl {
        TraceLogLevel::LOG_ALL | TraceLogLevel::LOG_TRACE => trace!("{msg}"),
        TraceLogLevel::LOG_DEBUG => debug!("{msg}"),
        TraceLogLevel::LOG_INFO => info!("{msg}"),
        TraceLogLevel::LOG_WARNING => warn!("{msg}"),
        TraceLogLevel::LOG_ERROR => error!("{msg}"),
        TraceLogLevel::LOG_FATAL => panic!("Raylib fatal error: {msg}"),
        _ => info!("{msg}"),
    })?;

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
            load_default_icon(DEFAULT_ICON_DATA)?
        } else {
            match rl.load_texture(&thread, &path) {
                Ok(texture) => texture,
                Err(err) => {
                    warn!("GUI: Failed to load icon `{path}`: {err}");
                    warn!("GUI: Falling back to default icon");
                    load_default_icon(DEFAULT_ICON_DATA)
                        .context("GUI: Failed to load default icon")?
                }
            }
        };

        icon_textures.push(texture);
    }

    let rx = input_checker_thread();

    'render_loop: while !rl.window_should_close() {
        while let Ok(msg) = rx.try_recv() {
            match msg {
                Some(idx) => {
                    highlight_idx = Some(idx);
                }
                None => {
                    break 'render_loop;
                }
            }
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

    Ok(())
}

fn input_checker_thread() -> mpsc::Receiver<Option<usize>> {
    let (tx, rx) = mpsc::channel();
    let stdin = io::stdin();

    thread::spawn(move || {
        for line in stdin.lock().lines() {
            let text = match line {
                Ok(t) => t,
                Err(err) => {
                    warn!("GUI: Error reading stdin: {err}");
                    continue;
                }
            };

            let trimmed = text.trim();

            if let Some(idx_str) = trimmed.to_uppercase().strip_prefix("HIGHLIGHT ") {
                match idx_str.trim().parse::<usize>() {
                    Ok(idx) => {
                        let _ = tx.send(Some(idx));
                    }
                    Err(_) => warn!("GUI: Invalid index in `{trimmed}`"),
                }
            } else if trimmed.eq_ignore_ascii_case("QUIT") {
                let _ = tx.send(None);
            } else {
                warn!("GUI: Unexpected input `{trimmed}`");
            }
        }
    });

    rx
}
pub fn load_default_icon(raw_icon_data: &[u8]) -> anyhow::Result<Texture2D> {
    let extension =
        CString::new(".png").context("GUI: Failed to convert file extension to CString")?;

    let image: Image = unsafe {
        LoadImageFromMemory(
            extension.as_ptr(),
            raw_icon_data.as_ptr(),
            raw_icon_data.len() as i32,
        )
    };

    if image.data.is_null() {
        bail!("GUI: Failed to load default icon from embedded PNG bytes");
    }

    let texture = unsafe {
        let raw_texture = LoadTextureFromImage(image);
        UnloadImage(image);
        Texture2D::from_raw(raw_texture)
    };

    Ok(texture)
}
