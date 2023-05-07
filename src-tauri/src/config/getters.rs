use crate::{utility::log_and_emit, BackendCommunicator};

use super::get_config;

pub fn get_config_path(backend_communicator: &BackendCommunicator) -> String {
    let config_path = format!(
        "{}{}",
        backend_communicator.data_dir.clone(),
        format!("config.txt")
    );
    return config_path;
}

/// Returns Initialization status codes
pub fn get_node_address(backend_communicator: &BackendCommunicator) -> String {
    let no_node_found = format!("Unset");
    let config;
    match get_config(backend_communicator) {
        Ok(ok_config) => {
            config = ok_config;
            let node_address = config.address;

            if node_address == no_node_found {
                let err = format!("Node not found in config.");
                log_and_emit(err.clone(), backend_communicator);
                return no_node_found;
            } else {
                let ok_message =
                    format!("Loaded node address successfully: {}", node_address.clone());
                log_and_emit(ok_message, backend_communicator);
                return node_address;
            }
        }
        Err(err) => {
            let err_message = format!("Could not load node address config. Err: {}", err);
            log_and_emit(err_message.clone(), backend_communicator);
            return no_node_found;
        }
    }
}

/// Returns Initialization status codes
pub fn get_initialization_status(backend_communicator: &BackendCommunicator) -> u64 {
    let initialized_code: u64 = 0;
    let not_initialized_code: u64 = 1;
    let failed_to_get_code: u64 = 2;
    let config;
    match get_config(backend_communicator) {
        Ok(ok_config) => config = ok_config,
        Err(err) => {
            let err_message = format!("Could not load initialization status. Err {}", err);
            log_and_emit(err_message.clone(), backend_communicator);
            return failed_to_get_code;
        }
    }
    if config.initialized {
        return initialized_code;
    } else {
        return not_initialized_code;
    }
}

/// Returns autostart status
pub fn get_autostart_status(backend_communicator: &BackendCommunicator) -> bool {
    let config;
    match get_config(backend_communicator) {
        Ok(ok_config) => {
            config = ok_config;
            return config.is_auto_start_enabled;
        }
        Err(err) => {
            let err_message = format!(
                "Could not load autostart status. Assuming false. Err {}",
                err
            );
            log_and_emit(err_message.clone(), backend_communicator);
            return false;
        }
    }
}

/// Returns whether the window should be launched hidden (minimized) or not.
pub fn get_launch_minimized_status(backend_communicator: &BackendCommunicator) -> bool {
    let config;
    match get_config(backend_communicator) {
        Ok(ok_config) => {
            config = ok_config;
            return config.launch_minimized;
        }
        Err(err) => {
            let err_message = format!(
                "Could not load minimized status. Assuming false. Err {}",
                err
            );
            log_and_emit(err_message.clone(), backend_communicator);
            return false;
        }
    }
}

/// Returns autostart status
pub fn get_wallet_address(backend_communicator: &BackendCommunicator) -> String {
    let config;
    match get_config(backend_communicator) {
        Ok(ok_config) => {
            config = ok_config;
            return config.wallet_address;
        }
        Err(err) => {
            let err_message = format!("Could not load wallet address. Assuming false. Err {}", err);
            log_and_emit(err_message.clone(), backend_communicator);
            return format!("CouldNotLoadWalletAddressFromConfig");
        }
    }
}

/// Returns last node payment timestamp. Default & error is 0.
pub fn get_last_node_payment(backend_communicator: &BackendCommunicator) -> u64 {
    let config;
    match get_config(backend_communicator) {
        Ok(ok_config) => {
            config = ok_config;
            return config.last_node_payment;
        }
        Err(err) => {
            let err_message = format!("Could not load last node payment. Err {}", err);
            log_and_emit(err_message.clone(), backend_communicator);
            return 0;
        }
    }
}
