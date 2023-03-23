use std::{ffi::CString, mem::zeroed};

use windows_sys::Win32::{
    Foundation::{LPARAM, LRESULT, WPARAM},
    System::LibraryLoader::GetModuleHandleW,
    UI::{
        Input::KeyboardAndMouse::{RegisterHotKey, UnregisterHotKey, MOD_CONTROL},
        WindowsAndMessaging::{
            CreateWindowExW, DefWindowProcW, DestroyWindow, GetWindowLongPtrW, PostMessageW,
            RegisterClassW, SetWindowLongPtrA, CS_HREDRAW, CS_VREDRAW, GWLP_USERDATA, WM_CLOSE,
            WM_HOTKEY, WNDCLASSW,
        },
    },
};

use crate::GlobalWindowData;

pub fn create_inputs_window(global_data: Box<GlobalWindowData>) -> isize {
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
            std::ptr::null(), // Box::into_raw(global_data) as *mut std::ffi::c_void,
        )
    };

    unsafe {
        SetWindowLongPtrA(hwnd, GWLP_USERDATA, Box::into_raw(global_data) as isize);
    }

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
            unregister_hotkeys(hwnd);
            DestroyWindow(hwnd);
            return 0;
        }
        WM_HOTKEY => {
            handle_hotkey(hwnd, wparam);
            return 0;
        }
        _ => return DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}

fn handle_hotkey(hwnd: isize, key: usize) {
    let window_data_ptr =
        unsafe { GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut GlobalWindowData };
    let window_data = unsafe { &*window_data_ptr };
    match key {
        1 => window_data.sender.send(crate::WmAction::Ping).unwrap(),
        2 => {
            window_data
                .sender
                .send(crate::actions::WmAction::Close { hwnd })
                .unwrap();
        }
        _ => {}
    }
}

pub fn register_hotkeys(hwnd: isize) {
    let modifier = MOD_CONTROL;
    let registered = unsafe { RegisterHotKey(hwnd, 1, modifier, 49) }; // VK_1
    println!("Hotkey 1 registered: {}", registered);

    let registered = unsafe { RegisterHotKey(hwnd, 2, modifier, 50) }; // VK_2
    println!("Hotkey 2 registered: {}", registered);
}

pub fn unregister_hotkeys(hwnd: isize) {
    println!("Unregistering the hotkeys");
    unsafe { UnregisterHotKey(hwnd, 1) };
    unsafe { UnregisterHotKey(hwnd, 2) };
}

pub fn close_inputs_window(hwnd: isize) {
    unsafe { PostMessageW(hwnd, WM_CLOSE, 0, 0) };
}
