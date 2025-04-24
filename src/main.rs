use raylib::prelude::*;

use ::input::LibinputInterface;
use std::fs::{ File, OpenOptions };
use std::os::unix::{ fs::OpenOptionsExt, io::OwnedFd };
use nix::libc::{ O_RDONLY, O_RDWR, O_WRONLY };

use std::path::Path;

mod input_check;

const WIN_W: i32 = 1920;
const WIN_H: i32 = 1080;

const COLOR_TRANSLUCENT_BLUE: Color = Color::new(100, 149, 237, 77);
const COLOR_DARK_BLUE: Color = Color::new(31, 102, 229, 220);

struct Interface;

#[allow(clippy::bad_bit_mask)]
impl LibinputInterface for Interface {
    fn open_restricted(&mut self, path: &Path, flags: i32) -> Result<OwnedFd, i32> {
        OpenOptions::new()
            .custom_flags(flags)
            .read((flags & O_RDONLY != 0) | (flags & O_RDWR != 0))
            .write((flags & O_WRONLY != 0) | (flags & O_RDWR != 0))
            .open(path)
            .map(|file| file.into())
            .map_err(|err| err.raw_os_error().unwrap())
    }
    fn close_restricted(&mut self, fd: OwnedFd) {
        drop(File::from(fd));
    }
}

fn main() {
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

