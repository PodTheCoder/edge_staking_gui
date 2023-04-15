// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//

mod check_requirements;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
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
//TODO: Add boolean for every phase. (eg. system requirements has been checked.)
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
