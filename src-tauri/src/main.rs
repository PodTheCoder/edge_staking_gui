// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Window;

mod check_requirements;
mod control_edge_cli;
mod utility_events;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
// Tauri API allowlist has no impact on Rust std functions.
// Example Frontend: https://github.com/tauri-apps/tauri/blob/dev/examples/api/src/views/Welcome.svelte
// TODO: Pass appDir and andappLocalDataDir to Rust, writeable folders without need for admin permissions. https://tauri.app/v1/api/js/path#appconfigdir
// Will need to place the snippet below in the tauri.conf.json
//   "allowlist": {
//     "path": {
//       "all": "true"
//      }

const STATUSLISTENER: &str = "program_status_listener";

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn check_requirements(datadir: &str) -> String {
    match check_requirements::main(String::from(datadir)) {
        Ok(res) => return res,
        Err(res) => return res,
    }
}

#[tauri::command]
fn install_edge_cli(datadir: &str) -> String {
    return check_requirements::check_edge::get_edge_cli(String::from(datadir));
}

#[tauri::command]
fn get_edge_cli_download_url() -> String {
    return check_requirements::check_edge::get_edge_cli_download_url();
}

#[tauri::command]
fn device_start(datadir: &str) -> String {
    return control_edge_cli::device_start(String::from(datadir));
}

#[tauri::command]
fn device_stop(datadir: &str) -> String {
    return control_edge_cli::device_stop(String::from(datadir));
}

#[tauri::command]
fn device_info(datadir: &str) -> String {
    return control_edge_cli::device_info(String::from(datadir));
}

#[tauri::command]
fn emit_from_backend(window: Window) {
    println!("function invoked");
    utility_events::test_emit(&window, STATUSLISTENER);
    return;
}
//TODO: Add persistent boolean if initialization is completed.

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            check_requirements,
            install_edge_cli,
            get_edge_cli_download_url,
            device_start,
            device_stop,
            device_info,
            emit_from_backend
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
