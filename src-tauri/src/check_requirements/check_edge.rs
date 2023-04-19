use crate::BackendCommunicator;
use isahc::prelude::*;
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::path::PathBuf;
use std::{fs, io};

use crate::check_requirements::check_specifications::{get_os_info, get_processor_info};
use crate::check_requirements::pretty_check_string;

/// Create an edge url based on url components
fn create_edge_url(
    net: String,
    os: String,
    arch: String,
    version: String,
    filename: String,
) -> String {
    let mut edge_url = String::from("https://files.edge.network/cli");

    fn add_url_component(mut edge_url: String, comp: String) -> String {
        edge_url.push_str(&String::from("/"));
        edge_url.push_str(&comp);
        return edge_url;
    }

    edge_url = add_url_component(edge_url.clone(), net.clone()); // eg. mainnet
    edge_url = add_url_component(edge_url.clone(), os.clone()); // eg. windows
    edge_url = add_url_component(edge_url.clone(), arch.clone()); // eg. arm64
    edge_url = add_url_component(edge_url.clone(), version.clone()); // eg. latest
    edge_url = add_url_component(edge_url.clone(), filename.clone()); // eg. checksum

    return edge_url;
}

// TODO: Add base_download_path arg, to download without needing admin permission
// https://github.com/tauri-apps/tauri/blob/dev/examples/api/src/views/Welcome.svelte
// https://github.com/tauri-apps/tauri/blob/dev/examples/api/src/views/FileSystem.svelte
// API documentation, app data dir does not need admin permission: https://tauri.app/v1/api/js/path#appdatadir
/// Downloads checksum of latest edge binary for system
fn get_edge_cli_checksum(backend_communicator: BackendCommunicator) -> Result<String, String> {
    let backend_communicator = backend_communicator.clone();
    let checksum_url = get_edge_cli_checksum_url(backend_communicator.clone());

    let filename = String::from("checksum");
    let filepath = format!("{}{}", backend_communicator.data_dir.clone(), filename);

    match download_file(checksum_url.clone(), filepath.clone()) {
        Ok(_) => {}
        Err(err) => {
            let error_message = err;
            return Err(error_message);
        }
    }

    println!("Checksum download Url: {}", checksum_url);

    let checksum: String;
    match fs::read_to_string(filepath) {
        Ok(checksum_str) => checksum = checksum_str, // Checksum is SHA256
        Err(err) => {
            let error_message = err.to_string();
            return Err(error_message);
        }
    }

    return Ok(checksum);
}

// Create URL based on user's system to filename.
fn get_edge_file_url(filename: String, backend_communicator: BackendCommunicator) -> String {
    let net = String::from("mainnet");
    let os_info = get_os_info(backend_communicator.clone());
    let os = os_info.cli_os_name;
    let processor_info = get_processor_info(backend_communicator.clone());
    let arch = processor_info.cli_architecture_name;
    let version = String::from("latest");
    let edge_url = create_edge_url(net, os, arch, version, filename);

    return edge_url;
}

fn get_edge_cli_checksum_url(backend_communicator: BackendCommunicator) -> String {
    let filename = String::from("checksum");
    let checksum_url = get_edge_file_url(filename, backend_communicator);
    return checksum_url;
}
/// Creates URL to Edge CLI based on user's system. eg. windows user will get link to windows binary.
pub fn get_edge_cli_download_url(backend_communicator: BackendCommunicator) -> String {
    let filename = String::from("edge.exe");
    let edge_cli_url = get_edge_file_url(filename, backend_communicator);

    return edge_cli_url;
}

/// Checks whether the Edge CLI was downloaded correctly by checksumming.
pub fn is_edge_correctly_downloaded(
    backend_communicator: BackendCommunicator,
) -> Result<String, String> {
    // Send a GET request and wait for the response headers.
    // Must be `mut` so we can read the response body.

    let filename = String::from("edge.exe");
    let filepath = format!("{}{}", backend_communicator.data_dir.clone(), filename);

    let edge_cli_path = Path::new(&filepath);

    if edge_cli_path.exists() {
        let calculated_checksum;
        match get_edge_cli_checksum(backend_communicator) {
            Ok(ok_checksum_str) => calculated_checksum = ok_checksum_str,
            Err(err_checksum_str) => {
                calculated_checksum = String::from(format!(
                    "Edge CLI Checksum not found. Err = {}",
                    err_checksum_str
                ))
            }
        }

        let hash_string: String;
        match hash_file(edge_cli_path) {
            Ok(hash_str) => hash_string = hash_str,
            Err(err_str) => {
                let error_message = String::from(err_str);
                return Err(error_message);
            }
        }

        if calculated_checksum.eq(&hash_string) {
            println!("Checksum matches: {}.", calculated_checksum);
            let success_message = String::from("Latest Edge CLI installed for your system.");
            return Ok(success_message);
        } else {
            let checksums_do_not_match_err = String::from(format!(
                "Edge CLI not correctly downloaded. Download checksum: {} . Calculated checksum: {} .",
                calculated_checksum, hash_string
            ));

            return Err(checksums_do_not_match_err);
        }
    } else {
        let cli_not_downloaded = String::from("Edge CLI not yet installed via GUI.");
        return Err(cli_not_downloaded);
    }
}

