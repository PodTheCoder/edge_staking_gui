use std::{fs, path::Path};

use serde::{Deserialize, Serialize};

use crate::{utility::log_and_emit, BackendCommunicator};

use self::getters::get_config_path;

pub mod getters;
pub mod setters;

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct ConfigStruct {
    pub initialized: bool, // Has the device be initialized? Set to true when node launched successfully.
    pub is_auto_start_enabled: bool, // Does the node auto start?
    pub launch_minimized: bool, // Does the program start minimized?
    pub last_node_payment: u64, // When was the last node earnings payment to the user?
    pub wallet_address: String, // What is the wallet address from which the device was assigned?
    pub network: String,   // On which Edge network is the device, mainnet or testnet?
    pub address: String,   // What is the device XE address?
    pub private_key: String, // What is the private key of the XE address?
    pub public_key: String, // What is the public key of the XE address?
}

/// Create the default config file locally
pub fn create_default_config(backend_communicator: &BackendCommunicator) -> Result<(), String> {
    let config_path = get_config_path(backend_communicator);

    let default_config = ConfigStruct {
        initialized: false,
        is_auto_start_enabled: false,
        launch_minimized: false,
        last_node_payment: 0,
        wallet_address: format!("Unset"),
        address: format!("Unset"),
        network: format!("Unset"),
        private_key: format!("Unset"),
        public_key: format!("Unset"),
    };
    match confy::store_path(config_path.clone(), default_config) {
        Ok(_) => {
            log_and_emit(
                format!("Created initial config file at location: {}", config_path),
                backend_communicator,
            );
            log_and_emit(format!("Awaiting initial command..."), backend_communicator);
            return Ok({});
        }
        Err(_) => {
            return Err(format!(
                "Unable to store default config at path {}",
                config_path
            ))
        }
    }
}

/// Create config file if it does not yet exist.
pub fn create_config_if_not_exists(
    backend_communicator: &BackendCommunicator,
) -> Result<String, String> {
    let filepath = format!(
        "{}{}",
        backend_communicator.data_dir.clone(),
        format!("config.txt")
    );
    let config_path = Path::new(&filepath);
    if !config_path.exists() {
        match create_default_config(backend_communicator) {
            Ok(_) => {
                let ok_message = format!("Created default config.");
                log_and_emit(ok_message.clone(), backend_communicator);
                return Ok(ok_message);
            }
            Err(err_string) => {
                log_and_emit(err_string.clone(), backend_communicator);
                return Err(err_string);
            }
        }
    }

    //
    let ok_message = format!("Default config already exists.");
    Ok(ok_message)
}

/// Load config file
pub fn get_config(backend_communicator: &BackendCommunicator) -> Result<ConfigStruct, String> {
    let filepath = format!(
        "{}{}",
        backend_communicator.data_dir.clone(),
        format!("config.txt")
    );
    let config_path = Path::new(&filepath);

    match create_config_if_not_exists(backend_communicator) {
        Ok(value) => value,
        Err(value) => return Err(value),
    };

    // Load config from file
    match confy::load_path(config_path) {
        Ok(ok_config) => return Ok(ok_config),
        Err(_) => {
            log_and_emit(
                format!(
                    "Unable to load config at path {}. Assumed corrupted.",
                    config_path.display()
                ),
                backend_communicator,
            );
            log_and_emit(
                format!(
                    "Attempting to restore corrupted config to default state. Path: {}",
                    config_path.display()
                ),
                backend_communicator,
            );
            log_and_emit(
                format!("Removing corrupted file at path {}", config_path.display()),
                backend_communicator,
            );
            match fs::remove_file(config_path) {
                Ok(_) => {
                    log_and_emit(
                        format!(
                            "Removed corrupted config at path : {}",
                            config_path.display()
                        ),
                        backend_communicator,
                    );

                    match create_default_config(backend_communicator) {
                        Ok(_) => {}
                        Err(err) => return Err(err),
                    }
                    let error_message =
                        format!("Could not load config, but restored to default value.");

                    log_and_emit(error_message.clone(), backend_communicator);
                    return Err(error_message);
                }
                Err(_) => {
                    let error_message = format!("Unable to remove corrupted config file.");
                    log_and_emit(error_message.clone(), backend_communicator);
                    return Err(error_message);
                }
            }
        }
    }
}
