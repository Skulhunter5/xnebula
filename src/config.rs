use std::ffi::{c_int, c_ulong};
use crate::util::Bounds;

#[derive(Debug)]
pub struct Monitor {
    pub bounds: Bounds,
}

impl Monitor {
    pub fn new(width: c_int, height: c_int, x: c_int, y: c_int) -> Self {
        Self {
            bounds: Bounds::new(x, y, width, height),
        }
    }
}

#[derive(Debug)]
pub struct Border {
    pub width: c_int,
    pub color: c_ulong,
}

impl Default for Border {
    fn default() -> Self {
        Self {
            width: 3,
            color: 0x00ffffff,
        }
    }
}

#[derive(Debug)]
pub struct Config {
    pub monitors: Vec<Monitor>,
    pub border: Option<Border>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            monitors: Vec::new(),
            border: Some(Border::default()),
        }
    }
}
