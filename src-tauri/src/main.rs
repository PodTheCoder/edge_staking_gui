// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod check_requirements;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
// Tauri API allowlist has no impact on Rust std functions.
// Example Frontend: https://github.com/tauri-apps/tauri/blob/dev/examples/api/src/views/Welcome.svelte
// TODO: Pass appDir and andappLocalDataDir to Rust, writeable folders without need for admin permissions. https://tauri.app/v1/api/js/path#appconfigdir
// Will need to place the snippet below in the tauri.conf.json
//   "allowlist": {
//     "path": {
//       "all": "true"
//      }

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn check_requirements() -> String {
    return check_requirements::main();
}

#[tauri::command]
fn install_edge_cli() -> String {
    return check_requirements::check_edge::get_edge_cli();
}

#[tauri::command]
fn get_edge_cli_download_url() -> String {
    return check_requirements::check_edge::get_edge_cli_download_url();
}
//TODO: Add persistent boolean if initialization is completed.
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            check_requirements,
            install_edge_cli,
            get_edge_cli_download_url
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
