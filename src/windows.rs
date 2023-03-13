use std::collections::LinkedList;

use windows_sys::Win32::UI::WindowsAndMessaging::SetWindowPos;

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

    pub fn set_window_pos(&self, x: i32, y: i32, width: i32, height: i32) -> bool {
        unsafe { SetWindowPos(self.hwnd, 0, x, y, width, height, 0x0040) == 1 }
    }
}

#[derive(Default)]
struct Workspace {
    windows: LinkedList<Window>,
}
impl Workspace {
    pub fn add_window(&mut self, window: Window) {
        self.windows.push_back(window);
    }

    pub fn remove_window(&self, hwnd: isize) {}

    pub fn arrange_windows(&self) {}
}
