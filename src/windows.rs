use windows_sys::Win32::UI::WindowsAndMessaging::{BringWindowToTop, GetWindowTextW, SetWindowPos};

#[derive(PartialEq, Eq, Clone)]
pub enum TilingMode {
    Managed,
    Monocle,
}

#[derive(Clone)]
pub struct Window {
    pub title: String,
    pub hwnd: isize,
    pub mode: TilingMode,
}
impl Window {
    pub fn new(title: &str, hwnd: isize) -> Self {
        Self {
            title: title.to_owned(),
            hwnd,
            mode: TilingMode::Managed,
        }
    }

    pub fn set_mode(&mut self, mode: TilingMode) {
        self.mode = mode;
    }

    pub fn set_window_pos(&self, x: i32, y: i32, width: i32, height: i32) -> bool {
        unsafe { SetWindowPos(self.hwnd, 0, x, y, width, height, 0x0040) == 1 }
    }

    pub fn put_on_top(&self) {
        unsafe { BringWindowToTop(self.hwnd) };
    }
}

impl Window {
    pub fn get_window_title(hwnd: isize) -> String {
        let mut text: [u16; 512] = [0; 512];
        let len = unsafe { GetWindowTextW(hwnd, text.as_mut_ptr(), text.len() as i32) };

        String::from_utf16_lossy(&text[..len as usize])
    }
}
