#![feature(proc_macro_hygiene)]

use skyline::nn::hid::{GetNpadStyleSet, NpadGcState};
use strum::IntoEnumIterator;

mod keyboard;
use crate::keyboard::{KeyboardKey, KeyboardState};
use std::collections::HashMap;

mod config;
use crate::config::CONFIG;

#[allow(improper_ctypes)]
extern "C" {
    fn add_nn_hid_hook(callback: fn(*mut NpadGcState, *const u32));
}

pub unsafe fn p1_controller_id() -> u32 {
    let min_controller_id = (0..8)
        .filter(|i| GetNpadStyleSet(i as *const _).flags != 0)
        .min()
        .unwrap_or(0);

    let handheld_id = 0x20;
    if GetNpadStyleSet(&handheld_id as *const _).flags != 0 {
        handheld_id
    } else {
        min_controller_id
    }
}

fn key_matches(
    kbd_state: KeyboardState, 
    keys: &serde_json::Value, 
    keyboard_values: &HashMap<String, u32>) -> bool {
    if !keys.is_array() {
        return false;
    }

    keys.as_array()
        .unwrap()
        .iter()
        .any(|key| {
            let keyboard_value = keyboard_values.get(key.as_str().unwrap());
            return keyboard_value.is_some() && kbd_state.is_key(*keyboard_value.unwrap() as i32);
        })
}

