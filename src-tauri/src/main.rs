// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// 

mod check_cpu;
mod check_requirements;
mod pretty_string;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn initialize_prereqs() -> String {
    // TODO: Make each string into an async event
    let mut test_string = String::from("");

    let mut cpu_arch = String::from("");
    match check_requirements::check_processor_architecture(){
        Ok(found_cpu_arch) => {
            let ok_cpu = pretty_string::ok_str(&found_cpu_arch);
            cpu_arch.push_str(&ok_cpu)}
        Err(not_found_cpu_arch) => {
            let err_cpu = pretty_string::err_str(&not_found_cpu_arch);
            cpu_arch.push_str(&err_cpu)}
    }

    let mut os_type= String::from("");
    match check_requirements::check_os_type(){
        Ok(found_os) => {
            let ok_os = pretty_string::ok_str(&found_os);
            os_type.push_str(&ok_os)
        }
        Err(err_os) => {
            let err_os = pretty_string::err_str(&err_os);
            os_type.push_str(&err_os)
        }
    }

    // Check System Infrastructure
    test_string.push_str(&format!("OS: {} | ", &os_type));
    test_string.push_str(&format!("Processor: {} | ", &cpu_arch));
    //TODO: Docker check. 
    test_string.push_str("Did you have docker installed & running? \n");

    println!("Checked all requirements.");

    return test_string
}

//TODO: Add boolean for every phase. (eg. system requirements has been checked.)
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, initialize_prereqs])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
