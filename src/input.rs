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

use crate::{
    actions::{WmAction, WorkspaceAction},
    windows::TilingMode,
    GlobalWindowData,
};

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
        1 => dispatch(
            window_data,
            WmAction::Workspace(WorkspaceAction::PreviousAsCurrent),
        ),
        2 => dispatch(
            window_data,
            WmAction::Workspace(WorkspaceAction::NextAsCurrent),
        ),
        3 => dispatch(
            window_data,
            WmAction::Workspace(WorkspaceAction::ToggleMode(TilingMode::Monocle)),
        ),
        4 => dispatch(
            window_data,
            WmAction::Workspace(WorkspaceAction::ToggleMode(TilingMode::Managed)),
        ),
        9 => {
            window_data
                .sender
                .send(crate::actions::WmAction::Close { hwnd })
                .unwrap();
        }
        _ => {}
    }
}

fn dispatch(window_data: &GlobalWindowData, action: WmAction) {
    window_data
        .sender
        .send(action)
        .expect("Could not dispatch the Window manager action ")
}

pub fn register_hotkeys(hwnd: isize) {
    let modifier = MOD_CONTROL;

    for hotkey_index in 0..9 {
        let registered =
            unsafe { RegisterHotKey(hwnd, hotkey_index + 1, modifier, 49 + hotkey_index as u32) }; // VK_1
        println!("Hotkey {} registered: {}", hotkey_index + 1, registered);
    }
}

pub fn unregister_hotkeys(hwnd: isize) {
    println!("Unregistering the hotkeys");
    for hotkey_index in 0..9 {
        unsafe { UnregisterHotKey(hwnd, hotkey_index) };
    }
}

pub fn close_inputs_window(hwnd: isize) {
    unsafe { PostMessageW(hwnd, WM_CLOSE, 0, 0) };
}
