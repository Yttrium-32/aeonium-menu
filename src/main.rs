use raylib::prelude::*;

use ::input::LibinputInterface;
use std::fs::{ File, OpenOptions };
use std::os::unix::{ fs::OpenOptionsExt, io::OwnedFd };
use nix::libc::{ O_RDONLY, O_RDWR, O_WRONLY };

use std::path::Path;

mod input_check;

const WIN_W: i32 = 1920;
const WIN_H: i32 = 1080;

const COLOR_BLUE: Color = Color::new(100, 149, 237, 77);
const COLOR_MAGENTA: Color = Color::new(255, 0, 128, 77);

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

        draw_ring_menu(&mut d, screen_h, screen_w);
    }
}

fn draw_ring_menu(
    d: &mut RaylibDrawHandle,
    screen_h: f32,
    screen_w: f32,
    //segments: u32
) {
    let center = Vector2::new(screen_w / 2.0, screen_h / 2.0);
    let outer_radius = screen_h.min(screen_w) * 0.25;
    let inner_radius = outer_radius * 0.75;

    d.draw_ring(
        center,
        inner_radius,
        outer_radius,
        0.0,
        360.0,
        0,
        COLOR_BLUE
    );
}

