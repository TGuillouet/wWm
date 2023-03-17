use regex::Regex;
use windows_sys::Win32::Foundation::LPARAM;
use windows_sys::Win32::Foundation::RECT;
use windows_sys::Win32::Graphics::Gdi::{EnumDisplayMonitors, HDC, HMONITOR};
use windows_sys::Win32::UI::WindowsAndMessaging::{EnumWindows, GetWindowTextW, IsWindowVisible};

use crate::monitor::get_monitor_resolution;
use crate::tree::{Node, TilingDirection};
use crate::windows::Window;
use crate::WINDOW_MANAGER;

#[derive(Debug)]
pub struct WindowManager {
    windows: Node<Window>,
}
impl WindowManager {
    pub fn new() -> Self {
        Self {
            windows: Node::new(
                Window {
                    title: "()".to_owned(),
                    hwnd: 1,
                },
                TilingDirection::Vertical,
            ),
        }
    }

    pub fn global() -> &'static WindowManager {
        unsafe {
            WINDOW_MANAGER
                .get()
                .expect("Could not get the global instance")
        }
    }
    pub fn global_mut() -> &'static mut WindowManager {
        unsafe {
            WINDOW_MANAGER
                .get_mut()
                .expect("Could not get the global instance")
        }
    }

    pub fn get_monitors(&self) {
        let mut monitors: Vec<HMONITOR> = Vec::new();

        unsafe {
            EnumDisplayMonitors(
                0,
                std::ptr::null_mut(),
                Some(enum_monitors_callback),
                &mut monitors as *mut Vec<HMONITOR> as LPARAM,
            );
        }

        for monitor in monitors {
            let monitor_resolution = get_monitor_resolution(monitor);

            println!(
                "{}, {}",
                monitor_resolution.width, monitor_resolution.height
            )
        }
    }

    pub fn fetch_windows(&self) {
        unsafe {
            EnumWindows(Some(get_window_def), 0);
        }
    }

    pub fn arrange_windows(&self, x: i32, y: i32, width: i32, height: i32) {
        self.arrange_recursive(&self.windows, x, y, width, height);
    }

    fn arrange_recursive(
        &self,
        current_node: &Node<Window>,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) {
        let mut child_x = x;
        let mut child_y = y;

        let width_ratio = width / current_node.childrens.len() as i32;
        let height_ratio = height / current_node.childrens.len() as i32;

        for children in current_node.childrens.iter() {
            let child_width = if children.direction == TilingDirection::Horizontal {
                width_ratio
            } else {
                width
            };
            let child_height = if children.direction == TilingDirection::Vertical {
                height_ratio
            } else {
                height
            };
            if children.is_leaf() {
                let new_width = child_width;
                let new_height = child_height;
                let new_x = child_x;
                let new_y = child_y;

                children
                    .value
                    .set_window_pos(new_x, new_y, new_width, new_height);
            } else {
                self.arrange_recursive(&children, child_x, child_y, child_width, child_height)
            }

            match children.direction {
                TilingDirection::Vertical => child_y += child_height,
                TilingDirection::Horizontal => child_x += child_width,
            }
        }
    }
}

unsafe extern "system" fn get_window_def(hwnd: isize, _l_param: LPARAM) -> i32 {
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
        let re = Regex::new(r"Obsidian").unwrap();
        let re2 = Regex::new(r"Teamcraft").unwrap();
        let re3 = Regex::new(r"Opera").unwrap();
        if re.is_match(&window.title) || re2.is_match(&window.title) || re3.is_match(&window.title)
        {
            wm.windows.insert(window, TilingDirection::Horizontal);
        }
    }

    1
}

unsafe extern "system" fn enum_monitors_callback(
    monitor: HMONITOR,
    _: HDC,
    _: *mut RECT,
    data: LPARAM,
) -> i32 {
    let monitors = &mut *(data as *mut Vec<HMONITOR>);
    monitors.push(monitor);
    1
}
