use std::os::fd::{AsRawFd, BorrowedFd};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::os::unix::{fs::OpenOptionsExt, io::OwnedFd};
use std::path::Path;
use std::sync::mpsc::Sender;
use std::collections::HashSet;

use anyhow::{Context, anyhow};
use input::event::keyboard::{KeyState, KeyboardEventTrait};
use input::event::pointer::{Axis, PointerScrollEvent};
use input::event::{Event, PointerEvent};
use input::{Libinput, LibinputInterface};
use libc::{O_ACCMODE, O_RDONLY, O_RDWR, O_WRONLY};
use nix::poll::{poll, PollFd, PollFlags, PollTimeout};
use num_enum::TryFromPrimitive;

use crate::EventType;

#[allow(non_camel_case_types)]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive)]
pub enum KeyCode {
    KEY_RESERVED = 0,
    KEY_ESC = 1,
    KEY_KEY1 = 2,
    KEY_KEY2 = 3,
    KEY_KEY3 = 4,
    KEY_KEY4 = 5,
    KEY_KEY5 = 6,
    KEY_KEY6 = 7,
    KEY_KEY7 = 8,
    KEY_KEY8 = 9,
    KEY_KEY9 = 10,
    KEY_KEY0 = 11,
    KEY_MINUS = 12,
    KEY_EQUAL = 13,
    KEY_BACKSPACE = 14,
    KEY_TAB = 15,
    KEY_Q = 16,
    KEY_W = 17,
    KEY_E = 18,
    KEY_R = 19,
    KEY_T = 20,
    KEY_Y = 21,
    KEY_U = 22,
    KEY_I = 23,
    KEY_O = 24,
    KEY_P = 25,
    KEY_LEFTBRACE = 26,
    KEY_RIGHTBRACE = 27,
    KEY_ENTER = 28,
    KEY_LEFTCTRL = 29,
    KEY_A = 30,
    KEY_S = 31,
    KEY_D = 32,
    KEY_F = 33,
    KEY_G = 34,
    KEY_H = 35,
    KEY_J = 36,
    KEY_K = 37,
    KEY_L = 38,
    KEY_SEMICOLON = 39,
    KEY_APOSTROPHE = 40,
    KEY_GRAVE = 41,
    KEY_LEFTSHIFT = 42,
    KEY_BACKSLASH = 43,
    KEY_Z = 44,
    KEY_X = 45,
    KEY_C = 46,
    KEY_V = 47,
    KEY_B = 48,
    KEY_N = 49,
    KEY_M = 50,
    KEY_COMMA = 51,
    KEY_DOT = 52,
    KEY_SLASH = 53,
    KEY_RIGHTSHIFT = 54,
    KEY_KPASTERISK = 55,
    KEY_LEFTALT = 56,
    KEY_SPACE = 57,
    KEY_CAPSLOCK = 58,
    KEY_F1 = 59,
    KEY_F2 = 60,
    KEY_F3 = 61,
    KEY_F4 = 62,
    KEY_F5 = 63,
    KEY_F6 = 64,
    KEY_F7 = 65,
    KEY_F8 = 66,
    KEY_F9 = 67,
    KEY_F10 = 68,
    KEY_F13 = 183,
    KEY_F14 = 184,
    KEY_F15 = 185,
    KEY_F16 = 186,
    KEY_F17 = 187,
    KEY_F18 = 188,
    KEY_F19 = 189,
    KEY_F20 = 190,
    KEY_F21 = 191,
    KEY_F22 = 192,
    KEY_F23 = 193,
    KEY_F24 = 194,
    KEY_NUMLOCK = 69,
    KEY_SCROLLLOCK = 70,
    KEY_KP7 = 71,
    KEY_KP8 = 72,
    KEY_KP9 = 73,
    KEY_KPMINUS = 74,
    KEY_KP4 = 75,
    KEY_KP5 = 76,
    KEY_KP6 = 77,
    KEY_KPPLUS = 78,
    KEY_KP1 = 79,
    KEY_KP2 = 80,
    KEY_KP3 = 81,
    KEY_KP0 = 82,
    KEY_KPDOT = 83,
}

pub struct Interface;

