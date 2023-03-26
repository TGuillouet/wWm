use windows_sys::Win32::Foundation::RECT;
use windows_sys::Win32::Graphics::Gdi::{
    GetMonitorInfoW, MonitorFromWindow, HMONITOR, MONITORINFO, MONITOR_DEFAULTTONEAREST,
};

pub struct MonitorResolution {
    pub rect: RECT,
    pub width: i32,
    pub height: i32,
}
impl MonitorResolution {
    pub fn is_point_in_monitor(&self, x: i32, y: i32) -> bool {
        x > self.rect.left && x < self.rect.right && y > self.rect.top && y < self.rect.bottom
    }
}

pub fn get_monitor_resolution(monitor: HMONITOR) -> MonitorResolution {
    let mut monitor_info = MONITORINFO {
        cbSize: std::mem::size_of::<MONITORINFO>() as u32,
        dwFlags: 0,
        rcMonitor: RECT {
            bottom: 0,
            left: 0,
            right: 0,
            top: 0,
        },
        rcWork: RECT {
            bottom: 0,
            left: 0,
            right: 0,
            top: 0,
        },
    };

    unsafe {
        GetMonitorInfoW(monitor, &mut monitor_info);
    }

    let screen_width = monitor_info.rcMonitor.right - monitor_info.rcMonitor.left;
    let screen_height = monitor_info.rcMonitor.bottom - monitor_info.rcMonitor.top;

    return MonitorResolution {
        rect: monitor_info.rcMonitor,
        width: screen_width,
        height: screen_height,
    };
}

pub fn get_monitor_from_window(window_hwnd: isize) -> HMONITOR {
    unsafe { MonitorFromWindow(window_hwnd, MONITOR_DEFAULTTONEAREST) }
}
