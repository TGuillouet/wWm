use std::collections::HashMap;

use windows_sys::Win32::UI::WindowsAndMessaging::{EnumWindows, GetWindowTextW, IsWindowVisible};

use crate::WINDOW_MANAGER;

#[derive(Debug)]
pub struct Window {
    pub title: String,
    pub hwnd: isize,
}
impl Window {
    pub fn new(title: &str, hwnd: isize) -> Self {
        Self {
            title: title.to_owned(),
            hwnd,
        }
    }
}

#[derive(Debug, Default)]
pub struct WindowManager {
    pub opened_windows: HashMap<String, Window>,
}
impl WindowManager {
    pub fn global() -> &'static WindowManager {
        unsafe {
            WINDOW_MANAGER
                .get()
                .expect("Could not get the global instance")
        }
    }

    pub fn fetch_opened_windows(&self) -> &HashMap<String, Window> {
        unsafe { EnumWindows(Some(WindowManager::get_window_def), 0) == 1 };
        return &self.opened_windows;
    }

    unsafe extern "system" fn get_window_def(hwnd: isize, l_param: isize) -> i32 {
        if IsWindowVisible(hwnd) == 0 {
            return 1;
        }

        let mut text: [u16; 512] = [0; 512];
        let len = GetWindowTextW(hwnd, text.as_mut_ptr(), text.len() as i32);
        let window_title = String::from_utf16_lossy(&text[..len as usize]);

        if !window_title.is_empty() {
            let window = Window::new(&window_title, hwnd);

            let wm = WINDOW_MANAGER
                .get_mut()
                .expect("window manager not initialized");
            wm.opened_windows.insert(window_title, window);
        }

        1
    }
}
