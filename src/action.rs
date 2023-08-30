use std::process::Command;
use crate::window_manager::WindowManager;

pub enum Action {
    Exit,
    ExecuteCommand {
        command: String,
    },
}

impl Action {
    pub fn execute(&self, window_manager: &WindowManager) {
        match self {
            Action::Exit => unsafe {
                window_manager.exit();
            }
            Action::ExecuteCommand { command } => {
                Command::new(command).spawn().expect(format!("Couldn't execute command: '{}'", command).as_str());
            }
        }
    }
}
