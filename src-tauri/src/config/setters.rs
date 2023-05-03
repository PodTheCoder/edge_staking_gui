use crate::{utility::log_and_emit, BackendCommunicator};

use super::{getters::get_config_path, load_config};

/// Write initialized value to config
pub fn set_device_initialization_status(
    device_initialization_status: bool,
    backend_communicator: BackendCommunicator,
) -> Result<(), std::string::String> {
    let config_path = get_config_path(backend_communicator.clone());

    match load_config(backend_communicator.clone()) {
        Ok(ok_config) => {
            let mut changed_config = ok_config;
            changed_config.initialized = device_initialization_status;

            log_and_emit(
                format!(
                    "Changed config initialization status to: {}",
                    device_initialization_status.to_string()
                ),
                backend_communicator,
            );

            match confy::store_path(config_path, changed_config) {
                Ok(_) => return Ok({}),
                Err(_) => return Err(format!("Unable to store config file at location")),
            }
        }
        Err(err) => return Err(err),
    }
}

/// Save device data into the config file.
pub fn set_device_data(
    network: &String,
    address: &String,
    private_key: &String,
    public_key: &String,
    backend_communicator: BackendCommunicator,
) -> Result<String, String> {
    let config_path = get_config_path(backend_communicator.clone());

    match load_config(backend_communicator.clone()) {
        Ok(ok_config) => {
            let mut changed_config = ok_config;
            changed_config.address = address.to_string();
            changed_config.network = network.to_string();
            changed_config.private_key = private_key.to_string();
            changed_config.public_key = public_key.to_string();

            match confy::store_path(config_path, changed_config) {
                Ok(_) => {
                    let ok_message = format!("Stored device data in config.");
                    log_and_emit(ok_message.clone(), backend_communicator.clone());
                    return Ok(ok_message);
                }
                Err(_) => return Err(format!("Unable to store config file at location")),
            }
        }
        Err(err) => return Err(err),
    }
}

pub fn set_autostart_status(
    autostart_status: bool,
    backend_communicator: BackendCommunicator,
) -> Result<(), std::string::String> {
    let config_path = get_config_path(backend_communicator.clone());

    match load_config(backend_communicator.clone()) {
        Ok(ok_config) => {
            let mut changed_config = ok_config;
            changed_config.is_auto_start_enabled = autostart_status;

            log_and_emit(
                format!(
                    "Changed autostart state to: {}",
                    changed_config.is_auto_start_enabled.to_string()
                ),
                backend_communicator.clone(),
            );

            match confy::store_path(config_path, changed_config) {
                Ok(_) => return Ok({}),
                Err(_) => {
                    let err_msg = format!("Unable to store config file at location");
                    log_and_emit(err_msg.clone(), backend_communicator.clone());
                    return Err(err_msg);
                }
            }
        }
        Err(err) => return Err(err),
    }
}

pub fn set_launch_minimized_status(
    launch_minimized: bool,
    backend_communicator: BackendCommunicator,
) -> Result<(), std::string::String> {
    let config_path = get_config_path(backend_communicator.clone());

    match load_config(backend_communicator.clone()) {
        Ok(ok_config) => {
            let mut changed_config = ok_config;
            changed_config.launch_minimized = launch_minimized;

            log_and_emit(
                format!(
                    "The program will launch minimized: {}",
                    changed_config.launch_minimized.to_string()
                ),
                backend_communicator.clone(),
            );

            match confy::store_path(config_path, changed_config) {
                Ok(_) => return Ok({}),
                Err(_) => {
                    let err_msg = format!("Unable to store config file at location");
                    log_and_emit(err_msg.clone(), backend_communicator.clone());
                    return Err(err_msg);
                }
            }
        }
        Err(err) => return Err(err),
    }
}
