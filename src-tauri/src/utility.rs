use chrono::{DateTime, Utc};
use futures::StreamExt;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::BackendCommunicator;

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct ConfigStruct {
    pub initialized: bool, // Has the device be initialized? Set to true when node launched successfully.
    pub index_api_last_contacted: String, // When did the index api last get contacted? datetime as rfc2822 string
    pub network: String, // On which Edge network is the device, mainnet or testnet?
    pub address: String, // What is the device XE address?
    pub private_key: String, // What is the private key of the XE address?
    pub public_key: String, // What is the public key of the XE address?
}

#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

// TODO: Add caching via config file. Lookup timestamp if exist, check if beyond caching time arg, save timestamp when file fully downloaded.
/// Download a file from a url to a local download path
pub async fn download_file(
    download_url: String,
    download_path_str: String,
    backend_communicator: BackendCommunicator,
) -> Result<(), String> {
    let download_path = PathBuf::new();
    let download_path = download_path.join(download_path_str.clone());

    log_and_emit(
        format!(
            "Preparing to download {} to {}.",
            download_url.clone(),
            download_path_str.clone()
        ),
        backend_communicator.clone(),
    );

    let client = Client::new();
    let response = client
        .get(download_url.clone())
        .send()
        .await
        .or(Err(format!(
            "Error while downloading file {} to {}.",
            download_url.clone(),
            download_path_str.clone()
        )));

    let filesize;
    let valid_response;
    match response {
        Ok(ok_response) => {
            valid_response = ok_response;
            match valid_response.content_length() {
                Some(ok_filesize) => filesize = ok_filesize,
                None => {
                    let error_message = format!("Unable to read filesize from valid response.");
                    log_and_emit(error_message.clone(), backend_communicator);
                    return Err(error_message);
                }
            };
            log_and_emit(
                format!("Download Headers: {:#?}", valid_response.headers()),
                backend_communicator.clone(),
            );
        }
        Err(_) => {
            log_and_emit(
                format!("Unable to read filesize from response."),
                backend_communicator,
            );
            return Err(format!("Unable to read filesize from response."));
        }
    }

    log_and_emit(
        format!(
            "Opening file: {}. Program may be temporarily unresponsive while writing.",
            download_path_str.clone()
        ),
        backend_communicator.clone(),
    );
    let mut file;
    match File::create(download_path) {
        Ok(ok_file) => file = ok_file,
        Err(err) => {
            let error_message = format!("Error creating File based on filename. Potential solution: Run the program as administrator. Error = {}", err);
            return Err(error_message);
        }
    }

    let mut bytes_downloaded: u64 = 0;
    let mut download_stream = valid_response.bytes_stream();
    let mut chunk_counter: u64 = 0;
    while let Some(stream_content) = download_stream.next().await {
        let stream_chunk;
        match stream_content {
            Ok(ok_chunk) => stream_chunk = ok_chunk,
            Err(_) => {
                let error_message = format!("Unable to read chunk {}", chunk_counter);
                log_and_emit(error_message.clone(), backend_communicator.clone());
                return Err(error_message);
            }
        }
        match file.write_all(&stream_chunk) {
            Ok(_) => {}
            Err(_) => {
                let error_message = format!("Unable to write chunk.");
                log_and_emit(error_message.clone(), backend_communicator.clone());
                return Err(error_message);
            }
        }
        let bytes_added = stream_chunk.len() as u64;
        bytes_downloaded += bytes_added;
        if chunk_counter % 250 == 0 {
            log_and_emit(
                format!(
                    "Downloading file: {} to {}. Downloaded {} / {} bytes",
                    download_url.clone(),
                    download_path_str,
                    bytes_downloaded,
                    filesize
                ),
                backend_communicator.clone(),
            )
        }
        chunk_counter += 1;
    }

    log_and_emit(
        format!("File downloaded: {} !", download_url),
        backend_communicator,
    );

    return Ok(());
}

/// Convenience function, logs a message and emits an event with the message as payload.
pub fn log_and_emit(message: String, backend_communicator: BackendCommunicator) {
    match log_message(message.clone(), backend_communicator.clone()) {
        Ok(_) => {}
        Err(err_str) => println!("{}", err_str),
    }
    emit_event(message.clone(), backend_communicator);
    return {};
}

pub fn log_message(
    message: String,
    backend_communicator: BackendCommunicator,
) -> Result<String, String> {
    let log_name = format!("log.txt");
    let log_path_str = format!("{}{}", backend_communicator.data_dir.clone(), log_name);

    let dt: DateTime<Utc> = Utc::now();

    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_path_str.clone());
    match file {
        Ok(ok_file) => {
            let mut valid_file = ok_file;
            let mut complete_log_string = dt.format("%d %B %Y %H:%M:%S%.3f %Z ").to_string();
            complete_log_string.push_str(&message);
            complete_log_string.push_str(&format!("\n"));

            match valid_file.write(complete_log_string.as_bytes()) {
                Ok(_) => {
                    return {
                        println!("Logged: {}", complete_log_string.clone());
                        Ok(complete_log_string)
                    }
                }
                Err(_) => return Err(format!("Unable to write log to file.")),
            }
        }
        Err(_) => return Err(format!("Unable to open log file {}", log_path_str)),
    }
}

