use std::collections::{HashMap, LinkedList};

use regex::Regex;
use windows_sys::Win32::UI::WindowsAndMessaging::{EnumWindows, GetWindowTextW, IsWindowVisible};

use crate::windows::Window;
use crate::WINDOW_MANAGER;

#[derive(Debug)]
pub struct WindowManager {
    windows: LinkedList<Window>,
}
impl WindowManager {
    pub fn new() -> Self {
        Self {
            windows: Default::default(),
        }
    }

    pub fn global() -> &'static WindowManager {
        unsafe {
            WINDOW_MANAGER
                .get()
                .expect("Could not get the global instance")
        }
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
            wm.windows.push_back(window);
        }

        1
    }

    pub fn arrange_windows(&self) {
        let re = Regex::new(r"Obsidian").unwrap();
        for window in self.windows.iter() {
            if re.is_match(&window.title) {
                window.set_window_pos(10, 10, 1600, 900);
            }
        }
    }
}
