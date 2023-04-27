// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Window;
use utility::{load_config, log_and_emit};

mod check_requirements;
mod control_edge_cli;
mod device;
mod docker;
mod utility;

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
async fn install_edge_cli(window: Window, datadir: String) -> String {
    let backend_communicator = BackendCommunicator {
        status_listener: String::from(STATUSLISTENER),
        data_dir: datadir.clone(),
        front_end_window: window,
    };
    return check_requirements::check_edge::get_edge_cli_binary(backend_communicator).await;
}

#[tauri::command]
fn get_edge_cli_download_url(window: Window, datadir: String) -> String {
    let backend_communicator = BackendCommunicator {
        status_listener: String::from(STATUSLISTENER),
        data_dir: datadir.clone(),
        front_end_window: window,
    };

    return check_requirements::check_edge::get_edge_cli_download_url(backend_communicator);
}

#[tauri::command]
async fn device_start(window: Window, datadir: String) -> String {
    let backend_communicator = BackendCommunicator {
        status_listener: String::from(STATUSLISTENER),
        data_dir: datadir.clone(),
        front_end_window: window,
    };

    return control_edge_cli::device_start(backend_communicator).await;
}

#[tauri::command]
fn load_config_frontend(window: Window, datadir: String) -> String {
    let backend_communicator = BackendCommunicator {
        status_listener: String::from(STATUSLISTENER),
        data_dir: datadir.clone(),
        front_end_window: window,
    };
    match load_config(backend_communicator.clone()) {
        Ok(_) => return format!("Config initialized successfully."),
        Err(_) => return format!("Config failed to initialize."),
    }
}

#[tauri::command]
async fn device_stop(window: Window, datadir: String) -> String {
    let backend_communicator = BackendCommunicator {
        status_listener: String::from(STATUSLISTENER),
        data_dir: datadir.clone(),
        front_end_window: window,
    };

    return control_edge_cli::device_stop(backend_communicator).await;
}

#[tauri::command]
async fn device_info(window: Window, datadir: String) -> String {
    let backend_communicator = BackendCommunicator {
        status_listener: String::from(STATUSLISTENER),
        data_dir: datadir.clone(),
        front_end_window: window,
    };

    return control_edge_cli::device_info(backend_communicator).await;
}

#[tauri::command]
fn emit_from_backend(window: Window, datadir: String) {
    let backend_communicator = BackendCommunicator {
        status_listener: String::from(STATUSLISTENER),
        data_dir: datadir.clone(),
        front_end_window: window,
    };

    let message = format!("Called from backend.");
    log_and_emit(message, backend_communicator);
    return;
}

#[tauri::command]
async fn add_device(
    address: String,
    privatekey: String,
    publickey: String,
    window: Window,
    datadir: String,
) -> String {
    let backend_communicator = BackendCommunicator {
        status_listener: String::from(STATUSLISTENER),
        data_dir: datadir.clone(),
        front_end_window: window,
    };

    match device::create_device_code(address, privatekey, publickey, backend_communicator).await {
        Ok(ok_str) => return ok_str,
        Err(err_str) => return err_str,
    }
}

//TODO: Add persistent boolean if initialization is completed.

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            install_edge_cli,
            get_edge_cli_download_url,
            device_start,
            device_stop,
            device_info,
            emit_from_backend,
            load_config_frontend,
            add_device
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
