use std::ffi::c_ulong;
use crate::action::Direction;

pub struct Window {
    pub id: c_ulong,
}

impl Window {
    pub fn new(id: c_ulong) -> Self {
        Self {
            id,
        }
    }
}

pub struct WindowLayout {
    pub windows: Vec<Window>,
    pub focused_index: Option<usize>,
}

impl WindowLayout {
    pub fn new() -> Self {
        Self {
            windows: Vec::new(),
            focused_index: Some(0),
        }
    }

    pub fn insert(&mut self, window: Window) {
        self.focused_index = Some(self.windows.len());
        self.windows.push(window);
    }

    pub fn get_focused_window(&self) -> Option<&Window> {
        if let Some(focused_index) = self.focused_index {
            Some(&self.windows[focused_index])
        } else {
            None
        }
    }

    pub fn index_of(&self, window: &Window) -> usize {
        for i in 0..self.windows.len() {
            if self.windows[i].id == window.id {
                return i;
            }
        }
        return 0;
    }

    pub fn move_focus(&mut self, direction: &Direction) -> Option<usize> {
        if let Some(focused_index) = self.focused_index {
            match direction {
                Direction::Left => {
                    if focused_index > 0 {
                        self.focused_index = Some(focused_index - 1);
                        self.focused_index
                    } else {
                        None
                    }
                }
                Direction::Right => {
                    if focused_index < self.windows.len() - 1 {
                        self.focused_index = Some(focused_index + 1);
                        self.focused_index
                    } else {
                        None
                    }
                }
                Direction::Up | Direction::Down => {
                    None
                }
            }
        } else {
            None
        }
    }

    pub fn remove_focused_window(&mut self) {
        if let Some(focused_index) = self.focused_index {
            self.windows.remove(focused_index);
            if self.windows.len() > 0 {
                if focused_index == 0 {
                    self.focused_index = Some(0)
                } else {
                    self.focused_index = Some(focused_index - 1);
                }
            } else {
                self.focused_index = None;
            }
        }
    }
}