pub fn emit_event(message: String, backend_communicator: BackendCommunicator) {
    // window.center();
    backend_communicator
        .front_end_window
        .emit(
            &backend_communicator.status_listener.clone(),
            message.clone(),
        )
        .unwrap();
    println!(
        "Sent event on listener: {},  payload: {}",
        &backend_communicator.status_listener, message
    );
    return {};
}

pub fn create_default_config(backend_communicator: BackendCommunicator) -> Result<(), String> {
    let filepath = format!(
        "{}{}",
        backend_communicator.data_dir.clone(),
        format!("config.txt")
    );
    let config_path = Path::new(&filepath);

    let dt_not_yet_downloaded;
    match DateTime::parse_from_str("1970 Jan 19 14:04:0.000 +0000", "%Y %b %d %H:%M:%S%.3f %z") {
        Ok(the_internet_belongs_to_everyone) => {
            dt_not_yet_downloaded = the_internet_belongs_to_everyone
        }
        Err(_) => {
            let error_message = format!(
                "Unable to parse the beginning of greatness. The Edge genesis date string."
            );
            log_and_emit(error_message.clone(), backend_communicator.clone());
            return Err(error_message);
        }
    }
    let default_config = ConfigStruct {
        initialized: false,
        index_api_last_contacted: dt_not_yet_downloaded.to_rfc2822(),
        address: format!("Unset"),
        network: format!("Unset"),
        private_key: format!("Unset"),
        public_key: format!("Unset"),
    };
    match confy::store_path(config_path, default_config) {
        Ok(_) => {
            log_and_emit(
                format!("Created initial config file at location: {}", filepath),
                backend_communicator.clone(),
            );
            log_and_emit(
                format!("Awaiting initial command..."),
                backend_communicator.clone(),
            );
            return Ok({});
        }
        Err(_) => {
            return Err(format!(
                "Unable to store default config at path {}",
                config_path.display()
            ))
        }
    }
}

/// Load config file
pub fn load_config(backend_communicator: BackendCommunicator) -> Result<ConfigStruct, String> {
    let filepath = format!(
        "{}{}",
        backend_communicator.data_dir.clone(),
        format!("config.txt")
    );
    let config_path = Path::new(&filepath);

    match create_config_if_not_exists(backend_communicator.clone()) {
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
                backend_communicator.clone(),
            );
            log_and_emit(
                format!(
                    "Attempting to restore corrupted config to default state. Path: {}",
                    config_path.display()
                ),
                backend_communicator.clone(),
            );
            log_and_emit(
                format!("Removing corrupted file at path {}", config_path.display()),
                backend_communicator.clone(),
            );
            match fs::remove_file(config_path) {
                Ok(_) => {
                    log_and_emit(
                        format!(
                            "Removed corrupted config at path : {}",
                            config_path.display()
                        ),
                        backend_communicator.clone(),
                    );

                    match create_default_config(backend_communicator.clone()) {
                        Ok(_) => {}
                        Err(err) => return Err(err),
                    }
                    let error_message =
                        format!("Could not load config, but restored to default value.");

                    log_and_emit(error_message.clone(), backend_communicator.clone());
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

pub fn create_config_if_not_exists(
    backend_communicator: BackendCommunicator,
) -> Result<String, String> {
    let filepath = format!(
        "{}{}",
        backend_communicator.data_dir.clone(),
        format!("config.txt")
    );
    let config_path = Path::new(&filepath);
    if !config_path.exists() {
        match create_default_config(backend_communicator.clone()) {
            Ok(_) => {
                let ok_message = format!("Created default config.");
                log_and_emit(ok_message.clone(), backend_communicator.clone());
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
    log_and_emit(ok_message.clone(), backend_communicator.clone());
    Ok(ok_message)
}

pub fn config_set_device_data(
    network: &String,
    address: &String,
    private_key: &String,
    public_key: &String,
    backend_communicator: BackendCommunicator,
) -> Result<String, String> {
    let filepath = format!(
        "{}{}",
        backend_communicator.data_dir.clone(),
        format!("config.txt")
    );
    let config_path = Path::new(&filepath);

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

/// Write initialized value to config
pub fn config_set_device_initialization_status(
    device_initialization_status: bool,
    backend_communicator: BackendCommunicator,
) -> Result<(), std::string::String> {
    let filepath = format!(
        "{}{}",
        backend_communicator.data_dir.clone(),
        format!("config.txt")
    );
    let config_path = Path::new(&filepath);

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

/// Returns Initialization status codes
pub fn load_initialization_status(backend_communicator: BackendCommunicator) -> u64 {
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
