use once_cell::sync::OnceCell;
use wm::WindowManager;

mod monitor;
mod tree;
mod windows;
mod wm;
mod workspace;

pub static mut WINDOW_MANAGER: OnceCell<WindowManager> = OnceCell::new();

fn main() {
    let window_manager = WindowManager::new();
    unsafe {
        WINDOW_MANAGER
            .set(window_manager)
            .expect("Could not set the wm instance");
    }

    WindowManager::global_mut().get_monitors();
    WindowManager::global().fetch_windows();
    WindowManager::global_mut().arrange_workspaces()
}
