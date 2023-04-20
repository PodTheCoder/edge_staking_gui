use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;

use crate::BackendCommunicator;

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct ConfigStruct {
    pub initialized: bool, // Has the device be initialized? Set to true when node launched successfully.
    pub index_api_last_contacted: String, // When did the index api last get contacted? datetime as rfc2822 string
}

#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

// TODO: Add more logging.
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
            &backend_communicator.event_listener.clone(),
            message.clone(),
        )
        .unwrap();
    println!(
        "Sent event on listener: {},  payload: {}",
        &backend_communicator.event_listener, message
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
    // Create default config if not yet exists
    if !config_path.exists() {
        match create_default_config(backend_communicator.clone()) {
            Ok(_) => {}
            Err(err_string) => return Err(err_string),
        }
    }

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