impl LibinputInterface for Interface {
    fn open_restricted(&mut self, path: &Path, flags: i32) -> Result<OwnedFd, i32> {
        OpenOptions::new()
            .custom_flags(flags)
            .read((flags & O_ACCMODE) == O_RDONLY || (flags & O_ACCMODE) == O_RDWR)
            .write((flags & O_ACCMODE) == O_WRONLY || (flags & O_ACCMODE) == O_RDWR)
            .open(path)
            .map(|file| file.into())
            .map_err(|err| err.raw_os_error().unwrap())
    }
    fn close_restricted(&mut self, fd: OwnedFd) {
        drop(File::from(fd));
    }
}

#[derive(Debug)]
pub struct InputState {
    pressed_keys: HashSet<KeyCode>,
    just_pressed: HashSet<KeyCode>,
    wheel_delta: i32,
}

impl Default for InputState {
    fn default() -> Self {
        InputState::new()
    }
}

impl InputState {
    pub fn new() -> Self {
        Self {
            pressed_keys: HashSet::new(),
            just_pressed: HashSet::new(),
            wheel_delta: 0,
        }
    }

    pub fn update(&mut self, input: &mut Libinput) {
        input.dispatch().unwrap();
        self.wheel_delta = 0; // reset every cycle
        self.just_pressed.clear();

        for event in input {
            match event {
                Event::Keyboard(k) => {
                    if let Ok(key) = KeyCode::try_from(k.key()) {
                        match k.key_state() {
                            KeyState::Pressed => {
                                if !self.pressed_keys.contains(&key) {
                                    self.just_pressed.insert(key);
                                }
                                self.pressed_keys.insert(key);
                            }
                            KeyState::Released => {
                                self.pressed_keys.remove(&key);
                            }
                        }
                    }
                }

                Event::Pointer(p) => match p {
                    PointerEvent::ScrollWheel(p) => {
                        let val = p.scroll_value(Axis::Vertical);
                        self.wheel_delta = val.signum() as i32;
                    }
                    PointerEvent::ScrollFinger(p) => {
                        if p.has_axis(Axis::Vertical) {
                            let val = p.scroll_value(Axis::Vertical);
                            self.wheel_delta = val.signum() as i32;
                        }
                    }
                    _ => {}
                },

                _ => {}
            }
        }
    }

    pub fn key_bind_pressed(&self, modifiers: &HashSet<KeyCode>, main: KeyCode) -> bool {
        modifiers.iter().all(|m| self.pressed_keys.contains(m)) && self.just_pressed.contains(&main)
    }

    pub fn scrolled(&self, modifiers: &HashSet<KeyCode>) -> i32 {
        if modifiers.iter().all(|k| self.pressed_keys.contains(k)) {
            self.wheel_delta
        } else {
            0
        }
    }
}

pub fn run_input_checker(
    tx: Sender<EventType>,
    modifiers: &HashSet<KeyCode>,
    menu_control_keys: HashMap<&str, KeyCode>,
) -> anyhow::Result<()> {
    let mut libinput = Libinput::new_with_udev(Interface);

    let fd = libinput.as_raw_fd();

    // Safety: `libinput` owns the fd and it remains valid while the variable `libinput` lives
    let borrowed_fd: BorrowedFd<'_> = unsafe { BorrowedFd::borrow_raw(fd) };
    let mut fds = [PollFd::new(borrowed_fd, PollFlags::POLLIN)];

    libinput
        .udev_assign_seat("seat0")
        .map_err(|_| anyhow!("Failed to assign seat0"))?;

    let mut state = InputState::new();

    loop {
        // Block until fd is ready
        poll(&mut fds, PollTimeout::NONE)?;

        state.update(&mut libinput);

        if state.key_bind_pressed(modifiers, menu_control_keys["up"]) {
            tx.send(EventType::MenuUp)
                .context("Failed to send MenuUp event")?;
        }

        if state.key_bind_pressed(modifiers, menu_control_keys["down"]) {
            tx.send(EventType::MenuDown)
                .context("Failed to send MenuDown event")?;
        }

        let delta = state.scrolled(modifiers);
        if delta != 0 {
            tx.send(EventType::Scroll(delta))
                .context(format!("Failed to send Scroll event with delta {}", delta))?;
        }
    }
}
