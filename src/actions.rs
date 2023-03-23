#[derive(Debug)]
pub enum WmAction {
    Ping,
    Close { hwnd: isize },
}
