use std::collections::LinkedList;

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
