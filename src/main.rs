use dotenv::dotenv;
use input::{create_inputs_window, unregister_hotkeys};
use std::mem::zeroed;

use config::ConfigBuilder;
use windows_sys::Win32::UI::WindowsAndMessaging::{
    DispatchMessageW, GetMessageA, TranslateMessage,
};
use wm::WindowManager;

use crate::input::register_hotkeys;

mod config;
mod input;
mod monitor;
mod tree;
mod windows;
mod wm;
mod workspace;

fn main() {
    dotenv().ok();

    let config_path_str = std::env::var("CONFIG_PATH").expect("Could not load the config file !");
    let config_path = std::path::Path::new(config_path_str.as_str());

    let config_file = config_path.join("config");
    let config_file_str = config_file
        .to_str()
        .expect("Could not build the config path !");

    let config = ConfigBuilder::new(config_file_str).build();

    let global_hwnd = create_inputs_window();

    let mut window_manager = WindowManager::new(config);
    window_manager.get_monitors();

    window_manager.list_managable_windows();
    window_manager.fetch_windows();
    window_manager.arrange_workspaces();

    register_hotkeys(global_hwnd);
    let mut msg = unsafe { zeroed() };
    while unsafe { GetMessageA(&mut msg, global_hwnd, 0, 0) > 0 } {
        unsafe { TranslateMessage(&msg) };
        unsafe { DispatchMessageW(&msg) };

        window_manager.fetch_windows();
        window_manager.arrange_workspaces();
    }
    unregister_hotkeys(global_hwnd);
}
