use input::event::pointer::PointerScrollEvent;
use input::event::PointerEvent;
use input::event::pointer;

use input::event::keyboard::{ KeyboardEventTrait, KeyState };
use input::Libinput;

use std::collections::HashSet;

pub fn mouse_wheel_scrolled(input: &mut Libinput) {
    for event in input {
        if let input::Event::Pointer(PointerEvent::ScrollWheel(wheel_event)) = event {
            let vert_scroll = wheel_event.scroll_value(pointer::Axis::Vertical);
            println!("Vertical Scroll: {}", vert_scroll);
            println!();
        }
    }
}

pub fn keys_fully_pressed(key_arr: Vec<u32>, input: &mut Libinput) -> bool {
    let mut pressed_keys = HashSet::<u32>::new();
    let target_keys: HashSet<u32> = key_arr.into_iter().collect();

    for event in input {
        if let input::Event::Keyboard(key_event) = event {
            let key = key_event.key();
            let state = key_event.key_state();
            match state {
                KeyState::Pressed => {
                    pressed_keys.insert(key);
                }
                KeyState::Released => {
                    pressed_keys.remove(&key);
                }
            }
            if target_keys.is_subset(&pressed_keys) {
                return true;
            }
        }
    }

    false
}

