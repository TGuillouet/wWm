use windows_sys::Win32::Foundation::LPARAM;
use windows_sys::Win32::Foundation::RECT;
use windows_sys::Win32::Graphics::Gdi::{EnumDisplayMonitors, HDC, HMONITOR};
use windows_sys::Win32::UI::WindowsAndMessaging::{EnumWindows, IsWindowVisible};

use crate::actions::WorkspaceAction;
use crate::config::Config;
use crate::monitor::get_monitor_from_window;
use crate::monitor::get_monitor_resolution;
use crate::windows::Window;
use crate::workspace::Workspace;

pub struct WindowManager {
    config: Config,
    windows: Vec<isize>,
    workspaces: Vec<Workspace>,

    current_workspace_index: usize,
}
impl WindowManager {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            windows: Vec::new(),
            workspaces: Vec::new(),

            current_workspace_index: 0,
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

    fn get_windows(&self) -> Vec<isize> {
        let mut windows: Vec<isize> = Vec::new();

        unsafe {
            EnumWindows(Some(get_window_def), &mut windows as *mut _ as LPARAM);
        }

        windows
    }

    fn get_managed_windows(&self, windows: &Vec<isize>) -> Vec<isize> {
        let mut managed_windows = Vec::new();
        for window_hwnd in windows.clone() {
            let title = Window::get_window_title(window_hwnd);

            if title.is_empty() {
                continue;
            }

            if self.config.is_managed(&title) {
                managed_windows.push(window_hwnd);
            }
        }

        managed_windows
    }

    pub fn fetch_windows(&mut self) {
        let windows = self.get_windows();

        let managed_windows = self.get_managed_windows(&windows);

        // Check if a window has been closed
        if managed_windows.len() < self.windows.len() {
            let current_windows = self.windows.clone();
            let windows_to_delete = current_windows
                .iter()
                .filter(|item| !managed_windows.contains(item))
                .into_iter();

            for window_to_delete in windows_to_delete {
                for workspace in self.workspaces.iter_mut() {
                    Workspace::remove_window(&mut workspace.windows, *window_to_delete);
                }
            }
            self.windows = managed_windows;
            return;
        }

        for window_hwnd in windows {
            let title = Window::get_window_title(window_hwnd);

            if title.is_empty() {
                continue;
            }

            if self.config.is_managed(&title) {
                let monitor: HMONITOR = get_monitor_from_window(window_hwnd);

                for workspace in self.workspaces.iter_mut() {
                    if workspace.monitor_handle == monitor && !self.windows.contains(&window_hwnd) {
                        self.windows.push(window_hwnd);
                        workspace.add_window(Window::new(&title, window_hwnd));
                    }
                }
            }
        }
    }

    pub fn list_managable_windows(&self) {
        let windows = self.get_windows();
        let windows = self.get_managed_windows(&windows);

        for window_hwnd in windows {
            let title = Window::get_window_title(window_hwnd);

            println!("{}", &title);
        }
    }

    pub fn arrange_workspaces(&self) {
        for workspace in self.workspaces.iter() {
            workspace.arrange_windows()
        }
    }

    pub fn handle_action(&mut self, action: WorkspaceAction) {
        match action {
            WorkspaceAction::NextAsCurrent => {
                self.get_current_workspace().set_current_next();
            }
            WorkspaceAction::PreviousAsCurrent => {
                self.get_current_workspace().set_current_previous()
            }
            WorkspaceAction::ToggleMode(mode) => {
                self.get_current_workspace().set_current_tiling_mode(&mode);
            }
        }
    }

    pub fn update_current_monitor(&mut self, x: i32, y: i32) {
        for (index, workspace) in self.workspaces.iter().enumerate() {
            if workspace.is_current_workspace(x, y) {
                self.current_workspace_index = index;
            }
        }
    }

    fn get_current_workspace(&mut self) -> &mut Workspace {
        &mut self.workspaces[self.current_workspace_index]
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
