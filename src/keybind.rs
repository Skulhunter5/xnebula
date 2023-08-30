use std::ffi::c_uint;
use crate::action::Action;

pub struct Keybind {
    pub keycode: u32,
    pub modifiers: c_uint,
    pub action: Action,
}

impl Keybind {
    pub fn new(keycode: c_uint, modifiers: c_uint, action: Action) -> Self {
        Self {
            keycode,
            modifiers,
            action
        }
    }
}
