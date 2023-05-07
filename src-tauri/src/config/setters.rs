use crate::{utility::log_and_emit, BackendCommunicator};

use super::{get_config, getters::get_config_path_as_str};

/// Write initialized value to config
pub fn set_device_initialization_status(
    device_initialization_status: bool,
    backend_communicator: &BackendCommunicator,
) -> Result<(), std::string::String> {
    let config_path = get_config_path_as_str(backend_communicator);

    match get_config(backend_communicator) {
        Ok(ok_config) => {
            let mut changed_config = ok_config;
            changed_config.initialized = device_initialization_status;

            log_and_emit(
                format!(
                    "Changed config initialization status to: {}",
                    device_initialization_status
                ),
                backend_communicator,
            );

            match confy::store_path(config_path, changed_config) {
                Ok(_) => Ok(()),
                Err(_) => Err("Unable to store config file at location".to_string()),
            }
        }
        Err(err) => Err(err),
    }
}

/// Save device data into the config file.
pub fn set_device_data(
    network: &String,
    address: &String,
    private_key: &String,
    public_key: &String,
    backend_communicator: &BackendCommunicator,
) -> Result<String, String> {
    let config_path = get_config_path_as_str(backend_communicator);

    match get_config(backend_communicator) {
        Ok(ok_config) => {
            let mut changed_config = ok_config;
            changed_config.address = address.to_string();
            changed_config.network = network.to_string();
            changed_config.private_key = private_key.to_string();
            changed_config.public_key = public_key.to_string();

            match confy::store_path(config_path, changed_config) {
                Ok(_) => {
                    let ok_message = "Stored device data in config.".to_string();
                    log_and_emit(ok_message.clone(), backend_communicator);
                    Ok(ok_message)
                }
                Err(_) => Err("Unable to store config file at location".to_string()),
            }
        }
        Err(err) => Err(err),
    }
}

pub fn set_autostart_status(
    autostart_status: bool,
    backend_communicator: &BackendCommunicator,
) -> Result<(), std::string::String> {
    let config_path = get_config_path_as_str(backend_communicator);

    match get_config(backend_communicator) {
        Ok(ok_config) => {
            let mut changed_config = ok_config;
            changed_config.is_auto_start_enabled = autostart_status;

            log_and_emit(
                format!(
                    "Changed autostart state to: {}",
                    changed_config.is_auto_start_enabled
                ),
                backend_communicator,
            );

            match confy::store_path(config_path, changed_config) {
                Ok(_) => Ok(()),
                Err(_) => {
                    let err_msg = "Unable to store config file at location".to_string();
                    log_and_emit(err_msg.clone(), backend_communicator);
                    Err(err_msg)
                }
            }
        }
        Err(err) => Err(err),
    }
}

pub fn set_launch_minimized_status(
    launch_minimized: bool,
    backend_communicator: &BackendCommunicator,
) -> Result<(), std::string::String> {
    let config_path = get_config_path_as_str(backend_communicator);

    match get_config(backend_communicator) {
        Ok(ok_config) => {
            let mut changed_config = ok_config;
            changed_config.launch_minimized = launch_minimized;

            log_and_emit(
                format!(
                    "The program will launch minimized: {}",
                    changed_config.launch_minimized
                ),
                backend_communicator,
            );

            match confy::store_path(config_path, changed_config) {
                Ok(_) => Ok(()),
                Err(_) => {
                    let err_msg = "Unable to store config file at location".to_string();
                    log_and_emit(err_msg.clone(), backend_communicator);
                    Err(err_msg)
                }
            }
        }
        Err(err) => Err(err),
    }
}

pub fn set_wallet_address(
    wallet_address: String,
    backend_communicator: &BackendCommunicator,
) -> Result<(), std::string::String> {
    let config_path = get_config_path_as_str(backend_communicator);

    match get_config(backend_communicator) {
        Ok(ok_config) => {
            let mut changed_config = ok_config;
            changed_config.wallet_address = wallet_address;

            log_and_emit(
                format!(
                    "Set wallet address in config: {}",
                    changed_config.wallet_address
                ),
                backend_communicator,
            );

            match confy::store_path(config_path, changed_config) {
                Ok(_) => Ok(()),
                Err(_) => {
                    let err_msg = "Unable to store config file at location".to_string();
                    log_and_emit(err_msg.clone(), backend_communicator);
                    Err(err_msg)
                }
            }
        }
        Err(err) => Err(err),
    }
}

pub fn set_last_node_payment(
    last_node_payment: u64,
    backend_communicator: &BackendCommunicator,
) -> Result<(), std::string::String> {
    let config_path = get_config_path_as_str(backend_communicator);

    match get_config(backend_communicator) {
        Ok(ok_config) => {
            let mut changed_config = ok_config;
            changed_config.last_node_payment = last_node_payment;

            log_and_emit(
                format!(
                    "Set last node payment timestamp in config: {}",
                    changed_config.last_node_payment
                ),
                backend_communicator,
            );

            match confy::store_path(config_path, changed_config) {
                Ok(_) => Ok(()),
                Err(_) => {
                    let err_msg = "Unable to store config file at location".to_string();
                    log_and_emit(err_msg.clone(), backend_communicator);
                    Err(err_msg)
                }
            }
        }
        Err(err) => Err(err),
    }
}
