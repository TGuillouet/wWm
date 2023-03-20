use config::ConfigBuilder;
use wm::WindowManager;

mod config;
mod monitor;
mod tree;
mod windows;
mod wm;
mod workspace;

fn main() {
    let config = ConfigBuilder::new("./config").build();

    let mut window_manager = WindowManager::new(config);
    window_manager.get_monitors();

    window_manager.list_managable_windows();

    loop {
        window_manager.fetch_windows();
        window_manager.arrange_workspaces();
    }
}
