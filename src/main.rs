use actions::WmAction;
use dotenv::dotenv;
use input::{create_inputs_window, unregister_hotkeys};
use std::{mem::zeroed, sync::mpsc::Sender, thread::JoinHandle};

use config::{Config, ConfigBuilder};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    DispatchMessageW, GetMessageA, TranslateMessage,
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

    let mut window_manager = WindowManager::new(config);
    window_manager.get_monitors();
    window_manager.list_managable_windows();

    let inputs_thread_handle = init_inputs_thread(hotkeys_sender);

    window_manager.fetch_windows();
    window_manager.arrange_workspaces();
    loop {
        match hotkeys_receiver.try_recv() {
            Ok(action) => match action {
                WmAction::Ping => window_manager.handle_action(action),
                WmAction::Close { hwnd } => {
                    close_inputs_window(hwnd);
                    break;
                }
            },
            Err(_) => {}
        }

        window_manager.fetch_windows();
        window_manager.arrange_workspaces();
    }
    inputs_thread_handle
        .join()
        .expect("Could not join the inputs thread !");
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

fn init_inputs_thread(hotkeys_sender: Sender<WmAction>) -> JoinHandle<()> {
    std::thread::spawn(move || {
        let global_window_data = Box::new(GlobalWindowData {
            sender: hotkeys_sender,
        });
        let global_hwnd = create_inputs_window(global_window_data);
        register_hotkeys(global_hwnd);
        let mut msg = unsafe { zeroed() };
        while unsafe { GetMessageA(&mut msg, global_hwnd, 0, 0) > 0 } {
            unsafe { TranslateMessage(&msg) };
            unsafe { DispatchMessageW(&msg) };
        }
    })
}
