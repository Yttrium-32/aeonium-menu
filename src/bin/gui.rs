use raylib::prelude::*;

use std::io::BufRead;
use std::sync::mpsc;
use std::{env, io, thread};

use aeonium_menu::ring_menu;

const WIN_W: i32 = 1920;
const WIN_H: i32 = 1080;

fn main() {
    let args: Vec<String> = env::args().collect();
    let segments: usize = args[1].parse().expect("Failed to parse arg");
    let mut highlight_idx: Option<usize> = None;

    let (mut rl, thread) = raylib::init()
        .size(WIN_W, WIN_H)
        .title("Aeonium")
        .transparent()
        .undecorated()
        .build();

    rl.set_target_fps(30);

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
                    if let Some(idx_str) = trimmed.strip_prefix("HIGHLIGHT ") {
                        if let Ok(idx) = idx_str.parse::<usize>() {
                            println!("GUI: INFO: Highlight received {idx}");
                            if tx.send(Some(idx)).is_err() {
                                eprintln!("GUI: WARN: Receiver dropped");
                            }
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
