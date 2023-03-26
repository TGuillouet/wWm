use actions::WmAction;
use dotenv::dotenv;
use input::create_inputs_window;
use std::{
    mem::zeroed,
    sync::mpsc::{Receiver, Sender},
    thread::JoinHandle,
};

use config::{Config, ConfigBuilder};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    DispatchMessageW, GetMessageA, PeekMessageA, PostMessageA, TranslateMessage, PM_REMOVE,
    WM_CLOSE, WM_HOTKEY,
};
use wm::WindowManager;

use crate::input::{close_inputs_window, register_hotkeys};

mod actions;
mod config;
mod input;
mod monitor;
mod tree;
mod windows;
mod wm;
mod workspace;

pub struct GlobalWindowData {
    sender: Sender<WmAction>,
}

fn main() {
    dotenv().ok();

    let config = init_configuration();

    let (hotkeys_sender, hotkeys_receiver) = std::sync::mpsc::channel();
    let (shutdown_sender, shutdown_receiver) = std::sync::mpsc::channel::<bool>();

    let mut window_manager = WindowManager::new(config);
    window_manager.get_monitors();
    window_manager.list_managable_windows();

    let inputs_thread_handle = init_inputs_thread(hotkeys_sender, shutdown_receiver);

    window_manager.fetch_windows();
    window_manager.arrange_workspaces();
    loop {
        match hotkeys_receiver.try_recv() {
            Ok(action) => match action {
                WmAction::Workspace(action) => window_manager.handle_action(action),
                WmAction::Close { hwnd } => {
                    close_inputs_window(hwnd);
                    shutdown_sender
                        .send(true)
                        .expect("Could not send the shutdown message");
                    break;
                }
            },
            Err(_) => {}
        }

        window_manager.fetch_windows();
        window_manager.arrange_workspaces();
        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    for handle in inputs_thread_handle.into_iter() {
        handle.join().expect("Could not join the inputs thread !");
    }
}

fn init_configuration() -> Config {
    let config_path_str = std::env::var("CONFIG_PATH").expect("Could not load the config file !");
    let config_path = std::path::Path::new(config_path_str.as_str());

    let config_file = config_path.join("config");
    let config_file_str = config_file
        .to_str()
        .expect("Could not build the config path !");

    ConfigBuilder::new(config_file_str).build()
}

fn init_inputs_thread(
    hotkeys_sender: Sender<WmAction>,
    shutdown_receiver: Receiver<bool>,
) -> [JoinHandle<()>; 2] {
    let (hwnd_sender, hwnd_receiver) = std::sync::mpsc::channel::<isize>();

    let desktop_handle = std::thread::spawn(move || {
        register_hotkeys();
        let mut msg = unsafe { zeroed() };

        let global_window_hwnd;
        loop {
            match hwnd_receiver.try_recv() {
                Ok(hwnd) => {
                    global_window_hwnd = hwnd;
                    break;
                }
                Err(_) => {}
            }
        }

        loop {
            match shutdown_receiver.try_recv() {
                Ok(_) => {
                    println!("Receive shutdown");
                    break;
                }
                Err(_) => {}
            }

            if unsafe { PeekMessageA(&mut msg, 0, 0, 0, PM_REMOVE) } > 0 {
                if msg.message == WM_HOTKEY {
                    unsafe { PostMessageA(global_window_hwnd, msg.message, msg.wParam, 0) };
                }
            }
        }
    });

    let window_handle = std::thread::spawn(move || {
        let global_window_data = Box::new(GlobalWindowData {
            sender: hotkeys_sender,
        });
        let global_hwnd = create_inputs_window(global_window_data);
        hwnd_sender
            .send(global_hwnd)
            .expect("Could not send the hwnd to the other thread !");

        let mut msg = unsafe { zeroed() };
        while unsafe { GetMessageA(&mut msg, global_hwnd, 0, 0) } != 0 {
            unsafe { TranslateMessage(&msg) };
            unsafe { DispatchMessageW(&msg) };

            if msg.message == WM_CLOSE {
                break;
            }
        }
    });

    [desktop_handle, window_handle]
}
