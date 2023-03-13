use std::collections::HashMap;

use regex::Regex;
use windows_sys::Win32::UI::WindowsAndMessaging::{EnumWindows, GetWindowTextW, IsWindowVisible};

use crate::windows::Window;
use crate::WINDOW_MANAGER;

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

    pub fn arrange_windows(&self) {
        let re = Regex::new(r"Obsidian").unwrap();
        for (key, window) in self.opened_windows.iter() {
            if re.is_match(&window.title) {
                println!("Arranging window {}", window.title);
                let could_arrange_window = window.set_window_pos(10, 10, 1600, 700);
                println!("Could arrange window: {}", could_arrange_window);
            }
        }
    }
}
