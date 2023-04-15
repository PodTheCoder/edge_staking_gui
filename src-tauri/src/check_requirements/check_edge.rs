use isahc::prelude::*;
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::{fs, io};

use crate::check_requirements::check_specifications::{get_os_info, get_processor_info};
use crate::check_requirements::pretty_check_string;

// Create url to Edge
pub fn create_edge_url(
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
pub fn get_checksum(
    net: String,
    os: String,
    arch: String,
    version: String,
) -> Result<String, isahc::Error> {
    // Send a GET request and wait for the response headers.
    // Must be `mut` so we can read the response body.

    let filename = String::from("checksum");
    let download_url = create_edge_url(net, os, arch, version, filename);
    println!("Download Url: {}", download_url);

    // let mut response =
    //     isahc::get("https://files.edge.network/cli/mainnet/linux/x64/latest/checksum")?;
    let mut response = isahc::get(download_url)?;

    // Print some basic info about the response to standard output.
    println!("Status: {}", response.status());
    println!("Headers: {:#?}", response.headers());

    // Read the response body as text into a string and print it.
    // print!("{}", response.text()?);
    let checksum = response.text()?; // Checksum is SHA256

    return Ok(checksum);
}

pub fn is_edge_correctly_downloaded(
    net: String,
    os: String,
    arch: String,
    version: String,
) -> Result<String, String> {
    // Send a GET request and wait for the response headers.
    // Must be `mut` so we can read the response body.

    let filename = String::from("edge.exe");

    let edge_cli_path = Path::new(&filename);

    // Skip download if file already exists.
    // TODO: Refactor into separate functions.
    if edge_cli_path.exists() {
        println!("Filename exists = {} ", filename);
        let calculated_checksum;
        match get_checksum(net.clone(), os.clone(), arch.clone(), version.clone()) {
            Ok(ok_checksum_str) => calculated_checksum = ok_checksum_str,
            Err(err_checksum_str) => {
                calculated_checksum =
                    String::from(format!("Checksum not found. Err = {}", err_checksum_str))
            }
        }

        // TODO: Rework to more elegant.
        let mut edge_binary: File;
        match fs::File::open(edge_cli_path) {
            Ok(valid_path) => edge_binary = valid_path,
            Err(invalid_path) => {
                panic!(
                    "Path no longer exists after opening. Error = {}",
                    invalid_path
                )
            }
        }
        let mut hasher = Sha256::new();

        match io::copy(&mut edge_binary, &mut hasher) {
            Ok(_) => {}
            Err(_) => {}
        }
        let hash = hasher.finalize();

        let hash_string = format!("{:x}", hash);

        if calculated_checksum.eq(&hash_string) {
            println!("Checksum matches: {}.", calculated_checksum);
            let success_message = String::from("Latest Edge CLI installed for your system.");
            return Ok(success_message);
        } else {
            let checksums_do_not_match_err = String::from(format!(
                "Checksums do not match. Download checksum: {} . Calculated checksum: {} .",
                calculated_checksum, hash_string
            ));

            return Err(checksums_do_not_match_err);
        }
    } else {
        let cli_not_downloaded = String::from("Edge CLI not yet installed via GUI.");
        return Err(cli_not_downloaded);
    }
}

pub(crate) fn get_edge_cli() -> String {
    let filename = String::from("edge.exe");

    // TODO: Check Edge
    let net = String::from("mainnet");
    let os_info = get_os_info();
    let os = os_info.cli_os_name;
    let processor_info = get_processor_info();
    let arch = processor_info.cli_architecture_name;
    let version = String::from("latest");

    match is_edge_correctly_downloaded(net.clone(), os.clone(), arch.clone(), version.clone()) {
        Ok(_) => {
            let result_string = pretty_check_string::pretty_ok_str(&String::from(
                "Latest Edge CLI is already correctly installed.",
            ));
            return result_string;
        }
        Err(_) => {}
    }

    let cli_download_url = create_edge_url(
        net.clone(),
        os.clone(),
        arch.clone(),
        version.clone(),
        filename.clone(),
    );
    println!("Download Url: {}", cli_download_url);

    // let mut response =
    //     isahc::get("https://files.edge.network/cli/mainnet/linux/x64/latest/checksum")?;
    let mut response;
    match isahc::get(cli_download_url) {
        Ok(successful_response) => response = successful_response,
        Err(error_response) => {
            let error_message = format!("Error while downloading CLI. Error = {}", error_response);
            return error_message;
        }
    }

    // Print some basic info about the response to standard output.
    println!("Status: {}", response.status());
    println!("Headers: {:#?}", response.headers());

    // Read the response body as text into a string and print it.
    // print!("{}", response.text()?);
    // let checksum = response.text()?; // Checksum is SHA256

    let edge_cli_bytes;
    match response.bytes() {
        Ok(converted_byte_vector) => edge_cli_bytes = converted_byte_vector,
        Err(error_response) => {
            let error_message = format!(
                "Error while converting downloaded file to bytes. Error: {}",
                error_response
            );
            return error_message;
        }
    }
    println!("Writing file. Program may lag while downloading.");
    let mut file;
    match File::create(filename) {
        Ok(ok_file) => file = ok_file,
        Err(err) => {
            let error_message = format!("Error creating File based on filename. Error = {}", err);
            return error_message;
        }
    }
    match file.write_all(&edge_cli_bytes) {
        Ok(_) => {}
        Err(err) => {
            let error_message = format!("Unable to write file. Error = {}", err);
            return error_message;
        }
    }

    match is_edge_correctly_downloaded(net, os, arch, version) {
        Ok(_) => {
            let result_string = pretty_check_string::pretty_ok_str(&String::from(
                "Latest Edge CLI downloaded & correctly installed.",
            ));
            return result_string;
        }
        Err(_) => {
            let errormessage = format!("File was not downloaded correctly. Please remove the edge.exe file from your directory.");
            // TODO: automatically remove file.
            return errormessage;
        }
    }
}
