use serde_json::Value;
use std::{collections::HashMap, collections::VecDeque, fs};

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

/// Parse the api hashmap using the object key. Key format supports nested lookups eg. "first:second:third"
pub async fn lookup_value_from_api_hashmap(
    api_hashmap: HashMap<String, Value>,
    json_object_key: String,
    backend_communicator: BackendCommunicator,
) -> Result<String, String> {
    // Parse JSON object key
    log_and_emit(
        format!("Parsing string from API: {}", json_object_key.clone()),
        backend_communicator.clone(),
    );
    let nesting_limit = 5;
    let mut parsed_strings_vector: VecDeque<String> = VecDeque::new(); // Contains parsed strings
    let mut string_to_parse = json_object_key.clone();

    for parse_json_loop_count in 0..nesting_limit.clone() {
        match string_to_parse.find(":") {
            Some(ok_colon_pos) => {
                let colon_pos = ok_colon_pos;
                let parsed_word = String::from(&string_to_parse[0..colon_pos]);
                parsed_strings_vector.push_back(parsed_word.clone());
                log_and_emit(
                    format!("Parsed word: {}", parsed_word.clone()),
                    backend_communicator.clone(),
                );
                if colon_pos < string_to_parse.len() - 1 {
                    string_to_parse = String::from(&string_to_parse[colon_pos + 1..]);
                    continue; // next loop
                } else {
                    // colon pos is last char, nothing left to parse.
                    let error_message = format!(
                        "Malformed json_object_key {} . Should not end with colon.",
                        json_object_key
                    );
                    log_and_emit(error_message.clone(), backend_communicator.clone());
                    return Err(error_message);
                }
            }
            None => {
                if parse_json_loop_count == 0 {
                    log_and_emit(
                        format!(
                            "Json object key: {} is not nested.",
                            json_object_key.clone()
                        ),
                        backend_communicator.clone(),
                    );
                } else {
                    log_and_emit(
                        format!(
                            "Json object was nested. Nest count : {}",
                            parse_json_loop_count
                        ),
                        backend_communicator.clone(),
                    );
                }
                let parsed_word = String::from(&string_to_parse[0..]);
                parsed_strings_vector.push_back(parsed_word.clone());
                log_and_emit(
                    format!("Parsed word: {}", parsed_word.clone()),
                    backend_communicator.clone(),
                );
                break; // Break parsing loop.
            }
        }
    }

    let mut api_hashmap = api_hashmap;
    let mut strings_left_to_traverse = parsed_strings_vector;
    // Traverse hashmap until find final key.
    log_and_emit(format!("Traversing hashmap."), backend_communicator.clone());
    for traverse_hashmap_loop_count in 0..nesting_limit.clone() {
        if strings_left_to_traverse.len() == 0 {
            let error_message = format!("Error: Expected a non_empty parsed strings vector.");
            log_and_emit(error_message.clone(), backend_communicator.clone());
            return Err(error_message);
        }

        match strings_left_to_traverse.pop_front() {
            Some(popped_front_str) => {
                let ultimate_key_str = popped_front_str;
                if strings_left_to_traverse.len() == 0 {
                    log_and_emit(
                        format!("No more strings in parsed_strings_vector. Reached final string? Nesting level: {}", traverse_hashmap_loop_count),
                        backend_communicator.clone(),
                    );
                    match api_hashmap.get(&ultimate_key_str.clone()) {
                        Some(ultimate_value) => {
                            let ultimate_value_str;
                            match ultimate_value.as_str() {
                                Some(str_without_quotes) => {
                                    ultimate_value_str = str_without_quotes;
                                }
                                None => {
                                    let error_message = format!("Unable to interpret ultimate value as string. To string variant: {}", ultimate_value.to_string());
                                    log_and_emit(
                                        error_message.clone(),
                                        backend_communicator.clone(),
                                    );
                                    return Err(error_message);
                                }
                            }
                            log_and_emit(
                                format!(
                                    "Key: {} has value: {}",
                                    ultimate_key_str.clone(),
                                    ultimate_value_str.clone()
                                ),
                                backend_communicator.clone(),
                            );
                            let ultimate_string_without_quotes = format!("{}", ultimate_value_str);
                            return Ok(ultimate_string_without_quotes);
                        }
                        None => {
                            let error_message =
                                format!("Could not find value for key {}", ultimate_key_str);
                            log_and_emit(error_message.clone(), backend_communicator.clone());
                            return Err(error_message);
                        }
                    }
                } else {
                    let nested_key = ultimate_key_str;
                    log_and_emit(
                        format!("Traversing nested key: {}", nested_key),
                        backend_communicator.clone(),
                    );
                    match api_hashmap.get(&nested_key) {
                        Some(nested_api_hashmap) => {
                            let potential_valid_json = nested_api_hashmap.to_string();
                            match serde_json::from_str(&potential_valid_json) {
                                Ok(ok_json) => {
                                    let json_content_hashmap: HashMap<String, Value> = ok_json;
                                    log_and_emit(
                                        format!("Setting traversed hashmap as new hashmap."),
                                        backend_communicator.clone(),
                                    );
                                    api_hashmap = json_content_hashmap;
                                    continue;
                                }

                                Err(_) => {
                                    let error_message =
                                        format!("Could not parse traversed hashmap as JSON.");
                                    log_and_emit(
                                        error_message.clone(),
                                        backend_communicator.clone(),
                                    );
                                    return Err(error_message);
                                }
                            }
                        }
                        None => {
                            let error_message = format!(
                                "Could not find nested_key: {} in api_hashmap.",
                                nested_key
                            );
                            log_and_emit(error_message.clone(), backend_communicator.clone());
                            return Err(error_message);
                        }
                    }
                }
            }
            None => return Err(format!("Could not pop vector value of parsed string.")),
        }

        // Traverse hashmap. Parents are Hashmap<String, String>. Deepest level is <String, Value>
        // Try to convert .into (casting? )
    }

    let error_message = format!("Did not execute parse_api_hashmap_to_field_value loops.");
    log_and_emit(error_message.clone(), backend_communicator.clone());
    return Err(error_message);
}

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
                "Derived wallet address: {} from stake {} based on node address {}",
                ok_wallet_address.clone(),
                stake_token.clone(),
                node_address.clone()
            );
            log_and_emit(ok_message.clone(), backend_communicator.clone());
            return Ok(ok_wallet_address);
        }
        Err(err_str) => return Err(err_str),
    }
}

// char::is_whitespace
