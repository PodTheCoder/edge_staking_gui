use crate::{
    api::{get_node_session_from_api, get_stake_info_from_api, lookup_value_from_api_hashmap},
    utility::log_and_emit,
    BackendCommunicator,
};

/// Use the API to derive the stake token from node address
pub async fn derive_stake_token_from_node_address(
    node_address: String,
    backend_communicator: BackendCommunicator,
) -> Result<String, String> {
    let api_hashmap;
    match get_node_session_from_api(node_address.clone(), backend_communicator.clone()).await {
        Ok(ok_hashmap) => {
            api_hashmap = ok_hashmap;
        }
        Err(_) => {
            let error_message = format!("Unable to derive stake.");
            log_and_emit(error_message.clone(), backend_communicator.clone());
            return Err(error_message);
        }
    }

    let json_object_key = format!("node:stake");
    match lookup_value_from_api_hashmap(api_hashmap, json_object_key, backend_communicator.clone())
        .await
    {
        Ok(ok_stake_value) => {
            let stake_value = ok_stake_value;
            let ok_message = format!("Found stake value {}", stake_value.clone());
            log_and_emit(ok_message, backend_communicator.clone());
            return Ok(stake_value);
        }
        Err(_) => {
            let error_message = format!("Could not lookup stake value.");
            log_and_emit(error_message.clone(), backend_communicator.clone());
            return Err(error_message);
        }
    }
}

/// Use the api to derive the wallet address from the node address
pub async fn derive_wallet_address_from_node_address(
    node_address: String,
    backend_communicator: BackendCommunicator,
) -> Result<String, String> {
    // Derive stake from device address
    let stake_token;
    match derive_stake_token_from_node_address(node_address.clone(), backend_communicator.clone())
        .await
    {
        Ok(ok_stake_token) => {
            stake_token = ok_stake_token;
        }
        Err(_) => {
            let error_message = format!("Could not derive stake inside derive_wallet_address.");
            log_and_emit(error_message.clone(), backend_communicator.clone());
            return Err(error_message);
        }
    }

    // Derive wallet address from stake

    match derive_wallet_address_from_stake(stake_token, backend_communicator).await {
        Ok(ok_wallet_address) => return Ok(ok_wallet_address),
        Err(err) => return Err(err),
    }
}

async fn derive_wallet_address_from_stake(
    stake_token: String,
    backend_communicator: BackendCommunicator,
) -> Result<String, String> {
    // Get JSON
    let api_hashmap;
    match get_stake_info_from_api(stake_token.clone(), backend_communicator.clone()).await {
        Ok(ok_api_hashmap) => {
            api_hashmap = ok_api_hashmap;
        }
        Err(err_str) => {
            return Err(err_str);
        }
    }

    // Lookup Value
    let json_object_key = format!("wallet");
    match lookup_value_from_api_hashmap(api_hashmap, json_object_key, backend_communicator.clone())
        .await
    {
        Ok(ok_wallet_address) => {
            let ok_message = format!(
                "Derived wallet address: {} from stake {}",
                ok_wallet_address.clone(),
                stake_token.clone(),
            );
            log_and_emit(ok_message.clone(), backend_communicator.clone());
            return Ok(ok_wallet_address);
        }
        Err(err_str) => return Err(err_str),
    }
}
