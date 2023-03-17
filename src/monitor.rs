use windows_sys::Win32::Foundation::RECT;
use windows_sys::Win32::Graphics::Gdi::{GetMonitorInfoW, HMONITOR, MONITORINFO};

#[derive(Debug)]
pub struct MonitorResolution {
    pub width: i32,
    pub height: i32,
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
        width: screen_width,
        height: screen_height,
    };
}