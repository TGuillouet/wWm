use std::{ffi::CString, mem::zeroed};

use windows_sys::Win32::{
    Foundation::{LPARAM, LRESULT, WPARAM},
    System::LibraryLoader::GetModuleHandleW,
    UI::{
        Input::KeyboardAndMouse::{RegisterHotKey, UnregisterHotKey, MOD_CONTROL, MOD_SHIFT, VK_1},
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

fn handle_hotkey(hwnd: isize, key: u16) {
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
        10 => dispatch(
            window_data,
            WmAction::Workspace(WorkspaceAction::PutCurrentWindowInWorkspace {
                workspace_index: 0,
            }),
        ),
        11 => dispatch(
            window_data,
            WmAction::Workspace(WorkspaceAction::PutCurrentWindowInWorkspace {
                workspace_index: 1,
            }),
        ),
        _ => {}
    }
}

fn dispatch(window_data: &GlobalWindowData, action: WmAction) {
    window_data
        .sender
        .send(action)
        .expect("Could not dispatch the Window manager action ")
}

pub fn close_inputs_window(hwnd: isize) {
    unsafe { PostMessageW(hwnd, WM_CLOSE, 0, 0) };
}

unsafe extern "system" fn window_proc(
    hwnd: isize,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    println!(
        "HWND: {}, Close: {}, Hotkey: {}, msg: {}",
        hwnd,
        msg == WM_CLOSE,
        msg == WM_HOTKEY,
        msg
    );
    match msg {
        WM_CLOSE => {
            unregister_hotkeys();
            DestroyWindow(hwnd);
            return 0;
        }
        WM_HOTKEY => {
            handle_hotkey(hwnd, wparam as u16);
            return 0;
        }
        _ => return DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}

pub fn register_hotkeys() {
    let modifier = MOD_CONTROL;
    for hotkey_index in 0..9 {
        let registered = unsafe {
            RegisterHotKey(
                0,
                hotkey_index + 1,
                modifier,
                VK_1 as u32 + hotkey_index as u32,
            )
        }; // VK_1
        println!("Hotkey {} registered: {}", hotkey_index + 1, registered);
    }

    let modifier = MOD_SHIFT | MOD_CONTROL;
    for hotkey_index in 1..9 {
        let registered = unsafe {
            RegisterHotKey(
                0,
                hotkey_index + 9,
                modifier,
                VK_1 as u32 + (hotkey_index - 1) as u32,
            )
        }; // VK_1
        println!("Hotkey {} registered: {}", hotkey_index + 9, registered);
    }
}

pub fn unregister_hotkeys() {
    println!("Unregistering the hotkeys");
    for hotkey_index in 0..18 {
        unsafe { UnregisterHotKey(0, hotkey_index + 1) };
    }
}
