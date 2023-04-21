use serde_json::Value;
use std::{collections::HashMap, fs};

use crate::{
    utility::{download_file, log_and_emit},
    BackendCommunicator,
};

/// Convenience function, download file from Edge Index API and conver it into a hashmap
pub async fn api_json_query_to_hashmap(
    download_url: String,
    download_path_str: String,
    backend_communicator: BackendCommunicator,
) -> Result<HashMap<String, Value>, String> {
    // Download file
    match download_file(
        download_url,
        download_path_str.clone(),
        backend_communicator.clone(),
    )
    .await
    {
        Ok(_) => {}
        Err(err_str) => {
            log_and_emit(err_str.clone(), backend_communicator.clone());
            return Err(err_str);
        }
    }

    // Open downloaded file
    let download_as_str;
    match fs::read_to_string(download_path_str.clone()) {
        Ok(ok_str) => download_as_str = ok_str,
        Err(_) => {
            let error_message = format!(
                "Unable to open downloaded API file as String {}",
                download_path_str.clone()
            );
            log_and_emit(error_message.clone(), backend_communicator.clone());
            return Err(error_message);
        }
    }

    // Parse downloaded file
    match serde_json::from_str(&download_as_str) {
        Ok(ok_json) => {
            let json_content_hashmap: HashMap<String, Value> = ok_json;
            log_and_emit(
                format!("Parsed JSON content from {}", download_path_str.clone()),
                backend_communicator.clone(),
            );

            // If contains key error, assume error
            match json_content_hashmap.get(&format!("error")) {
                Some(err_value) => {
                    let err_value_str = err_value.to_string();
                    let error_message = format!(
                        "Found error in parsed JSON content: {}. Did you enter a valid input? (eg. XE address or Stake?)",
                        err_value_str
                    );
                    log_and_emit(error_message.clone(), backend_communicator.clone());
                    return Err(error_message);
                }
                None => return Ok(json_content_hashmap), // if no error key found
            }
        }
        Err(_) => {
            let error_message = format!(
                "Unable to parse JSON content after converting string downloaded from {}",
                download_path_str.clone()
            );
            log_and_emit(error_message.clone(), backend_communicator.clone());
            return Err(error_message);
        }
    }

    // Convert parsed file to Hashmap
}

/// Query Index API for session info based on node address
pub async fn get_node_session_from_api(
    node_address: String, // eg. xe_7a65d81dC21E87d593aC30DFe0AcbC2622bbdAE8
    backend_communicator: BackendCommunicator,
) -> Result<String, String> {
    let base_download_url = format!("https://index.xe.network/session/");
    let download_url = format!("{}{}", base_download_url, node_address);

    let download_path_str = format!(
        "{}{}",
        backend_communicator.data_dir,
        format!("node_session.json")
    );

    match api_json_query_to_hashmap(
        download_url,
        download_path_str,
        backend_communicator.clone(),
    )
    .await
    {
        Ok(ok_hashmap) => {
            let ok_hashmap_str = format!("{:?}", ok_hashmap);
            log_and_emit(
                format!("Node session info {}", ok_hashmap_str.clone()),
                backend_communicator.clone(),
            );
            return Ok(ok_hashmap_str);
        }
        Err(err_str) => return Err(err_str),
    }
}

/// Query Index API for snapshots info based on node address
pub async fn get_node_snapshots_from_api(
    node_address: String, // eg. xe_7a65d81dC21E87d593aC30DFe0AcbC2622bbdAE8
    backend_communicator: BackendCommunicator,
) -> Result<String, String> {
    let base_download_url = format!("https://index.xe.network/snapshots/");
    let download_url = format!("{}{}", base_download_url, node_address);

    let download_path_str = format!(
        "{}{}",
        backend_communicator.data_dir,
        format!("node_snapshot.json")
    );

    match api_json_query_to_hashmap(
        download_url,
        download_path_str,
        backend_communicator.clone(),
    )
    .await
    {
        Ok(ok_hashmap) => {
            let ok_hashmap_str = format!("{:?}", ok_hashmap);
            log_and_emit(
                format!("Node snapshots info {}", ok_hashmap_str.clone()),
                backend_communicator.clone(),
            );
            return Ok(ok_hashmap_str);
        }
        Err(err_str) => return Err(err_str),
    }
}

/// Query Index API for stake info based on stake token
pub async fn get_stake_info_from_api(
    stake_token: String, // eg. 9d51f5129e9188ba9622163f06b34e51071be224209365ad367d1300979e0b0e
    backend_communicator: BackendCommunicator,
) -> Result<String, String> {
    let base_download_url = format!("https://index.xe.network/stake/");
    let download_url = format!("{}{}", base_download_url, stake_token);

    let download_path_str = format!(
        "{}{}",
        backend_communicator.data_dir,
        format!("stake_info.json")
    );

    match api_json_query_to_hashmap(
        download_url,
        download_path_str,
        backend_communicator.clone(),
    )
    .await
    {
        Ok(ok_hashmap) => {
            let ok_hashmap_str = format!("{:?}", ok_hashmap);
            log_and_emit(
                format!("Stake info {}", ok_hashmap_str.clone()),
                backend_communicator.clone(),
            );
            return Ok(ok_hashmap_str);
        }
        Err(err_str) => return Err(err_str),
    }
}

/// Query Index API for transaction history based on xe_address
pub async fn get_transaction_history_from_api(
    xe_address: String, // eg. xe_7a65d81dC21E87d593aC30DFe0AcbC2622bbdAE8
    backend_communicator: BackendCommunicator,
) -> Result<String, String> {
    let base_download_url = format!("https://index.xe.network/transactions/");
    let download_url = format!("{}{}", base_download_url, xe_address);

    let download_path_str = format!(
        "{}{}",
        backend_communicator.data_dir,
        format!("transaction_history.json")
    );

    match api_json_query_to_hashmap(
        download_url,
        download_path_str,
        backend_communicator.clone(),
    )
    .await
    {
        Ok(ok_hashmap) => {
            let ok_hashmap_str = format!("{:?}", ok_hashmap);
            log_and_emit(
                format!("Transaction history: {}", ok_hashmap_str.clone()),
                backend_communicator.clone(),
            );
            return Ok(ok_hashmap_str);
        }
        Err(err_str) => return Err(err_str),
    }
}
