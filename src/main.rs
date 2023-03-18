use wm::WindowManager;

mod monitor;
mod tree;
mod windows;
mod wm;
mod workspace;

fn main() {
    let mut window_manager = WindowManager::new();
    window_manager.get_monitors();
    window_manager.fetch_windows();
    window_manager.arrange_workspaces()
}