pub fn handle_get_npad_state(state: *mut NpadGcState, controller_id: *const u32) {
    unsafe {
        if *controller_id != p1_controller_id() {
            return;
        }

        let update_count = (*state).updateCount;
        let attributes = (*state).Flags;
        
        let mut kbd_state = KeyboardState::new();
        keyboard::GetKeyboardState(&mut kbd_state);

        let keyboard_keys : std::collections::HashMap<String, u32> = KeyboardKey::iter()
            .map(|key| (key.to_string().to_uppercase(), key as u32))
            .collect();
        let overall_button_config = serde_json::to_value(&CONFIG)
            .unwrap();
        let button_config = overall_button_config
            .as_object()
            .unwrap();
        for (button, keys) in button_config {
            if !key_matches(kbd_state, keys, &keyboard_keys) {
                continue;
            }
                
            let bit = match button.as_str() {
                "A" => Some(0),              //  A
                "B" => Some(1),              //  B
                "X" => Some(2),              //  X
                "Y" => Some(3),              //  Y
                "LSTICK" => Some(4),         //  Left Stick Button
                "RSTICK" => Some(5),         //  Right Stick Button
                "L" => Some(6),              //  L
                "R" => Some(7),              //  R
                "ZL" => Some(8),             //  ZL
                "ZR" => Some(9),             //  ZR
                "PLUS" => Some(10),          //  Plus
                "MINUS" => Some(11),         //  Minus
                "DLEFT" => Some(12),         //  D-Pad Left
                "DUP" => Some(13),           //  D-Pad Up
                "DRIGHT" => Some(14),        //  D-Pad Right
                "DDOWN" => Some(15),         //  D-Pad Down
                "LSTICK_LEFT" => Some(16),   //  Left Stick Left
                "LSTICK_UP" => Some(17),     //  Left Stick Up
                "LSTICK_RIGHT" => Some(18),  //  Left Stick Right
                "LSTICK_DOWN" => Some(19),   //  Left Stick Down
                "RSTICK_LEFT" => Some(20),   //  Right Stick Left
                "RSTICK_UP" => Some(21),     //  Right Stick Up
                "RSTICK_RIGHT" => Some(22),  //  Right Stick Right
                "RSTICK_DOWN" => Some(23),   //  Right Stick Down
                "SL_LEFT" => Some(24),       //  SL on Left Joy-Con
                "SR_LEFT" => Some(25),       //  SR on Left Joy-Con
                "SL_RIGHT" => Some(26),      //  SL on Right Joy-Con
                "SR_RIGHT" => Some(27),      //  SR on Right Joy-Con
                _ => None
            };
            if bit.is_some() {
                (*state).Buttons = (*state).Buttons | 2u64.pow(bit.unwrap());
            }
            let lstick_x = match button.as_str() {
                "LSTICK_RIGHT" => Some(32767),  //  Left Stick Right,
                "LSTICK_LEFT" => Some(-32767),   //  Left Stick Left
                _ => None
            };
            let lstick_y = match button.as_str() {
                "LSTICK_UP" => Some(32767),     //  Left Stick Up
                "LSTICK_DOWN" => Some(-32767),   //  Left Stick Down
                _ => None
            };
            let rstick_x = match button.as_str() {
                "RSTICK_RIGHT" => Some(32767),  //  Right Stick Right
                "RSTICK_LEFT" => Some(-32767),   //  Right Stick Left
                _ => None
            };
            let rstick_y = match button.as_str() {
                "RSTICK_UP" => Some(32767),     //  Right Stick Up
                "RSTICK_DOWN" => Some(-32767),   //  Right Stick Down
                _ => None
            };
            if lstick_x.is_some() { (*state).LStickX = lstick_x.unwrap(); }
            if lstick_y.is_some() { (*state).LStickY = lstick_y.unwrap(); }
            if rstick_x.is_some() { (*state).RStickX = rstick_x.unwrap(); }
            if rstick_y.is_some() { (*state).RStickY = rstick_y.unwrap(); }
        }

        // Handle stick modifiers
        let tilt_1_matches = key_matches(
            kbd_state, button_config.get("TILT1").unwrap(), &keyboard_keys);
        let tilt_2_matches = key_matches(
            kbd_state, button_config.get("TILT2").unwrap(), &keyboard_keys);
        let x1_matches = key_matches(
            kbd_state, button_config.get("X1").unwrap(), &keyboard_keys);
        let x2_matches = key_matches(
            kbd_state, button_config.get("X2").unwrap(), &keyboard_keys);
        let x3_matches = key_matches(
            kbd_state, button_config.get("X3").unwrap(), &keyboard_keys);
        let y1_matches = key_matches(
            kbd_state, button_config.get("Y1").unwrap(), &keyboard_keys);
        let y2_matches = key_matches(
            kbd_state, button_config.get("Y2").unwrap(), &keyboard_keys);
        let y3_matches = key_matches(
            kbd_state, button_config.get("Y3").unwrap(), &keyboard_keys);

        if x1_matches {
            (*state).LStickX = ((*state).LStickX as f64 * CONFIG.X1_MODIFIER) as i32;
        } else if x2_matches {
            (*state).LStickX = ((*state).LStickX as f64 * CONFIG.X2_MODIFIER) as i32;
        } else if x3_matches {
            (*state).LStickX = ((*state).LStickX as f64 * CONFIG.X3_MODIFIER) as i32;
        } else if tilt_1_matches {
            (*state).LStickX = ((*state).LStickX as f64 * CONFIG.TILT1_MODIFIER) as i32;
        } else if tilt_2_matches {
            (*state).LStickX = ((*state).LStickX as f64 * CONFIG.TILT2_MODIFIER) as i32;
        }

        if y1_matches {
            (*state).LStickY = ((*state).LStickY as f64 * CONFIG.Y1_MODIFIER) as i32;
        } else if y2_matches {
            (*state).LStickY = ((*state).LStickY as f64 * CONFIG.Y2_MODIFIER) as i32;
        } else if y3_matches {
            (*state).LStickY = ((*state).LStickY as f64 * CONFIG.Y3_MODIFIER) as i32;
        } else if tilt_1_matches {
            (*state).LStickY = ((*state).LStickY as f64 * CONFIG.TILT1_MODIFIER) as i32;
        } else if tilt_2_matches {
            (*state).LStickY = ((*state).LStickY as f64 * CONFIG.TILT2_MODIFIER) as i32;
        }

        (*state).updateCount = update_count;
        (*state).Flags = attributes;
    }
}

#[skyline::main(name = "switch-keyboard-controller")]
pub fn main() {
    println!("Hello from switch-keyboard-controller");

    std::panic::set_hook(Box::new(|info| {
        let location = info.location().unwrap();

        let msg = match info.payload().downcast_ref::<&'static str>() {
            Some(s) => *s,
            None => match info.payload().downcast_ref::<String>() {
                Some(s) => &s[..],
                None => "Box<Any>",
            },
        };

        let err_msg = format!("thread has panicked at '{}', {}", msg, location);
        skyline::error::show_error(
            69,
            "Skyline plugin has panicked! Please open the details and send a screenshot to the developer, then close the game.\n",
            err_msg.as_str(),
        );
    }));

    config::init();

    unsafe {
        keyboard::InitializeKeyboard();
    }

    unsafe {
        if (add_nn_hid_hook as *const ()).is_null() {
            panic!("The NN-HID hook plugin could not be found and is required to add NRO hooks. Make sure libnn_hid_hook.nro is installed.");
        }
        add_nn_hid_hook(handle_get_npad_state);
    }
}
