// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// Disable
#![allow(clippy::needless_late_init)]

use tauri::{CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem};

use check_requirements::pretty_check_string::{self, pretty_err_str};
use config::{
    getters::{
        get_autostart_status, get_initialization_status, get_last_node_payment,
        get_launch_minimized_status, get_node_address, get_wallet_address,
    },
    setters::{
        set_autostart_status, set_device_initialization_status, set_last_node_payment,
        set_launch_minimized_status, set_wallet_address,
    },
};
use tauri::{Manager, Window};
use tauri_plugin_autostart::MacosLauncher;
use utility::log_and_emit;

mod check_requirements;
mod config;
mod control_edge_cli;
mod device;
mod docker;
mod utility;

#[derive(Clone, serde::Serialize)]
struct Payload {
    args: Vec<String>,
    cwd: String,
}

// Note: Every tauri function requires the following boilerplate to enable communication with front-end:
// datadir: &str
// window: Window
// These are contained in a BackendCommunicator for convenience and passed along each function as arg.
const STATUSLISTENER: &str = "program_status_listener";

#[derive(Clone)]
pub struct BackendCommunicator {
    status_listener: String,
    data_dir: String,
    front_end_window: Window,
}

#[tauri::command]
async fn install_edge_cli_from_frontend(window: Window, datadir: String) -> bool {
    let backend_communicator = &BackendCommunicator {
        status_listener: String::from(STATUSLISTENER),
        data_dir: datadir.clone(),
        front_end_window: window,
    };

    check_requirements::check_edge::get_edge_cli_binary(backend_communicator).await
}

#[tauri::command]
fn get_edge_cli_download_url_from_frontend(window: Window, datadir: String) -> String {
    let backend_communicator = &BackendCommunicator {
        status_listener: String::from(STATUSLISTENER),
        data_dir: datadir,
        front_end_window: window,
    };

    check_requirements::check_edge::get_edge_cli_download_url_from_frontend(backend_communicator)
}

#[tauri::command]
async fn device_start_from_frontend(
    checklatestbinary: bool,
    stake: String,
    window: Window,
    datadir: String,
) -> bool {
    let backend_communicator = &BackendCommunicator {
        status_listener: String::from(STATUSLISTENER),
        data_dir: datadir.clone(),
        front_end_window: window,
    };
    match config::setters::set_stake_id(stake, backend_communicator) {
        Ok(_) => (),
        Err(_) => return false,
    };
    control_edge_cli::device_start_from_frontend(checklatestbinary, backend_communicator).await
}

/// Returns true if initialization is complete, false if not.
#[tauri::command]
fn get_device_initialization_status_from_frontend(window: Window, datadir: String) -> bool {
    let backend_communicator = &BackendCommunicator {
        status_listener: String::from(STATUSLISTENER),
        data_dir: datadir,
        front_end_window: window,
    };

    let initialization_status = get_initialization_status(backend_communicator);

    // Frontend div hide needs a bool.
    initialization_status == 0
}

/// Returns true if initialization is complete, false if not.
#[tauri::command]
fn set_device_fully_initialized_from_frontend(window: Window, datadir: String) -> bool {
    let backend_communicator = &BackendCommunicator {
        status_listener: String::from(STATUSLISTENER),
        data_dir: datadir,
        front_end_window: window,
    };

    match set_device_initialization_status(true, backend_communicator) {
        Ok(_) => {
            let ok_message = "Congratulations! Your device has fully started! You can now close the Staking GUI.".to_string();
            let ok_message_pretty = pretty_check_string::pretty_ok_str(&ok_message, false);
            log_and_emit(ok_message_pretty, backend_communicator);
            true
        }
        Err(err_str) => {
            let err_message = format!(
                "Congratulations, your device is working! Yet was unable to set the initialization config value to true. On next restart, just click the start node button again. If the error keeps persisting, please contact support. Err: {}", err_str
            );
            let err_message_pretty = pretty_err_str(&err_message, false);
            log_and_emit(err_message_pretty, backend_communicator);
            false
        }
    }
}

