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
    let wm_result = unsafe { WINDOW_MANAGER.set(window_manager) };

    match wm_result {
        Ok(_) => {
            WindowManager::global_mut().get_monitors();
            WindowManager::global_mut().fetch_windows();
            WindowManager::global_mut().arrange_workspaces()
        }
        Err(_) => panic!("An error occured while putting the window manager in the once_cell"),
    }
}
