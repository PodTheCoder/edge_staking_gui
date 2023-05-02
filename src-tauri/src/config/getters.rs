use crate::{utility::log_and_emit, BackendCommunicator};

use super::load_config;

/// Returns Initialization status codes
pub fn get_node_address(backend_communicator: BackendCommunicator) -> String {
    let no_node_found = format!("Unset");
    let config;
    match load_config(backend_communicator.clone()) {
        Ok(ok_config) => {
            config = ok_config;
            let node_address = config.address;

            if node_address == no_node_found {
                let err = format!("Node not found in config.");
                log_and_emit(err.clone(), backend_communicator.clone());
                return no_node_found;
            } else {
                let ok_message =
                    format!("Loaded node address successfully: {}", node_address.clone());
                log_and_emit(ok_message, backend_communicator.clone());
                return node_address;
            }
        }
        Err(err) => {
            let err_message = format!("Could not load node address config. Err: {}", err);
            log_and_emit(err_message.clone(), backend_communicator.clone());
            return no_node_found;
        }
    }
}

/// Returns Initialization status codes
pub fn get_initialization_status(backend_communicator: BackendCommunicator) -> u64 {
    let initialized_code: u64 = 0;
    let not_initialied_code: u64 = 1;
    let failed_to_load_code: u64 = 2;
    let config;
    match load_config(backend_communicator.clone()) {
        Ok(ok_config) => config = ok_config,
        Err(err) => {
            let err_message = format!("Could not load initialization status. Err {}", err);
            log_and_emit(err_message.clone(), backend_communicator.clone());
            return failed_to_load_code;
        }
    }
    if config.initialized {
        return initialized_code;
    } else {
        return not_initialied_code;
    }
}
