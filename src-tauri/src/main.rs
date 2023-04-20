// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Window;
use utility_events::log_and_emit;

mod check_requirements;
mod control_edge_cli;
mod utility_events;

// Note: Every tauri function requires the following boilerplate to enable communication with front-end:
// datadir: &str
// window: Window
// These are contained in a BackendCommunicator for convenience and passed along each function as arg.
const STATUSLISTENER: &str = "program_status_listener";

#[derive(Clone)]
pub struct BackendCommunicator {
    event_listener: String,
    data_dir: String,
    front_end_window: Window,
}

#[tauri::command]
async fn greet(window: Window, datadir: String, name: String) -> String {
    let _backend_communicator = BackendCommunicator {
        event_listener: String::from(STATUSLISTENER),
        data_dir: datadir.clone(),
        front_end_window: window,
    };

    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn check_requirements(window: Window, datadir: String) -> String {
    let backend_communicator = BackendCommunicator {
        event_listener: String::from(STATUSLISTENER),
        data_dir: datadir.clone(),
        front_end_window: window,
    };

    let check_requirements_future = check_requirements::main(backend_communicator).await;
    match check_requirements_future {
        Ok(res) => return res,
        Err(res) => return res,
    }
}

#[tauri::command]
async fn install_edge_cli(window: Window, datadir: String) -> String {
    let backend_communicator = BackendCommunicator {
        event_listener: String::from(STATUSLISTENER),
        data_dir: datadir.clone(),
        front_end_window: window,
    };
    return check_requirements::check_edge::get_edge_cli(backend_communicator).await;
}

#[tauri::command]
fn get_edge_cli_download_url(window: Window, datadir: String) -> String {
    let backend_communicator = BackendCommunicator {
        event_listener: String::from(STATUSLISTENER),
        data_dir: datadir.clone(),
        front_end_window: window,
    };

    return check_requirements::check_edge::get_edge_cli_download_url(backend_communicator);
}

#[tauri::command]
async fn device_start(window: Window, datadir: String) -> String {
    let backend_communicator = BackendCommunicator {
        event_listener: String::from(STATUSLISTENER),
        data_dir: datadir.clone(),
        front_end_window: window,
    };

    return control_edge_cli::device_start(backend_communicator).await;
}

#[tauri::command]
async fn device_stop(window: Window, datadir: String) -> String {
    let backend_communicator = BackendCommunicator {
        event_listener: String::from(STATUSLISTENER),
        data_dir: datadir.clone(),
        front_end_window: window,
    };

    return control_edge_cli::device_stop(backend_communicator).await;
}

#[tauri::command]
async fn device_info(window: Window, datadir: String) -> String {
    let backend_communicator = BackendCommunicator {
        event_listener: String::from(STATUSLISTENER),
        data_dir: datadir.clone(),
        front_end_window: window,
    };

    return control_edge_cli::device_info(backend_communicator).await;
}

#[tauri::command]
fn emit_from_backend(window: Window, datadir: String) {
    let backend_communicator = BackendCommunicator {
        event_listener: String::from(STATUSLISTENER),
        data_dir: datadir.clone(),
        front_end_window: window,
    };

    let message = format!("Called from backend.");
    log_and_emit(message, backend_communicator);
    return;
}
//TODO: Add persistent boolean if initialization is completed.

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            check_requirements,
            install_edge_cli,
            get_edge_cli_download_url,
            device_start,
            device_stop,
            device_info,
            emit_from_backend
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
