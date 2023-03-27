use windows_sys::Win32::Foundation::{LPARAM, RECT};
use windows_sys::Win32::Graphics::Gdi::{
    EnumDisplayDevicesW, EnumDisplayMonitors, GetMonitorInfoW, MonitorFromWindow, DISPLAY_DEVICEW,
    HDC, HMONITOR, MONITORINFO, MONITOR_DEFAULTTONEAREST,
};

#[derive(Clone)]
pub struct Monitor {
    pub name: String,
    pub monitor_handle: HMONITOR,
    pub rect: RECT,
    pub width: i32,
    pub height: i32,
}
impl Monitor {
    pub fn is_point_in_monitor(&self, x: i32, y: i32) -> bool {
        x > self.rect.left && x < self.rect.right && y > self.rect.top && y < self.rect.bottom
    }
}

pub struct Monitors;
impl Monitors {
    pub fn get_monitors_list() -> Vec<Monitor> {
        let mut monitors = Vec::new();
        let monitor_handles = Monitors::get_monitors_handles();

        let mut display_device: DISPLAY_DEVICEW = unsafe { std::mem::zeroed() };
        display_device.cb = std::mem::size_of::<DISPLAY_DEVICEW>() as u32;
        let mut i: usize = 0;
        while unsafe { EnumDisplayDevicesW(std::ptr::null(), i as u32, &mut display_device, 0) }
            != 0
        {
            let mut monitor: DISPLAY_DEVICEW = unsafe { std::mem::zeroed() };
            monitor.cb = std::mem::size_of::<DISPLAY_DEVICEW>() as u32;

            let mut j: usize = 0;
            while unsafe {
                EnumDisplayDevicesW(
                    display_device.DeviceName.as_ptr(),
                    j as u32,
                    &mut monitor,
                    0,
                )
            } != 0
            {
                let device_name = Monitors::to_string(&monitor.DeviceString);

                monitors.push(Monitors::create_monitor(device_name, monitor_handles[i]));
                j += 1;
            }

            display_device = unsafe { std::mem::zeroed() };
            display_device.cb = std::mem::size_of::<DISPLAY_DEVICEW>() as u32;

            i += 1;
        }

        monitors
    }

    fn get_monitors_handles() -> Vec<HMONITOR> {
        let mut monitors: Vec<HMONITOR> = Vec::new();

        unsafe {
            EnumDisplayMonitors(
                0,
                std::ptr::null_mut(),
                Some(enum_monitors_callback),
                &mut monitors as *mut Vec<HMONITOR> as LPARAM,
            );
        }

        monitors
    }

    fn create_monitor(device_name: String, monitor: HMONITOR) -> Monitor {
        let mut monitor_info: MONITORINFO = unsafe { std::mem::zeroed() };
        monitor_info.cbSize = std::mem::size_of::<MONITORINFO>() as u32;

        unsafe {
            GetMonitorInfoW(monitor, &mut monitor_info);
        }

        let screen_width = monitor_info.rcMonitor.right - monitor_info.rcMonitor.left;
        let screen_height = monitor_info.rcMonitor.bottom - monitor_info.rcMonitor.top;

        return Monitor {
            name: device_name,
            monitor_handle: monitor,
            rect: monitor_info.rcMonitor,
            width: screen_width,
            height: screen_height,
        };
    }

    fn to_string(ptr: &[u16]) -> String {
        let len = ptr.iter().position(|&c| c == 0).unwrap_or(ptr.len());
        String::from_utf16_lossy(&ptr[0..len])
    }
}

pub fn get_monitor_from_window(window_hwnd: isize) -> HMONITOR {
    unsafe { MonitorFromWindow(window_hwnd, MONITOR_DEFAULTTONEAREST) }
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