fn hash_file(file_path: &Path) -> Result<String, String> {
    let mut file_binary: File;
    match fs::File::open(file_path) {
        Ok(valid_path) => file_binary = valid_path,
        Err(invalid_path) => {
            let error_message = String::from(format!(
                "Path no longer exists after opening. Invalid Path = {}",
                invalid_path
            ));
            return Err(error_message);
        }
    }
    let mut hasher = Sha256::new();

    match io::copy(&mut file_binary, &mut hasher) {
        Ok(_) => {}
        Err(_) => {}
    }
    let hash = hasher.finalize();

    let hash_string = format!("{:x}", hash);
    return Ok(hash_string);
}

// TODO: Add base_download_path arg, to download without needing admin permission
// https://github.com/tauri-apps/tauri/blob/dev/examples/api/src/views/Welcome.svelte
// https://github.com/tauri-apps/tauri/blob/dev/examples/api/src/views/FileSystem.svelte
// API documentation, app data dir does not need admin permission: https://tauri.app/v1/api/js/path#appdatadir
/// Download the fitting Edge CLI based on user's system.
pub(crate) fn get_edge_cli(backend_communicator: BackendCommunicator) -> String {
    let edge_binary_filename = String::from("edge.exe");
    let edge_binary_filepath = format!(
        "{}{}",
        backend_communicator.clone().data_dir.clone(),
        edge_binary_filename
    );
    println!(
        "Hello, {:?}! You've been greeted from Rust!",
        edge_binary_filepath
    );

    match is_edge_correctly_downloaded(backend_communicator.clone()) {
        Ok(_) => {
            let result_string = pretty_check_string::pretty_ok_str(&String::from(
                "Latest Edge CLI is already correctly installed.",
            ));
            return result_string;
        }
        Err(_) => {}
    }

    let cli_download_url = get_edge_cli_download_url(backend_communicator.clone());
    println!("Download Url: {}", cli_download_url);

    match download_file(cli_download_url, edge_binary_filepath.clone()) {
        Ok(_) => {}
        Err(err) => {
            let error_message = String::from(err);
            return error_message;
        }
    }

    match is_edge_correctly_downloaded(backend_communicator) {
        Ok(_) => {
            let result_string = pretty_check_string::pretty_ok_str(&String::from(
                "Latest Edge CLI downloaded & correctly installed.",
            ));
            return result_string;
        }
        Err(_) => {
            let errormessage = format!("File was not downloaded correctly. Please remove the edge.exe file from your directory.");
            fs::remove_file(edge_binary_filepath.clone())
                .expect("Could not remove downloaded file.");
            return errormessage;
        }
    }
}

// TODO: Rework to use Tauri API? https://tauri.app/v1/api/js/http/ . Add progress bar in GUI.
/// Download a file from a url to a local download path
fn download_file(download_url: String, download_path_str: String) -> Result<(), String> {
    let download_path = PathBuf::new();
    let download_path = download_path.join(download_path_str);
    let mut response;
    match isahc::get(download_url) {
        Ok(successful_response) => response = successful_response,
        Err(error_response) => {
            let error_message = format!("Error while downloading CLI. Error = {}", error_response);
            return Err(error_message);
        }
    }
    println!("Status: {}", response.status());
    println!("Headers: {:#?}", response.headers());
    let edge_cli_bytes;
    match response.bytes() {
        Ok(converted_byte_vector) => edge_cli_bytes = converted_byte_vector,
        Err(error_response) => {
            let error_message = format!(
                "Error while converting downloaded file to bytes. Error: {}",
                error_response
            );
            return Err(error_message);
        }
    }
    println!("Writing file. Program may lag while downloading.");
    let mut file;
    match File::create(download_path) {
        Ok(ok_file) => file = ok_file,
        Err(err) => {
            let error_message = format!("Error creating File based on filename. Potential solution: Run the program as administrator. Error = {}", err);
            return Err(error_message);
        }
    }

    match file.write_all(&edge_cli_bytes) {
        Ok(_) => {}
        Err(err) => {
            let error_message = format!("Unable to write file. Error = {}", err);
            return Err(error_message);
        }
    }
    return Ok(());
}
