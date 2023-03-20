use std::{ffi::CString, mem::zeroed};

use config::ConfigBuilder;
use windows_sys::Win32::{
    System::LibraryLoader::GetModuleHandleW,
    UI::WindowsAndMessaging::{CreateWindowExW, GetMessageW},
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
    let config = ConfigBuilder::new("./config").build();

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

    register_hotkeys(hwnd);
    loop {
        let mut msg = unsafe { zeroed() };
        unsafe { GetMessageW(&mut msg, hwnd, 0, 0) };

        window_manager.fetch_windows();
        window_manager.arrange_workspaces();
    }
}
