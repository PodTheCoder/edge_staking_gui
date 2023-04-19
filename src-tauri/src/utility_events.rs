use chrono::{DateTime, Utc};
use std::fs::OpenOptions;
use std::io::Write;
use tauri::Window;

#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

pub fn log_event(datadir: String, log_message: String) -> Result<String, String> {
    let log_name = format!("log.txt");
    let log_path_str = format!("{}{}", datadir, log_name);

    let dt: DateTime<Utc> = Utc::now();

    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_path_str.clone());
    match file {
        Ok(ok_file) => {
            let mut valid_file = ok_file;
            let mut complete_log_string = dt.format("%d %B %Y %H:%M:%S%.3f %Z ").to_string();
            complete_log_string.push_str(&log_message);
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

pub fn emit_event(window: &Window, event_listener_name: &str, payload: String) {
    // window.center();
    window.emit(&event_listener_name, payload.clone()).unwrap();
    println!(
        "Sent event on listener: {},  payload: {}",
        event_listener_name, payload
    );
    return {};
}
