use regex::Regex;
use windows_sys::Win32::UI::WindowsAndMessaging::{EnumWindows, GetWindowTextW, IsWindowVisible};

use crate::btree::Node;
use crate::windows::Window;
use crate::WINDOW_MANAGER;

#[derive(Debug)]
pub struct WindowManager {
    windows: Node<Window>,
}
impl WindowManager {
    pub fn new() -> Self {
        Self {
            windows: Node::new(Window {
                title: "()".to_owned(),
                hwnd: 1,
            }),
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

    pub fn fetch_windows(&self) {
        unsafe {
            EnumWindows(Some(WindowManager::get_window_def), 0);
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
            let re = Regex::new(r"Obsidian").unwrap();
            let re2 = Regex::new(r"Discord").unwrap();
            let re3 = Regex::new(r"Opera").unwrap();
            if re.is_match(&window.title)
                || re2.is_match(&window.title)
                || re3.is_match(&window.title)
            {
                wm.windows.insert(window);
            }
        }

        1
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
        println!("====================");
        println!("Current: {}", current_node.value.title);

        println!("Window pos set, {}, {}, {}, {}", x, y, width, height);
        println!("Left {:?}", current_node.left);
        println!("Right {:?}", current_node.right);

        if current_node.left.is_some() && current_node.right.is_some() {
            println!("Splitting");
            let new_width = width / 2;
            let new_height = height;

            current_node
                .left
                .as_ref()
                .unwrap()
                .value
                .set_window_pos(x, y, new_width, new_height);

            self.arrange_recursive(
                &current_node.right.as_ref().unwrap(),
                x + new_width,
                y,
                new_width,
                new_height,
            );
            return;
        }

        current_node.value.set_window_pos(x, y, width, height);
    }
}
