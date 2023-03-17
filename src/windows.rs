use windows_sys::Win32::UI::WindowsAndMessaging::SetWindowPos;

pub enum TilingMode {
    Managed,
    Monocle,
}

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

    pub fn set_window_pos(&self, x: i32, y: i32, width: i32, height: i32) -> bool {
        unsafe { SetWindowPos(self.hwnd, 0, x, y, width, height, 0x0040) == 1 }
    }
}
