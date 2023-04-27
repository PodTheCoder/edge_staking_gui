// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use check_requirements::pretty_check_string::{self, pretty_err_str};
use tauri::Window;
use utility::{load_initialization_status, load_node_address, log_and_emit};

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
async fn device_start(window: Window, datadir: String) -> bool {
    let backend_communicator = BackendCommunicator {
        status_listener: String::from(STATUSLISTENER),
        data_dir: datadir.clone(),
        front_end_window: window,
    };

    return control_edge_cli::device_start(backend_communicator).await;
}

/// Returns true if initialization is complete, false if not.
#[tauri::command]
fn load_device_initialization_status(window: Window, datadir: String) -> bool {
    let backend_communicator = BackendCommunicator {
        status_listener: String::from(STATUSLISTENER),
        data_dir: datadir.clone(),
        front_end_window: window,
    };

    let initialization_status = load_initialization_status(backend_communicator);

    // Frontend div hide needs a bool.
    if initialization_status == 0 {
        return true; // Show the setup divs
    } else {
        return false; // Hide the setup divs
    }
}

/// Returns true if initialization is complete, false if not.
#[tauri::command]
fn set_device_fully_initialized(window: Window, datadir: String) -> bool {
    let backend_communicator = BackendCommunicator {
        status_listener: String::from(STATUSLISTENER),
        data_dir: datadir.clone(),
        front_end_window: window,
    };

    match utility::config_set_device_initialization_status(true, backend_communicator.clone()) {
        Ok(_) => {
            let ok_message = format!("Congratulations! Your device has fully started! You can now close the Staking GUI.");
            let ok_message_pretty = pretty_check_string::pretty_ok_str(&ok_message, false);
            log_and_emit(ok_message_pretty.clone(), backend_communicator.clone());
            return true;
        }
        Err(err_str) => {
            let err_message = format!(
                "Congratulations, your device is working! Yet was unable to set the initialization config value to true. On next restart, just click the start node button again. If the error keeps persisting, please contact support. Err: {}", err_str
            );
            let err_message_pretty = pretty_err_str(&err_message, false);
            log_and_emit(err_message_pretty, backend_communicator.clone());
            return false;
        }
    }
}

#[tauri::command]
fn set_device_not_initialized(window: Window, datadir: String) -> bool {
    let backend_communicator = BackendCommunicator {
        status_listener: String::from(STATUSLISTENER),
        data_dir: datadir.clone(),
        front_end_window: window,
    };

    match utility::config_set_device_initialization_status(false, backend_communicator.clone()) {
        Ok(_) => {
            let ok_message = format!("Reset program to setup stage.");
            log_and_emit(ok_message.clone(), backend_communicator.clone());
            return true;
        }
        Err(err) => {
            let err_message = format!(
                "Unable to reset program to setup stage. Try pressing the button again and reloading. If the problem persists, contact support. Err: {}", err
            );
            log_and_emit(err_message.clone(), backend_communicator.clone());
            return false;
        }
    }
}

#[tauri::command]
fn load_node_online_status(window: Window, datadir: String) -> String {
    let backend_communicator = BackendCommunicator {
        status_listener: String::from(STATUSLISTENER),
        data_dir: datadir.clone(),
        front_end_window: window,
    };

    // let no_node_found = format!("Unset"); // "Unset" is the error String.
    let node_address = load_node_address(backend_communicator);
    return node_address;
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
fn log_and_emit_from_frontend(message: String, window: Window, datadir: String) {
    // Send message from frontend to backend,
    let backend_communicator = BackendCommunicator {
        status_listener: String::from(STATUSLISTENER),
        data_dir: datadir.clone(),
        front_end_window: window,
    };

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
        Ok(ok_str) => {
            return check_requirements::pretty_check_string::pretty_ok_str(&ok_str, false)
        }
        Err(err_str) => return err_str,
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            install_edge_cli,
            get_edge_cli_download_url,
            device_start,
            device_stop,
            device_info,
            log_and_emit_from_frontend,
            load_device_initialization_status,
            set_device_fully_initialized,
            set_device_not_initialized,
            load_node_online_status,
            add_device
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
