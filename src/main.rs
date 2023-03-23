use dotenv::dotenv;
use std::{ffi::CString, mem::zeroed};

use config::ConfigBuilder;
use windows_sys::Win32::{
    System::LibraryLoader::GetModuleHandleW,
    UI::WindowsAndMessaging::{CreateWindowExW, DispatchMessageW, GetMessageW, TranslateMessage},
};
use wm::WindowManager;

use crate::input::{register_hotkeys, unregister_hotkeys};

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

    let hwnd = unsafe {
        let h_instance = GetModuleHandleW(std::ptr::null());

        CreateWindowExW(
            0,
            CString::new("wWm").unwrap().as_bytes().as_ptr() as *const u16,
            CString::new("wWm").unwrap().as_bytes().as_ptr() as *const u16,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            h_instance,
            std::ptr::null(),
        )
    };

    println!("{}", hwnd);

    let mut window_manager = WindowManager::new(config);
    window_manager.get_monitors();

    window_manager.list_managable_windows();

    loop {
        // let mut msg = unsafe { zeroed() };
        // unsafe { GetMessageW(&mut msg, hwnd, 0, 0) };
        // unsafe { TranslateMessage(&msg) };
        // unsafe { DispatchMessageW(&msg) };

        window_manager.fetch_windows();
        window_manager.arrange_workspaces();
    }
}
