#![allow(non_snake_case)]
#[allow(dead_code)]

use skyline::libc::mkdir;
use std::fs;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

use crate::keyboard::KeyboardKey;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Config {
    LSTICK_UP: Vec<String>,
    LSTICK_LEFT: Vec<String>,
    LSTICK_RIGHT: Vec<String>,
    LSTICK_DOWN: Vec<String>,
    LSTICK: Vec<String>,
    RSTICK_UP: Vec<String>,
    RSTICK_LEFT: Vec<String>,
    RSTICK_RIGHT: Vec<String>,
    RSTICK_DOWN: Vec<String>,
    RSTICK: Vec<String>,
    DUP: Vec<String>,
    DLEFT: Vec<String>,
    DRIGHT: Vec<String>,
    DDOWN: Vec<String>,
    A: Vec<String>,
    B: Vec<String>,
    X: Vec<String>,
    Y: Vec<String>,
    L: Vec<String>,
    ZL: Vec<String>,
    R: Vec<String>,
    ZR: Vec<String>,
    PLUS: Vec<String>,
    MINUS: Vec<String>,
    TILT1: Vec<String>,
    TILT2: Vec<String>,
    X1: Vec<String>,
    X2: Vec<String>,
    X3: Vec<String>,
    Y1: Vec<String>,
    Y2: Vec<String>,
    Y3: Vec<String>,
    pub TILT1_MODIFIER: f64,
    pub TILT2_MODIFIER: f64,
    pub X1_MODIFIER: f64,
    pub X2_MODIFIER: f64,
    pub X3_MODIFIER: f64,
    pub Y1_MODIFIER: f64,
    pub Y2_MODIFIER: f64,
    pub Y3_MODIFIER: f64,
}

pub static mut CONFIG: Config = Config {
    LSTICK_UP: vec![],
    LSTICK_LEFT: vec![],
    LSTICK_RIGHT: vec![],
    LSTICK_DOWN: vec![],
    LSTICK: vec![],
    RSTICK_UP: vec![],
    RSTICK_LEFT: vec![],
    RSTICK_RIGHT: vec![],
    RSTICK_DOWN: vec![],
    RSTICK: vec![],
    DUP: vec![],
    DLEFT: vec![],
    DRIGHT: vec![],
    DDOWN: vec![],
    A: vec![],
    B: vec![],
    X: vec![],
    Y: vec![],
    L: vec![],
    ZL: vec![],
    R: vec![],
    ZR: vec![],
    PLUS: vec![],
    MINUS: vec![],
    TILT1: vec![],
    TILT2: vec![],
    X1: vec![],
    X2: vec![],
    X3: vec![],
    Y1: vec![],
    Y2: vec![],
    Y3: vec![],
    TILT1_MODIFIER: 0.5,
    TILT2_MODIFIER: 0.75,
    X1_MODIFIER: 0.33,
    X2_MODIFIER: 0.5,
    X3_MODIFIER: 0.66,
    Y1_MODIFIER: 0.33,
    Y2_MODIFIER: 0.5,
    Y3_MODIFIER: 0.66,
};

macro_rules! c_str {
    ($l:tt) => {
        [$l.as_bytes(), "\u{0}".as_bytes()].concat().as_ptr()
    };
}

pub fn init() {
    unsafe {
        mkdir(c_str!("sd:/switchkeyboard/"), 777);
    }
    let config_path = "sd:/switchkeyboard/config.toml";
    println!("[switch-keyboard-controller] Checking for config file in switchkeyboard/config.toml...");
    if fs::metadata(config_path).is_ok() {
        println!("[switch-keyboard-controller] Keyboard config found. Loading...");
        let combo_conf = fs::read_to_string(&config_path).unwrap();
        if validate_config(&combo_conf) {
            save_config_from_toml(&combo_conf);
        } else {
            save_config_from_defaults();
        }
    } else {
        println!("[switch-keyboard-controller] No previous button combo file found. Creating...");
        fs::write(config_path, DEFAULT_CONFIG).expect("Failed to write button config conf file");
        save_config_from_defaults();
    }
}

pub fn validate_config(data: &str) -> bool {
    let conf: Config = toml::from_str(data).unwrap();

    let all_keyboard_keys : Vec<String> = KeyboardKey::iter()
        .map(|key| key.to_string().to_uppercase())
        .collect();
    let mut bad_keys = vec![];
    for (button, keys) in serde_json::to_value(&conf)
        .unwrap()
        .as_object()
        .unwrap() {
        if !keys.is_array() {
            continue;
        }

        if !keys
            .as_array()
            .unwrap()
            .iter()
            .all(|key| all_keyboard_keys.contains(&key.as_str().unwrap().to_uppercase())) {
            bad_keys.push(format!("{button} = {keys:#?}"));
        }
    }

    if !bad_keys.is_empty() {
        panic!("switch-keyboard-controller\nconfiguration is invalid! Bad keys: {:#?}", bad_keys);
    }

    true
}

pub fn save_config_from_defaults() {
    save_config_from_toml(DEFAULT_CONFIG);
}

pub fn save_config_from_toml(data: &str) {
    let conf: Config = toml::from_str(data).unwrap();
    unsafe {
        CONFIG = conf;
    }
}

pub const DEFAULT_CONFIG: &'static str = r#"LSTICK_UP = ["W"]
LSTICK_LEFT = ["A"]
LSTICK_DOWN = ["S"]
LSTICK_RIGHT = ["D"]
LSTICK = []
RSTICK_UP = ["G"]
RSTICK_LEFT = ["V"]
RSTICK_DOWN = ["B"]
RSTICK_RIGHT = ["N"]
RSTICK = []
DUP = ["UPARROW"]
DLEFT = ["LEFTARROW"]
DDOWN = ["DOWNARROW"]
DRIGHT = ["RIGHTARROW"]
A = ["J"]
B = ["K"]
X = ["L"]
Y = ["I"]
L = ["Y"]
ZL = ["U"]
R = ["O"]
ZR = ["P"]
PLUS = ["RETURN"]
MINUS = ["QUOTE"]
TILT1 = ["LEFTSHIFT"]
TILT2 = ["RIGHTSHIFT"]
X1 = []
X2 = []
X3 = []
Y1 = []
Y2 = []
Y3 = []
TILT1_MODIFIER = 0.5
TILT2_MODIFIER = 0.75
X1_MODIFIER = 0.33
X2_MODIFIER = 0.5
X3_MODIFIER = 0.75
Y1_MODIFIER = 0.33
Y2_MODIFIER = 0.5
Y3_MODIFIER = 0.75

# Available Options for Keyboard Keys:
#
# https://github.com/jugeeya/switch-keyboard-controller/blob/master/src/keyboard.rs#L9-L140
"#;
