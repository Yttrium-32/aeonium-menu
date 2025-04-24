use std::collections::HashSet;

use input::event::keyboard::{ KeyboardEventTrait, KeyState };
use input::{ Libinput, LibinputInterface };

pub fn init_libinput<I: 'static + LibinputInterface>(interface: I, seat: &str) -> Libinput {
    let mut input = Libinput::new_from_path(interface);
    input.udev_assign_seat(seat).expect("Failed to assign seat");
    input
}

pub fn keys_fully_pressed(key_arr: Vec<u32>, mut input: Libinput) -> bool {
    let mut pressed_keys = HashSet::<u32>::new();
    let target_keys: HashSet<u32> = key_arr.into_iter().collect();

    for event in &mut input {
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