#[tauri::command]
fn set_device_not_initialized_from_frontend(window: Window, datadir: String) -> bool {
    let backend_communicator = &BackendCommunicator {
        status_listener: String::from(STATUSLISTENER),
        data_dir: datadir,
        front_end_window: window,
    };

    match set_device_initialization_status(false, backend_communicator) {
        Ok(_) => {
            let ok_message = "Reset program to setup stage.".to_string();
            log_and_emit(ok_message, backend_communicator);
            true
        }
        Err(err) => {
            let err_message = format!(
                "Unable to reset program to setup stage. Try pressing the button again and reloading. If the problem persists, contact support. Err: {}", err
            );
            log_and_emit(err_message, backend_communicator);
            false
        }
    }
}

#[tauri::command]
fn get_node_address_from_frontend(window: Window, datadir: String) -> String {
    let backend_communicator = &BackendCommunicator {
        status_listener: String::from(STATUSLISTENER),
        data_dir: datadir,
        front_end_window: window,
    };

    // let no_node_found = format!("Unset"); // "Unset" is the error String.

    get_node_address(backend_communicator)
}

#[tauri::command]
async fn device_stop_from_frontend(checklatestbinary: bool, window: Window, datadir: String) {
    let backend_communicator = &BackendCommunicator {
        status_listener: String::from(STATUSLISTENER),
        data_dir: datadir.clone(),
        front_end_window: window,
    };

    control_edge_cli::device_stop_from_frontend(checklatestbinary, backend_communicator).await;
}

#[tauri::command]
async fn update_edge_cli_from_frontend(
    checklatestbinary: bool,
    window: Window,
    datadir: String,
) -> bool {
    let backend_communicator = &BackendCommunicator {
        status_listener: String::from(STATUSLISTENER),
        data_dir: datadir.clone(),
        front_end_window: window,
    };

    let mut is_edge_cli_latest_ver =
        control_edge_cli::update_edge_cli(checklatestbinary, backend_communicator).await;

    if !is_edge_cli_latest_ver {
        // Update failed via CLI. Trying fallback.
        let err_msg = "Unable to update Edge CLI via update command. Trying fallback method using get_cli_binary.".to_string();
        log_and_emit(err_msg, backend_communicator);
        is_edge_cli_latest_ver =
            check_requirements::check_edge::get_edge_cli_binary(backend_communicator).await;
    }

    is_edge_cli_latest_ver
}

#[tauri::command]
fn log_and_emit_from_frontend(message: String, window: Window, datadir: String) {
    // Send message from frontend to backend,
    let backend_communicator = &BackendCommunicator {
        status_listener: String::from(STATUSLISTENER),
        data_dir: datadir,
        front_end_window: window,
    };

    log_and_emit(message, backend_communicator);
}

#[tauri::command]
async fn add_device_from_frontend(
    address: String,
    privatekey: String,
    publickey: String,
    window: Window,
    datadir: String,
) -> String {
    let backend_communicator = &BackendCommunicator {
        status_listener: String::from(STATUSLISTENER),
        data_dir: datadir.clone(),
        front_end_window: window,
    };

    match device::create_device_code(address, privatekey, publickey, backend_communicator).await {
        Ok(ok_str) => check_requirements::pretty_check_string::pretty_ok_str(&ok_str, false),
        Err(err_str) => err_str,
    }
}

#[tauri::command]
fn get_autostart_status_from_frontend(window: Window, datadir: String) -> bool {
    let backend_communicator = &BackendCommunicator {
        status_listener: String::from(STATUSLISTENER),
        data_dir: datadir,
        front_end_window: window,
    };

    get_autostart_status(backend_communicator)
}

#[tauri::command]
fn set_autostart_status_from_frontend(
    autostartstatus: bool,
    window: Window,
    datadir: String,
) -> bool {
    let backend_communicator = &BackendCommunicator {
        status_listener: String::from(STATUSLISTENER),
        data_dir: datadir,
        front_end_window: window,
    };

    set_autostart_status(autostartstatus, backend_communicator).is_ok()
}

