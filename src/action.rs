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
                window_manager.move_focus(direction);
            }
            Action::CloseFocusedWindow => unsafe {
                window_manager.close_focused_window();
            }
        }
    }
}
