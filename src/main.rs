use xnebula::window_manager::WindowManager;

fn main() {
    unsafe {
        let mut window_manager = WindowManager::new();
        window_manager.run();
    }
}
