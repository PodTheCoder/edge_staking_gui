use chrono::{DateTime, Utc};
use std::fs::OpenOptions;
use std::io::Write;

use crate::BackendCommunicator;

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
