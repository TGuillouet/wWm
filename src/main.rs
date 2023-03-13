use once_cell::sync::OnceCell;
use wm::WindowManager;

mod windows;
mod wm;

pub static mut WINDOW_MANAGER: OnceCell<WindowManager> = OnceCell::new();

fn main() {
    let window_manager = WindowManager::default();
    unsafe {
        WINDOW_MANAGER
            .set(window_manager)
            .expect("Could not set the wm instance");
    }

    let list = WindowManager::global().fetch_opened_windows();
    for (_, window) in list {
        println!("Window {:?}", window);
    }

    WindowManager::global().arrange_windows()
}
