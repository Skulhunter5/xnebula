use std::process::Command;
use crate::util::Direction;
use crate::window_manager::WindowManager;

#[derive(Clone)]
pub enum Action {
    Exit,
    ExecuteCommand {
        command: String,
    },
    MoveFocus {
        direction: Direction,
    },
    CloseFocusedWindow,
    ChangeTilingDirection {
        direction: Direction,
    },
    ResizeFocusedWindow {
        direction: Direction,
        amount: f32,
    },
}

impl Action {
    pub fn execute(&self, window_manager: &mut WindowManager) {
        match self {
            Action::Exit => unsafe {
                window_manager.exit();
            }
            Action::ExecuteCommand { command } => {
                Command::new(command).spawn().expect(format!("Couldn't execute command: '{}'", command).as_str());
            }
            Action::MoveFocus { direction } => unsafe {
                window_manager.move_focus(direction.clone());
            }
            Action::CloseFocusedWindow => unsafe {
                window_manager.close_focused_window();
            }
            Action::ChangeTilingDirection { direction } => {
                window_manager.change_tiling_direction(direction.clone());
            }
            Action::ResizeFocusedWindow { direction, amount } => unsafe {
                window_manager.resize_focused_window(direction.clone(), amount.clone());
            }
        }
    }
}
