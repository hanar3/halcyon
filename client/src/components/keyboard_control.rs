use std::f32::consts::E;

use crate::app::application::CURRENT_EVENT;
use sdl2::{event::Event, keyboard::Keycode};

pub struct KeyboardFns {
    pub up_key_fn: dyn FnMut(),
}

pub struct KeyboardControl {
    pub transate_component_id: String,
    up_key: String,
    down_key: String,
    left_key: String,
    right_key: String,
}

impl KeyboardControl {
    pub fn new(translate_component_id: String) -> KeyboardControl {
        KeyboardControl {
            transate_component_id: translate_component_id,
            up_key: "default".to_string(),
            down_key: "default".to_string(),
            left_key: "default".to_string(),
            right_key: "default".to_string(),
        }
    }

    pub fn key_press<F>(&self, mut f: F)
    where
        F: FnMut(),
    {
        f();
    }
}
