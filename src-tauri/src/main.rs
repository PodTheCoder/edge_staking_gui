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
fn initialize_prereqs() -> String {
    return check_requirements::main()
}

//TODO: Add boolean for every phase. (eg. system requirements has been checked.)
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, initialize_prereqs])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
