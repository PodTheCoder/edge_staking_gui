#![allow(non_snake_case)] // Disable non_snake_case check for privateKey.
                          // Impedance mismatch between Javascript best practice of camelCase and Rust best practice of snake_case
                          // Edge JSON expects Javascript. Javascript best practice used.
use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, OpenOptions};
use std::io::Write;

use crate::config::getters::get_network;
use crate::config::setters::set_device_data;
use crate::docker::{
    copy_data_to_running_container, remove_temporary_container,
    start_docker_container_for_copying_data,
};
use crate::utility::log_and_emit;
use crate::{check_requirements, BackendCommunicator};

/// Flow for creating a device token. If successful displays the next step.
/// Can be rerun multiple times to generate a different device token.
pub async fn create_device_code(
    address: String,
    private_key: String,
    public_key: String,
    backend_communicator: &BackendCommunicator,
) -> Result<String, String> {
    let network = get_network(backend_communicator);
    println!(
        "add_device_from_frontend function called. address {}, private_key {}, public_key {}",
        address, private_key, public_key
    );
    // TODO: Check if input is correct.

    #[derive(Default, Debug, Serialize, Deserialize, Clone)]
    pub struct DeviceJSON {
        pub address: String,    // XE address
        pub privateKey: String, // Private key of XE address
    }

    // Check requirements to see if Docker is running
    match check_requirements::main(false, false, true, false, backend_communicator).await {
        Ok(_) => {}
        Err(err_str) => return Err(err_str),
    }

    // Save info to config file.

    match set_device_data(
        &network,
        &address,
        &private_key,
        &public_key,
        backend_communicator,
    ) {
        Ok(_) => {}
        Err(err) => return Err(err),
    }

    let fields_hashmap: HashMap<String, String> = HashMap::from([
        ("network".to_string(), network.clone()),
        ("address".to_string(), address.clone()),
        ("privateKey".to_string(), private_key.clone()),
        ("publicKey".to_string(), public_key),
    ]); //

    // Convert config file to individual files in preparation of Docker

    let mut filepaths: Vec<String> = vec![];
    for (filename, file_value) in fields_hashmap.clone() {
        match helper_save_device_file(filename, file_value, backend_communicator) {
            Ok(filepath) => filepaths.push(filepath),
            Err(err_str) => return Err(err_str),
        }
    }

    // Copy files to Docker
    match start_docker_container_for_copying_data(backend_communicator) {
        Ok(_) => {}
        Err(err_str) => return Err(err_str),
    }

    for filepath in filepaths {
        match copy_data_to_running_container(filepath.clone(), backend_communicator) {
            Ok(_) => {
                let ok_message = format!("Copied file {} to edge-device-data", filepath.clone());
                log_and_emit(ok_message.clone(), backend_communicator);
            }
            Err(err_str) => return Err(err_str),
        }
    }

    // Cleanup

    match remove_temporary_container(backend_communicator) {
        Ok(_) => {}
        Err(err_str) => {
            return Err(err_str);
        }
    }

    for (filename, _) in fields_hashmap {
        match helper_delete_device_file(filename, backend_communicator) {
            Ok(_) => {}
            Err(err_str) => return Err(err_str),
        }
    }

    // Success!

    let success_message = "Successfully added device data.".to_string();
    log_and_emit(success_message, backend_communicator);

    let device_json = DeviceJSON {
        address,
        privateKey: private_key,
    };

    let device_json_string;
    match serde_json::to_string(&device_json) {
        Ok(json_str) => {
            device_json_string = json_str;
        }
        Err(err_str) => return Err(err_str.to_string()),
    }

    let url_safe_device_code =
        general_purpose::URL_SAFE_NO_PAD.encode(device_json_string.as_bytes());

    let network = get_network(backend_communicator);
    let mainnet = String::from("mainnet");
    let mainnet_wallet_url = String::from("https://wallet.xe.network/staking");
    let testnet = String::from("testnet");
    let testnet_wallet_url = String::from("https://wallet.test.network/staking");

    let wallet_url;
    if network == mainnet {
        wallet_url = mainnet_wallet_url;
        let ok_message = format!(
            "Derived staking wallet url {} based on network in config: {}",
            wallet_url, network
        );
        log_and_emit(ok_message, backend_communicator);
    } else if network == testnet {
        wallet_url = testnet_wallet_url;
        let ok_message = format!(
            "Derived staking wallet url {} based on network in config: {}",
            wallet_url, network
        );
        log_and_emit(ok_message, backend_communicator);
    } else {
        let err_message = format!(
            "Could not derive wallet url based on network in config: {}",
            network
        );
        log_and_emit(err_message.clone(), backend_communicator);
        return Err(err_message);
    }

    let next_step = format!(
        "Please assign your device token at {}. Your device token is : {}",
        wallet_url, url_safe_device_code
    );
    let ok_msg = String::from("Your device token was created.");
    log_and_emit(ok_msg, backend_communicator);
    Ok(next_step)
}

/// Save field as file with same name, returns filepath if successful
fn helper_save_device_file(
    file_name: String,
    field_value: String,
    backend_communicator: &BackendCommunicator,
) -> Result<String, String> {
    let file_path = format!("{}{}", backend_communicator.data_dir, file_name);
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(file_path.clone());
    match file {
        Ok(ok_file) => {
            let mut valid_file = ok_file;
            match valid_file.write(field_value.as_bytes()) {
                Ok(_) => Ok(file_path),
                Err(_) => Err(format!("Unable to write file: {}", file_path)),
            }
        }
        Err(_) => Err(format!("Unable to open file {}", file_path)),
    }
}

/// Delete device file
fn helper_delete_device_file(
    file_name: String,
    backend_communicator: &BackendCommunicator,
) -> Result<String, String> {
    let file_path = format!("{}{}", backend_communicator.data_dir, file_name);
    match fs::remove_file(file_path.clone()) {
        Ok(_) => {
            let ok_msg = format!("File {} was successfully cleaned up.", file_path);
            log_and_emit(ok_msg.clone(), backend_communicator);
            Ok(ok_msg)
        }
        Err(_) => Err(format!("Unable to delete file {}", file_path)),
    }
}
