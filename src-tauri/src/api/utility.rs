use std::{
    collections::{HashMap, VecDeque},
    fs,
};

use serde_json::Value;

use crate::{
    utility::{download_file, log_and_emit},
    BackendCommunicator,
};

/// Convenience function, download file from Edge Index API and conver it into a hashmap
pub async fn json_to_api_hashmap(
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
}

fn helper_parse_json_object_key_to_parsed_strings_vector(
    json_object_key: String,
    nesting_limit: i32,
    backend_communicator: BackendCommunicator,
) -> Result<VecDeque<String>, String> {
    log_and_emit(
        format!("Parsing string from API: {}", json_object_key.clone()),
        backend_communicator.clone(),
    );
    let mut parsed_strings_vector: VecDeque<String> = VecDeque::new();
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
    Ok(parsed_strings_vector)
}

fn helper_traverse_api_hashmap(
    api_hashmap: HashMap<String, Value>,
    parsed_strings_vector: VecDeque<String>,
    nesting_limit: i32,
    backend_communicator: BackendCommunicator,
) -> Result<String, String> {
    let mut api_hashmap = api_hashmap;
    let mut strings_left_to_traverse = parsed_strings_vector;
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
    }
    let error_message = format!("Did not execute parse_api_hashmap_to_field_value loops.");
    log_and_emit(error_message.clone(), backend_communicator.clone());
    return Err(error_message);
}

/// Parse the api hashmap using the object key. Key format supports nested lookups eg. "first:second:third"
pub async fn lookup_value_from_api_hashmap(
    api_hashmap: HashMap<String, Value>,
    json_object_key: String,
    backend_communicator: BackendCommunicator,
) -> Result<String, String> {
    let nesting_limit = 5;
    let parsed_strings_vector: VecDeque<String>;
    match helper_parse_json_object_key_to_parsed_strings_vector(
        json_object_key,
        nesting_limit,
        backend_communicator.clone(),
    ) {
        Ok(ok_vec_deque) => parsed_strings_vector = ok_vec_deque,
        Err(err) => return Err(err),
    };

    match helper_traverse_api_hashmap(
        api_hashmap,
        parsed_strings_vector,
        nesting_limit,
        backend_communicator.clone(),
    ) {
        Ok(final_value) => return Ok(final_value),
        Err(err) => return Err(err),
    }
}
