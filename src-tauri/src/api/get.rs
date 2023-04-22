use serde_json::Value;

use crate::{utility::log_and_emit, BackendCommunicator};
use std::collections::HashMap;

use super::utility::api_json_query_to_hashmap;

/// Query Index API for session info based on node address
pub async fn get_node_session_from_api(
    node_address: String, // eg. xe_7a65d81dC21E87d593aC30DFe0AcbC2622bbdAE8
    backend_communicator: BackendCommunicator,
) -> Result<HashMap<String, Value>, String> {
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
            return Ok(ok_hashmap);
        }
        Err(err_str) => return Err(err_str),
    }
}

/// Query Index API for snapshots info based on node address
pub async fn get_node_snapshots_from_api(
    node_address: String, // eg. xe_7a65d81dC21E87d593aC30DFe0AcbC2622bbdAE8
    backend_communicator: BackendCommunicator,
) -> Result<HashMap<String, Value>, String> {
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
            return Ok(ok_hashmap);
        }
        Err(err_str) => return Err(err_str),
    }
}

/// Query Index API for stake info based on stake token
pub async fn get_stake_info_from_api(
    stake_token: String, // eg. 9d51f5129e9188ba9622163f06b34e51071be224209365ad367d1300979e0b0e
    backend_communicator: BackendCommunicator,
) -> Result<HashMap<String, Value>, String> {
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
            return Ok(ok_hashmap);
        }
        Err(err_str) => return Err(err_str),
    }
}

/// Query Index API for transaction history based on xe_address
pub async fn get_transaction_history_from_api(
    xe_address: String, // eg. xe_7a65d81dC21E87d593aC30DFe0AcbC2622bbdAE8
    backend_communicator: BackendCommunicator,
) -> Result<HashMap<String, Value>, String> {
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
            return Ok(ok_hashmap);
        }
        Err(err_str) => return Err(err_str),
    }
}
