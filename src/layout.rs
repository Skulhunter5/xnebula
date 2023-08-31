use std::ffi::c_ulong;

pub struct Window {
    pub(crate) id: c_ulong,
}

impl Window {
    pub fn new(id: c_ulong) -> Self {
        Self {
            id,
        }
    }
}

pub struct WindowLayout {
    pub(crate) windows: Vec<Window>,
}

impl WindowLayout {
    pub fn new() -> Self {
        Self {
            windows: Vec::new(),
        }
    }

    pub fn insert(&mut self, window: Window) {
        self.windows.push(window);
    }

    pub fn remove(&mut self, window: &Window) {
        for i in 0..self.windows.len() {
            if self.windows[i].id == window.id {
                self.windows.remove(i);
                break;
            }
        }
    }
}
