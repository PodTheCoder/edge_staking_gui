use crate::{utility::log_and_emit, BackendCommunicator};

use super::{get_config, ConfigStruct};

pub fn get_config_path_as_str(backend_communicator: &BackendCommunicator) -> String {
    let config_path = format!("{}config.txt", backend_communicator.data_dir.clone()); // eg. appdata/config.txt
    config_path
}

/// Returns Initialization status codes
pub fn get_node_address(backend_communicator: &BackendCommunicator) -> String {
    let no_node_found = "Unset".to_string();
    let config;
    match get_config(backend_communicator) {
        Ok(ok_config) => {
            config = ok_config;
            let node_address = config.address;

            if node_address == no_node_found {
                let err = "Node not found in config.".to_string();
                log_and_emit(err, backend_communicator);
                no_node_found
            } else {
                let ok_message = format!("Loaded node address successfully: {}", node_address);
                log_and_emit(ok_message, backend_communicator);
                node_address
            }
        }
        Err(err) => {
            let err_message = format!("Could not load node address config. Err: {}", err);
            log_and_emit(err_message, backend_communicator);
            no_node_found
        }
    }
}

/// Returns Initialization status codes
pub fn get_initialization_status(backend_communicator: &BackendCommunicator) -> u64 {
    let initialized_code: u64 = 0;
    let not_initialized_code: u64 = 1;
    let failed_to_get_code: u64 = 2;

    let config: ConfigStruct;
    match get_config(backend_communicator) {
        Ok(ok_config) => config = ok_config,
        Err(err) => {
            let err_message = format!("Could not load initialization status. Err {}", err);
            log_and_emit(err_message, backend_communicator);
            return failed_to_get_code;
        }
    };
    if config.initialized {
        initialized_code
    } else {
        not_initialized_code
    }
}

/// Returns autostart status
pub fn get_autostart_status(backend_communicator: &BackendCommunicator) -> bool {
    let config;
    match get_config(backend_communicator) {
        Ok(ok_config) => {
            config = ok_config;
            config.is_auto_start_enabled
        }
        Err(err) => {
            let err_message = format!(
                "Could not load autostart status. Assuming false. Err {}",
                err
            );
            log_and_emit(err_message, backend_communicator);
            false
        }
    }
}

/// Returns whether the window should be launched hidden (minimized) or not.
pub fn get_launch_minimized_status(backend_communicator: &BackendCommunicator) -> bool {
    let config;
    match get_config(backend_communicator) {
        Ok(ok_config) => {
            config = ok_config;
            config.launch_minimized
        }
        Err(err) => {
            let err_message = format!(
                "Could not load minimized status. Assuming false. Err {}",
                err
            );
            log_and_emit(err_message, backend_communicator);
            false
        }
    }
}

/// Returns autostart status
pub fn get_wallet_address(backend_communicator: &BackendCommunicator) -> String {
    let config;
    match get_config(backend_communicator) {
        Ok(ok_config) => {
            config = ok_config;
            config.wallet_address
        }
        Err(err) => {
            let err_message = format!("Could not load wallet address. Assuming false. Err {}", err);
            log_and_emit(err_message, backend_communicator);
            "CouldNotLoadWalletAddressFromConfig".to_string()
        }
    }
}

/// Returns last node payment timestamp. Default & error is 0.
pub fn get_last_node_payment(backend_communicator: &BackendCommunicator) -> u64 {
    let config;
    match get_config(backend_communicator) {
        Ok(ok_config) => {
            config = ok_config;
            config.last_node_payment
        }
        Err(err) => {
            let err_message = format!("Could not load last node payment. Err {}", err);
            log_and_emit(err_message, backend_communicator);
            0
        }
    }
}

/// Returns stake id
pub fn get_stake_id(backend_communicator: &BackendCommunicator) -> String {
    let config;
    match get_config(backend_communicator) {
        Ok(ok_config) => {
            config = ok_config;
            config.stake_id
        }
        Err(err) => {
            let err_message = format!("Could not load stake id. Err {}", err);
            log_and_emit(err_message, backend_communicator);
            "Not Set".to_string()
        }
    }
}

/// Returns network
pub fn get_network(backend_communicator: &BackendCommunicator) -> String {
    let config;
    match get_config(backend_communicator) {
        Ok(ok_config) => {
            config = ok_config;
            config.network
        }
        Err(err) => {
            let err_message = format!("Could not load network. Err {}", err);
            log_and_emit(err_message, backend_communicator);
            "Not Set".to_string()
        }
    }
}

// Get edge binary name based on network value
pub fn get_edge_binary_name(backend_communicator: &BackendCommunicator) -> String {
    let network = get_network(backend_communicator);
    let mainnet = String::from("mainnet");
    let mainnet_bin = String::from("edge.exe");
    let testnet = String::from("testnet");
    let testnet_bin = String::from("edgetest.exe");
    if network == mainnet {
        let bin_name = mainnet_bin;
        let ok_message = format!(
            "Derived binary name {} based on network {}",
            bin_name, network
        );
        log_and_emit(ok_message, backend_communicator);
        bin_name
    } else if network == testnet {
        let bin_name = testnet_bin;
        let ok_message = format!(
            "Derived binary name {} based on network {}",
            bin_name, network
        );
        log_and_emit(ok_message, backend_communicator);
        bin_name
    } else {
        let err_message = format!("Unable to derive binary name based on network {}", network);
        log_and_emit(err_message.clone(), backend_communicator);
        err_message
    }
}
