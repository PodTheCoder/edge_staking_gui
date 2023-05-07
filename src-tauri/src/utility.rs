use chrono::{DateTime, Utc};
use futures::StreamExt;
use reqwest::Client;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

use crate::BackendCommunicator;

#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

/// Download a file from a url to a local download path
pub async fn download_file(
    download_url: String,
    download_path_str: String,
    backend_communicator: &BackendCommunicator,
) -> Result<(), String> {
    let download_path = PathBuf::new();
    let download_path = download_path.join(download_path_str.clone());

    log_and_emit(
        format!(
            "Preparing to download {} to {}.",
            download_url.clone(),
            download_path_str.clone()
        ),
        backend_communicator,
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
                    let error_message = "Unable to read filesize from valid response.".to_string();
                    log_and_emit(error_message.clone(), backend_communicator);
                    return Err(error_message);
                }
            };
            log_and_emit(
                format!("Download Headers: {:#?}", valid_response.headers()),
                backend_communicator,
            );
        }
        Err(_) => {
            log_and_emit(
                "Unable to read filesize from response.".to_string(),
                backend_communicator,
            );
            return Err("Unable to read filesize from response.".to_string());
        }
    }

    log_and_emit(
        format!(
            "Opening file: {}. Program may be temporarily unresponsive while writing.",
            download_path_str.clone()
        ),
        backend_communicator,
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
        let stream_chunk = match stream_content {
            Ok(ok_chunk) => ok_chunk,
            Err(_) => {
                let error_message = format!("Unable to read chunk {}", chunk_counter);
                log_and_emit(error_message.clone(), backend_communicator);
                return Err(error_message);
            }
        };
        match file.write_all(&stream_chunk) {
            Ok(_) => {}
            Err(_) => {
                let error_message = "Unable to write chunk.".to_string();
                log_and_emit(error_message.clone(), backend_communicator);
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
                backend_communicator,
            )
        }
        chunk_counter += 1;
    }

    log_and_emit(
        format!("File downloaded: {} !", download_url),
        backend_communicator,
    );

    Ok(())
}

/// Convenience function, logs a message and emits an event with the message as payload.
pub fn log_and_emit(message: String, backend_communicator: &BackendCommunicator) {
    match log_message(message.clone(), backend_communicator) {
        Ok(_) => {}
        Err(err_str) => println!("{}", err_str),
    }
    emit_event(message, backend_communicator);
    {}
}

/// Log a message locally
pub fn log_message(
    message: String,
    backend_communicator: &BackendCommunicator,
) -> Result<String, String> {
    let log_name = "log.txt".to_string();
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
            complete_log_string.push('\n');

            match valid_file.write(complete_log_string.as_bytes()) {
                Ok(_) => {
                    println!("Logged: {}", complete_log_string.clone());
                    Ok(complete_log_string)
                }
                Err(_) => Err("Unable to write log to file.".to_string()),
            }
        }
        Err(_) => Err(format!("Unable to open log file {}", log_path_str)),
    }
}

/// Emit an event to the main window
pub fn emit_event(message: String, backend_communicator: &BackendCommunicator) {
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
    {}
}
