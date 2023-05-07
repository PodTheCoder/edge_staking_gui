use crate::utility::download_file;
use crate::utility::log_and_emit;
use crate::BackendCommunicator;
use sha2::{Digest, Sha256};
use std::fs::File;
use std::path::Path;
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
        edge_url
    }

    edge_url = add_url_component(edge_url.clone(), net); // eg. mainnet
    edge_url = add_url_component(edge_url.clone(), os); // eg. windows
    edge_url = add_url_component(edge_url.clone(), arch); // eg. arm64
    edge_url = add_url_component(edge_url.clone(), version); // eg. latest
    edge_url = add_url_component(edge_url.clone(), filename); // eg. checksum

    edge_url
}

/// Downloads checksum of latest edge binary for system
async fn get_edge_cli_checksum(
    backend_communicator: &BackendCommunicator,
) -> Result<String, String> {
    let backend_communicator = backend_communicator;
    let checksum_url = get_edge_cli_checksum_url(backend_communicator);

    let filename = String::from("checksum");
    let filepath = format!("{}{}", backend_communicator.data_dir.clone(), filename);

    log_and_emit(
        format!("Downloading checksum from URL: {}", checksum_url.clone()),
        backend_communicator,
    );

    let download_file_future =
        download_file(checksum_url.clone(), filepath.clone(), backend_communicator).await;
    match download_file_future {
        Ok(_) => {}
        Err(err) => {
            let error_message = err;
            return Err(error_message);
        }
    }

    let checksum: String = match fs::read_to_string(filepath) {
        Ok(checksum_str) => checksum_str, // Checksum is SHA256
        Err(err) => {
            let error_message = err.to_string();
            return Err(error_message);
        }
    };

    Ok(checksum)
}

// Create URL based on user's system to filename.
fn get_edge_file_url(filename: String, backend_communicator: &BackendCommunicator) -> String {
    let net = String::from("mainnet");
    let os_info = get_os_info(backend_communicator);
    let os = os_info.cli_os_name;
    let processor_info = get_processor_info(backend_communicator);
    let arch = processor_info.cli_architecture_name;
    let version = String::from("latest");

    create_edge_url(net, os, arch, version, filename)
}

/// Returns the checksum url
fn get_edge_cli_checksum_url(backend_communicator: &BackendCommunicator) -> String {
    let filename = String::from("checksum");

    get_edge_file_url(filename, backend_communicator)
}
/// Creates URL to Edge CLI based on user's system. eg. windows user will get link to windows binary.
pub fn get_edge_cli_download_url_from_frontend(
    backend_communicator: &BackendCommunicator,
) -> String {
    let filename = String::from("edge.exe");

    get_edge_file_url(filename, backend_communicator)
}

/// Checks whether the Edge CLI was downloaded correctly by checksumming.
pub async fn is_edge_correctly_downloaded(
    backend_communicator: &BackendCommunicator,
) -> Result<String, String> {
    // Send a GET request and wait for the response headers.
    // Must be `mut` so we can read the response body.

    let filename = String::from("edge.exe");
    let filepath = format!("{}{}", backend_communicator.data_dir.clone(), filename);

    let edge_cli_path = Path::new(&filepath);

    if edge_cli_path.exists() {
        let get_edge_cli_checksum_future = get_edge_cli_checksum(backend_communicator).await;
        let calculated_checksum = match get_edge_cli_checksum_future {
            Ok(ok_checksum_str) => ok_checksum_str,
            Err(err_checksum_str) => {
                format!("Edge CLI Checksum not found. Err = {}", err_checksum_str)
            }
        };

        let hash_string: String = match hash_file(edge_cli_path, backend_communicator) {
            Ok(hash_str) => hash_str,
            Err(err_str) => {
                let error_message = err_str;
                return Err(error_message);
            }
        };

        if calculated_checksum.eq(&hash_string) {
            log_and_emit("CLI installed correctly!".to_string(), backend_communicator);
            let success_message = String::from("Latest Edge CLI installed for your system.");
            Ok(success_message)
        } else {
            let checksums_do_not_match_err = format!(
                "Edge CLI not correctly downloaded. Download checksum: {} . Calculated checksum: {} .",
                calculated_checksum, hash_string
            );

            Err(checksums_do_not_match_err)
        }
    } else {
        let cli_not_downloaded = String::from("Edge CLI not yet installed via GUI.");
        Err(cli_not_downloaded)
    }
}

/// Hash file with SHA256
fn hash_file(
    file_path: &Path,
    backend_communicator: &BackendCommunicator,
) -> Result<String, String> {
    let mut file_binary: File;
    match fs::File::open(file_path) {
        Ok(valid_path) => file_binary = valid_path,
        Err(invalid_path) => {
            let error_message = format!(
                "Path no longer exists after opening. Invalid Path = {}",
                invalid_path
            );
            return Err(error_message);
        }
    }
    let mut hasher = Sha256::new();

    match io::copy(&mut file_binary, &mut hasher) {
        Ok(_) => (),
        Err(err_str) => {
            let err_msg = format!("Unable to copy binary to hasher. Err {}", err_str);
            log_and_emit(err_msg.clone(), backend_communicator);
            return Err(err_msg);
        }
    }
    let hash = hasher.finalize();

    let hash_string = format!("{:x}", hash);
    Ok(hash_string)
}

/// Download the fitting Edge CLI based on user's system.
/// Returns true if latest binary installed.
pub(crate) async fn get_edge_cli_binary(backend_communicator: &BackendCommunicator) -> bool {
    let edge_binary_filename = String::from("edge.exe");
    let edge_binary_filepath = format!(
        "{}{}",
        backend_communicator.data_dir.clone(),
        edge_binary_filename
    );

    match is_edge_correctly_downloaded(backend_communicator).await {
        Ok(_) => {
            let ok_msg = pretty_check_string::pretty_ok_str(
                &String::from("Latest Edge CLI is already correctly installed."),
                false,
            );
            log_and_emit(ok_msg, backend_communicator);
            return true;
        }
        Err(_) => {}
    }

    let cli_download_url = get_edge_cli_download_url_from_frontend(backend_communicator);
    log_and_emit(
        format!("Download Url: {}", cli_download_url),
        backend_communicator,
    );

    let download_file_future = download_file(
        cli_download_url,
        edge_binary_filepath.clone(),
        backend_communicator,
    )
    .await;

    match download_file_future {
        Ok(_) => {}
        Err(err) => {
            let err_msg = err;
            log_and_emit(err_msg, backend_communicator);
            return false;
        }
    }

    let is_edge_correctly_downloaded_future_post_download_cli =
        is_edge_correctly_downloaded(backend_communicator).await;
    match is_edge_correctly_downloaded_future_post_download_cli {
        Ok(_) => {
            let ok_msg = pretty_check_string::pretty_ok_str(
                &String::from("Latest Edge CLI downloaded & correctly installed."),
                false,
            );
            log_and_emit(ok_msg, backend_communicator);
            true
        }
        Err(_) => {
            let err_msg = "File was not downloaded correctly. Attempting to remove automatically."
                .to_string();
            log_and_emit(err_msg, backend_communicator);
            match fs::remove_file(edge_binary_filepath.clone()) {
                Ok(_) => {
                    let ok_msg = "Removed incorrect downloaded file automatically.".to_string();
                    log_and_emit(ok_msg, backend_communicator);
                }
                Err(_) => {
                    let err_msg =
                        "Failed to remove incorrect downloaded file. Try rerunning.".to_string();
                    log_and_emit(err_msg, backend_communicator);
                }
            }
            false
        }
    }
}