#[tauri::command]
fn get_launch_minimized_status_from_frontend(window: Window, datadir: String) -> bool {
    let backend_communicator = &BackendCommunicator {
        status_listener: String::from(STATUSLISTENER),
        data_dir: datadir,
        front_end_window: window,
    };

    get_launch_minimized_status(backend_communicator)
}

#[tauri::command]
fn set_launch_minimized_status_from_frontend(
    launchminimized: bool,
    window: Window,
    datadir: String,
) -> bool {
    let backend_communicator = &BackendCommunicator {
        status_listener: String::from(STATUSLISTENER),
        data_dir: datadir,
        front_end_window: window,
    };

    set_launch_minimized_status(launchminimized, backend_communicator).is_ok()
}

#[tauri::command]
fn get_wallet_address_from_frontend(window: Window, datadir: String) -> String {
    let backend_communicator = &BackendCommunicator {
        status_listener: String::from(STATUSLISTENER),
        data_dir: datadir,
        front_end_window: window,
    };

    get_wallet_address(backend_communicator)
}

#[tauri::command]
fn set_wallet_address_from_frontend(
    walletaddress: String,
    window: Window,
    datadir: String,
) -> bool {
    let backend_communicator = &BackendCommunicator {
        status_listener: String::from(STATUSLISTENER),
        data_dir: datadir,
        front_end_window: window,
    };

    set_wallet_address(walletaddress, backend_communicator).is_ok()
}

#[tauri::command]
fn get_last_node_payment_from_frontend(window: Window, datadir: String) -> u64 {
    let backend_communicator = &BackendCommunicator {
        status_listener: String::from(STATUSLISTENER),
        data_dir: datadir,
        front_end_window: window,
    };

    get_last_node_payment(backend_communicator)
}

#[tauri::command]
fn set_last_node_payment_from_frontend(
    lastnodepayment: u64,
    window: Window,
    datadir: String,
) -> bool {
    let backend_communicator = &BackendCommunicator {
        status_listener: String::from(STATUSLISTENER),
        data_dir: datadir,
        front_end_window: window,
    };

    set_last_node_payment(lastnodepayment, backend_communicator).is_ok()
}

fn main() {
    let show = CustomMenuItem::new("show".to_string(), "Show");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let tray_menu = SystemTrayMenu::new()
        .add_item(show)
        .add_item(quit)
        .add_native_item(SystemTrayMenuItem::Separator);
    let tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            install_edge_cli_from_frontend,
            get_edge_cli_download_url_from_frontend,
            device_start_from_frontend,
            device_stop_from_frontend,
            update_edge_cli_from_frontend,
            log_and_emit_from_frontend,
            get_device_initialization_status_from_frontend,
            set_device_fully_initialized_from_frontend,
            set_device_not_initialized_from_frontend,
            get_node_address_from_frontend,
            get_autostart_status_from_frontend,
            set_autostart_status_from_frontend,
            get_launch_minimized_status_from_frontend,
            set_launch_minimized_status_from_frontend,
            get_wallet_address_from_frontend,
            set_wallet_address_from_frontend,
            get_last_node_payment_from_frontend,
            set_last_node_payment_from_frontend,
            add_device_from_frontend
        ])
        .system_tray(tray)
        .on_window_event(|event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event.event() {
                event.window().hide().unwrap();
                api.prevent_close();
            }
        })
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::DoubleClick {
                position: _,
                size: _,
                ..
            } => {
                println!("system tray received a double click");
                let window = app.get_window("main").unwrap();
                window.show().unwrap();
                window.unminimize().unwrap();
            }
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "show" => {
                    let window = app.get_window("main").unwrap();
                    window.show().unwrap();
                    window.unminimize().unwrap();
                }
                "quit" => {
                    std::process::exit(0);
                }
                _ => {}
            },
            _ => {}
        })
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec![]), /* arbitrary number of args to pass to your app */
        ))
        .plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
            println!("{}, {argv:?}, {cwd}", app.package_info().name);

            app.emit_all("single-instance", Payload { args: argv, cwd })
                .unwrap();
        }))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
