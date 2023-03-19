use regex::Regex;
use windows_sys::Win32::Foundation::LPARAM;
use windows_sys::Win32::Foundation::RECT;
use windows_sys::Win32::Graphics::Gdi::{EnumDisplayMonitors, HDC, HMONITOR};
use windows_sys::Win32::UI::WindowsAndMessaging::{EnumWindows, IsWindowVisible};

use crate::monitor::get_monitor_from_window;
use crate::monitor::get_monitor_resolution;
use crate::windows::TilingMode;
use crate::windows::Window;
use crate::workspace::Workspace;

pub struct WindowManager {
    workspaces: Vec<Workspace>,
}
impl WindowManager {
    pub fn new() -> Self {
        Self {
            workspaces: Vec::new(),
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

    pub fn fetch_windows(&mut self) {
        let mut windows: Vec<isize> = Vec::new();

        unsafe {
            EnumWindows(Some(get_window_def), &mut windows as *mut _ as LPARAM);
        }

        for window_hwnd in windows {
            let title = Window::get_window_title(window_hwnd);

            if title.is_empty() {
                continue;
            }

            let re = Regex::new(r"Obsidian").unwrap();
            let re2 = Regex::new(r"Teamcraft").unwrap();
            let re3 = Regex::new(r"Opera").unwrap();

            if re.is_match(&title) || re2.is_match(&title) || re3.is_match(&title) {
                let monitor: HMONITOR = get_monitor_from_window(window_hwnd);

                for workspace in self.workspaces.iter_mut() {
                    if workspace.monitor_handle == monitor {
                        let mode = if re3.is_match(&title) {
                            TilingMode::Monocle
                        } else {
                            TilingMode::Managed
                        };
                        workspace.add_window(Window::new(&title, window_hwnd).with_mode(mode));
                    }
                }
            }
        }
    }

    pub fn arrange_workspaces(&self) {
        for workspace in self.workspaces.iter() {
            workspace.arrange_windows()
        }
    }
}

unsafe extern "system" fn get_window_def(hwnd: isize, data: LPARAM) -> i32 {
    if IsWindowVisible(hwnd) == 0 {
        return 1;
    }

    let windows = &mut *(data as *mut Vec<isize>);
    windows.push(hwnd);
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
