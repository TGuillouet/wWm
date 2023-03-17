use regex::Regex;
use windows_sys::Win32::Foundation::LPARAM;
use windows_sys::Win32::Foundation::RECT;
use windows_sys::Win32::Graphics::Gdi::MonitorFromWindow;
use windows_sys::Win32::Graphics::Gdi::MONITOR_DEFAULTTONEAREST;
use windows_sys::Win32::Graphics::Gdi::{EnumDisplayMonitors, HDC, HMONITOR};
use windows_sys::Win32::UI::WindowsAndMessaging::{EnumWindows, GetWindowTextW, IsWindowVisible};

use crate::monitor::get_monitor_resolution;
use crate::windows::Window;
use crate::workspace::Workspace;
use crate::WINDOW_MANAGER;

pub struct WindowManager {
    workspaces: Vec<Workspace>,
}
impl WindowManager {
    pub fn new() -> Self {
        Self {
            workspaces: Vec::new(),
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

    pub fn get_monitors(&mut self) {
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

            self.workspaces
                .push(Workspace::new(monitor, monitor_resolution));
        }
    }

    pub fn fetch_windows(&self) {
        unsafe {
            EnumWindows(Some(get_window_def), 0);
        }
    }

    pub fn arrange_workspaces(&self) {
        for workspace in self.workspaces.iter() {
            workspace.arrange_windows()
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
        let wm = WINDOW_MANAGER
            .get_mut()
            .expect("window manager not initialized");
        let re = Regex::new(r"Obsidian").unwrap();
        let re2 = Regex::new(r"Teamcraft").unwrap();
        let re3 = Regex::new(r"Opera").unwrap();
        if re.is_match(&window_title) || re2.is_match(&window_title) || re3.is_match(&window_title)
        {
            let monitor = unsafe { MonitorFromWindow(hwnd, MONITOR_DEFAULTTONEAREST) };

            for workspace in wm.workspaces.iter_mut() {
                if workspace.monitor_handle == monitor {
                    workspace.add_window(Window::new(&window_title, hwnd));
                }
            }
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
