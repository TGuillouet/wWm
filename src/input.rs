use std::{ffi::CString, mem::zeroed};

use windows_sys::Win32::{
    Foundation::{LPARAM, LRESULT, WPARAM},
    System::LibraryLoader::GetModuleHandleW,
    UI::{
        Input::KeyboardAndMouse::{RegisterHotKey, UnregisterHotKey, MOD_CONTROL},
        WindowsAndMessaging::{
            CreateWindowExW, DefWindowProcW, DestroyWindow, RegisterClassW, CS_HREDRAW, CS_VREDRAW,
            WM_CLOSE, WM_HOTKEY, WNDCLASSW,
        },
    },
};

pub fn create_inputs_window() -> isize {
    let h_instance = unsafe { GetModuleHandleW(std::ptr::null()) };
    let name = CString::new("wWm").unwrap();

    let mut window_class: WNDCLASSW = unsafe { zeroed() };
    window_class.style = CS_HREDRAW | CS_VREDRAW;
    window_class.lpfnWndProc = Some(window_proc);
    window_class.hInstance = h_instance;
    window_class.lpszClassName = name.as_bytes().as_ptr() as *const u16;

    unsafe { RegisterClassW(&window_class) };

    let hwnd = unsafe {
        CreateWindowExW(
            0,
            name.as_bytes().as_ptr() as *const u16,
            name.as_bytes().as_ptr() as *const u16,
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

    hwnd
}

unsafe extern "system" fn window_proc(
    hwnd: isize,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_CLOSE => {
            DestroyWindow(hwnd);
            return 0;
        }
        WM_HOTKEY => {
            handle_hotkey(wparam);
            return 0;
        }
        _ => return DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}

fn handle_hotkey(key: usize) {
    match key {
        49 => {
            println!("Key 1 pressed")
        }
        _ => {}
    }
}

pub fn register_hotkeys(hwnd: isize) {
    let modifier = MOD_CONTROL;
    let registered = unsafe { RegisterHotKey(hwnd, 1, modifier, 49) }; // VK_1
    println!("Hotkey 1 registered: {}", registered);
}

pub fn unregister_hotkeys(hwnd: isize) {
    unsafe { UnregisterHotKey(hwnd, 1) };
}
