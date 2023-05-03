use base64::{engine::general_purpose, Engine as _};
use std::collections::HashMap;
use std::fs::{self, OpenOptions};
use std::io::Write;

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
    backend_communicator: BackendCommunicator,
) -> Result<String, String> {
    let network = format!("mainnet");
    println!(
        "add_device_from_frontend function called. address {}, private_key {}, public_key {}",
        address, private_key, public_key
    );
    // TODO: Check if input is correct.

    // Check requirements to see if Docker is running
    match check_requirements::main(false, false, true, false, backend_communicator.clone()).await {
        Ok(_) => {}
        Err(err_str) => return Err(err_str),
    }

    // Save info to config file.

    match set_device_data(
        &network,
        &address,
        &private_key,
        &public_key,
        backend_communicator.clone(),
    ) {
        Ok(_) => {}
        Err(err) => return Err(err),
    }

    let fields_hashmap: HashMap<String, String> = HashMap::from([
        (format!("network"), network.clone()),
        (format!("address"), address.clone()),
        (format!("privateKey"), private_key),
        (format!("publicKey"), public_key),
    ]); //

    // Convert config file to individual files in preparation of Docker

    let mut filepaths: Vec<String> = vec![];
    for (filename, file_value) in fields_hashmap.clone() {
        match helper_save_device_file(filename, file_value, backend_communicator.clone()) {
            Ok(filepath) => filepaths.push(filepath),
            Err(err_str) => return Err(err_str),
        }
    }

    // Copy files to Docker
    match start_docker_container_for_copying_data(backend_communicator.clone()) {
        Ok(_) => {}
        Err(err_str) => return Err(err_str),
    }

    for filepath in filepaths {
        match copy_data_to_running_container(filepath.clone(), backend_communicator.clone()) {
            Ok(_) => {
                let ok_message = format!("Copied file {} to edge-device-data", filepath.clone());
                log_and_emit(ok_message.clone(), backend_communicator.clone());
            }
            Err(err_str) => return Err(err_str),
        }
    }

    // Cleanup

    match remove_temporary_container(backend_communicator.clone()) {
        Ok(_) => {}
        Err(err_str) => {
            return Err(err_str);
        }
    }

    for (filename, _) in fields_hashmap {
        match helper_delete_device_file(filename, backend_communicator.clone()) {
            Ok(_) => {}
            Err(err_str) => return Err(err_str),
        }
    }

    // Success!

    let success_message = format!("Successfully added device data.");
    log_and_emit(success_message.clone(), backend_communicator.clone());

    let url_safe_device_code = general_purpose::URL_SAFE_NO_PAD.encode(address.as_bytes());
    let next_step = format!(
        "Please assign your device token at https://wallet.xe.network/staking. Your device token is : {}",
        url_safe_device_code
    );
    return Ok(next_step);
}

/// Save field as file with same name, returns filepath if successful
fn helper_save_device_file(
    file_name: String,
    field_value: String,
    backend_communicator: BackendCommunicator,
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
                Ok(_) => return Ok(file_path),
                Err(_) => return Err(format!("Unable to write file: {}", file_path)),
            }
        }
        Err(_) => return Err(format!("Unable to open file {}", file_path.clone())),
    }
}

/// Delete device file
fn helper_delete_device_file(
    file_name: String,
    backend_communicator: BackendCommunicator,
) -> Result<String, String> {
    let file_path = format!("{}{}", backend_communicator.data_dir, file_name);
    match fs::remove_file(file_path.clone()) {
        Ok(_) => {
            let ok_msg = format!("File {} was successfully cleaned up.", file_path);
            log_and_emit(ok_msg.clone(), backend_communicator.clone());
            return Ok(ok_msg);
        }
        Err(_) => return Err(format!("Unable to delete file {}", file_path.clone())),
    }
}
