use std::fs::{self, OpenOptions};
use std::io::Write;

use crate::utility::config_set_device_data;
use crate::BackendCommunicator;
// Complete flow for adding a device. Can be rerun multiple times in case it fails.
pub fn add_device(
    address: String,
    private_key: String,
    public_key: String,
    backend_communicator: BackendCommunicator,
) -> Result<String, String> {
    let network = format!("mainnet");
    println!(
        "add_device function called. address {}, private_key {}, public_key {}",
        address, private_key, public_key
    );
    // TODO: Check if input is correct.

    // Save info to config file.

    match config_set_device_data(
        &network,
        &address,
        &private_key,
        &public_key,
        backend_communicator.clone(),
    ) {
        Ok(_) => {}
        Err(err) => return Err(err),
    }

    let fields = vec![
        network.clone(),
        address.clone(),
        private_key.clone(),
        public_key.clone(),
    ];
    // Convert config file to individual files in preparation of Docker
    for field in fields {
        match helper_save_device_file(backend_communicator.clone(), field) {
            Ok(_) => {}
            Err(err_str) => return Err(err_str),
        }
    }

    return Ok(format!("Ran config file conversion"));

    // Copy files to Docker
}

fn helper_save_device_file(
    backend_communicator: BackendCommunicator,
    field: String,
) -> Result<(), String> {
    let file_path = format!("{}{}", backend_communicator.data_dir, field);
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path.clone());
    match file {
        Ok(ok_file) => {
            let mut valid_file = ok_file;
            match valid_file.write(field.as_bytes()) {
                Ok(_) => return Ok({}),
                Err(_) => return Err(format!("Unable to write file: {}", file_path)),
            }
        }
        Err(_) => return Err(format!("Unable to open file {}", file_path.clone())),
    }
}

// fn helper_copy_device_file_to_running_docker_volume(
//     field: String,
//     docker_volume_name: String,
//     backend_communicator: BackendCommunicator,
// ) -> Result<_, String> {
//     let file_path = format!("{}{}", backend_communicator.data_dir, field);

//     dock
// }
