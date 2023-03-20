use windows_sys::Win32::UI::Input::KeyboardAndMouse::{
    RegisterHotKey, UnregisterHotKey, MOD_CONTROL,
};

pub fn register_hotkeys(hwnd: isize) {
    let modifier = MOD_CONTROL;
    let registered = unsafe { RegisterHotKey(hwnd, 1, modifier, 49) }; // VK_1
    println!("Hotkey 1  registered: {}", registered);
}

pub fn unregister_hotkeys(hwnd: isize) {
    unsafe { UnregisterHotKey(hwnd, 1) };
}
