use crate::check_requirements;
use crate::utility::{config_set_device_initialization_status, log_and_emit};
use crate::BackendCommunicator;
use std::process::Command;

/// Run a command in the Edge Cli
async fn command_edge_cli(
    cli_command: String,
    backend_communicator: BackendCommunicator,
) -> Result<String, String> {
    let arglist: Vec<&str> = cli_command.split(' ').collect();
    let output;

    // Requirements must first be met before commands can be run.
    let check_requirements_future = check_requirements::main(backend_communicator.clone()).await;
    match check_requirements_future {
        Ok(_) => {}
        Err(err) => {
            let error_message = format!("You need to pass all system checks before running an Edge CLI command. Your system check results: {}", err);
            log_and_emit(error_message.clone(), backend_communicator.clone());
            return Err(error_message);
        }
    }
    log_and_emit(
        format!("Invoking command in Edge CLI = {}", cli_command),
        backend_communicator.clone(),
    );
    // TODO: Add edge binary program path as arg
    let bin_name = "edge.exe";
    let bin_path = format!("{}{}", backend_communicator.data_dir.clone(), bin_name);

    match Command::new(bin_path).args(arglist).output() {
        Ok(command_completed_result) => {
            output = command_completed_result;
            log_and_emit(
                format!(
                    "Edge CLI command `{}` completed.",
                    cli_command.clone(),
                    // output.clone().status.code().unwrap()
                ),
                backend_communicator.clone(),
            );
        }
        Err(command_not_completed) => {
            let error_message = format!(
                "Command {} did not complete successfully. Error = {}",
                cli_command.clone(),
                command_not_completed.to_string()
            );
            log_and_emit(error_message.clone(), backend_communicator.clone());
            return Err(error_message);
        }
    }

    // Convert stdout strings
    let stdout_output_str: String;
    match String::from_utf8(output.stdout.to_vec()) {
        Ok(ok_converted_str) => stdout_output_str = ok_converted_str,
        Err(_) => {
            let err_message = format!("Unable to convert stdout.");
            log_and_emit(err_message.clone(), backend_communicator.clone());
            return Err(err_message);
        }
    }
    let stderr_output_str: String;
    match String::from_utf8(output.stderr.to_vec()) {
        Ok(ok_converted_str) => stderr_output_str = ok_converted_str,
        Err(_) => {
            stderr_output_str = format!("Unable to convert stderr.");
            log_and_emit(stderr_output_str.clone(), backend_communicator.clone())
        }
    }

    let exit_code: i32;
    let cli_found_successful_command = 0;
    let cli_found_failed_command = 1;
    match output.status.code() {
        Some(code) => {
            exit_code = code;
            log_and_emit(
                format!("Edge CLI exit code = {}", exit_code),
                backend_communicator.clone(),
            );

            if exit_code == cli_found_successful_command {
                let log_message = format!("Stdout: {}", stdout_output_str);
                log_and_emit(log_message, backend_communicator.clone());
                return Ok(format!("{}", stdout_output_str));
            } else if exit_code == cli_found_failed_command {
                let err_message =
                    format!("CLI installed but ran with error: {}", stderr_output_str);
                log_and_emit(err_message.clone(), backend_communicator.clone());
                return Err(err_message);
            } else {
                let err_message = format!("Edge exit code {} not recognized.", exit_code);
                log_and_emit(err_message.clone(), backend_communicator.clone());
                return Err(err_message);
            }
        }
        None => {
            let err_message = format!("Edge CLI running status could not be checked.");
            log_and_emit(err_message.clone(), backend_communicator.clone());
            return Err(err_message);
        }
    }
}

/// Stop Edge device
pub async fn device_stop(backend_communicator: BackendCommunicator) -> String {
    let cli_command = String::from("device stop");
    let command_edge_cli_future = command_edge_cli(cli_command, backend_communicator.clone()).await;
    match command_edge_cli_future {
        Ok(stdout_str) => {
            let ok_message = format!("Device stopped successfully.");
            log_and_emit(ok_message, backend_communicator.clone());
            return stdout_str;
        }
        Err(stderr_str) => {
            let err_message = format!("Error in running device stop command.");
            log_and_emit(err_message, backend_communicator.clone());
            return stderr_str;
        }
    }
}

/// Start Edge device
pub async fn device_start(backend_communicator: BackendCommunicator) -> String {
    let cli_command = String::from("device start");
    let command_edge_cli_future = command_edge_cli(cli_command, backend_communicator.clone()).await;
    match command_edge_cli_future {
        Ok(stdout_str) => {
            match config_set_device_initialization_status(true, backend_communicator.clone()) {
                Ok(_) => {
                    let ok_message = format!("Started device!");
                    log_and_emit(ok_message, backend_communicator.clone());
                    return stdout_str;
                }
                Err(_) => {
                    let error_message = format!("{}, however could not change config. This has no impact on your running node.", stdout_str);
                    log_and_emit(error_message.clone(), backend_communicator.clone());
                    return error_message;
                }
            }
        }
        Err(stderr_str) => {
            let error_message = format!("Could not start device.");
            log_and_emit(error_message, backend_communicator.clone());
            return stderr_str;
        }
    }
}

/// Get info about node address
pub async fn get_node_address_info(
    throwaway_password: String,
    backend_communicator: BackendCommunicator,
) -> String {
    let cli_command = format!("wallet info -p {}", throwaway_password);
    let command_edge_cli_future = command_edge_cli(cli_command, backend_communicator.clone()).await;
    match command_edge_cli_future {
        Ok(stdout_str) => {
            let ok_message = format!("Received node address info successfully");
            log_and_emit(ok_message, backend_communicator.clone());
            return stdout_str;
        }
        Err(stderr_str) => {
            let err_message = format!("Failed to receive node address info.");
            log_and_emit(err_message, backend_communicator.clone());
            return stderr_str;
        }
    }
}

/// Create device address if not exists using throwaway password
pub async fn create_node_address(
    backend_communicator: BackendCommunicator,
) -> Result<String, String> {
    let throwaway_password = format!("ThrowawayPassDoNotUseAsWallet");
    let node_info_output_pre_creation =
        get_node_address_info(throwaway_password.clone(), backend_communicator.clone()).await;
    let input_error_str = format!("InputError: tty required");
    // Check if a node address already exists
    match node_address_exists(node_info_output_pre_creation, backend_communicator.clone()) {
        Ok(ok_str) => {
            let ok_message = format!("Device address already exists and could be decoded.");
            log_and_emit(ok_message, backend_communicator);
            return Err(ok_str); // Error because you want to create a device address and are unable to.
        }
        Err(_) => {}
    }

    let cli_command = format!("wallet create -p {}", throwaway_password);
    let command_edge_cli_future = command_edge_cli(cli_command, backend_communicator.clone()).await;

    match command_edge_cli_future {
        Ok(stdout_str) => {
            let ok_message = format!("Received device info successfully");
            log_and_emit(ok_message, backend_communicator.clone());
            return Ok(stdout_str);
        }
        Err(stderr_str) => {
            // TODO: Figure out how to interact with a programmatic interface.
            if stderr_str.contains(&input_error_str) {
                log_and_emit(
                    format!(
                        "Command requires user interaction. Checking if node address already exists on device."
                    ),
                    backend_communicator.clone(),
                );
                // Check if node address was created
                let node_address_info_output_post_creation =
                    get_node_address_info(throwaway_password, backend_communicator.clone()).await;

                match node_address_exists(
                    node_address_info_output_post_creation.clone(),
                    backend_communicator.clone(),
                ) {
                    Ok(ok_str) => {
                        let ok_message = format!("Node address created successfully!");
                        log_and_emit(ok_message.clone(), backend_communicator.clone());
                        return Ok(ok_str);
                    }
                    Err(err_str) => {
                        let error_message = format!(
                            "Unable to create new node address. Please contact support on Discord."
                        );
                        log_and_emit(error_message.clone(), backend_communicator.clone());
                        return Err(err_str);
                    }
                }
            } else {
                let error_message = format!(
                    "Unknown error. Expected to find {} in {}",
                    input_error_str, stderr_str
                );
                log_and_emit(error_message.clone(), backend_communicator);
                return Err(error_message);
            }
        }
    }
}

/// Checks if node address exist and passphrase can be decoded, if it does returns the node_address_info_output.
fn node_address_exists(
    node_address_info_output: String,
    backend_communicator: BackendCommunicator,
) -> Result<String, String> {
    let node_address_exists_str = format!("Address:");
    let node_address_not_exists_str = format!("NotFoundError: node address not found");
    let invalid_password_str = format!("Cannot display private key: invalid passphrase");
    let valid_password_str = format!("Private key:");

    // Case: Node address exists
    if node_address_info_output.contains(&node_address_exists_str) {
        // Case: Node address exists, Password invalid.
        if node_address_info_output.contains(&invalid_password_str) {
            let error_message = format!("Wallet already exists on device, but invalid throwaway password. Please contact support.");
            log_and_emit(error_message.clone(), backend_communicator);
            return Err(error_message);
        }
        // Case: Node address exists, Password valid.
        if node_address_info_output.contains(&valid_password_str) {
            let ok_message = format!(
                "Node address already exists on device and could successfully decode throwaway password."
            );
            log_and_emit(ok_message, backend_communicator.clone());
            return Ok(node_address_info_output);
        } else {
            let error_message = format!(
                "Unknown error in node_address_exists: {}",
                node_address_info_output
            );
            log_and_emit(error_message.clone(), backend_communicator.clone());
            return Err(error_message);
        }
    }
    // Case: Node address does not exist
    else if node_address_info_output.contains(&node_address_not_exists_str) {
        let error_message = format!("Node address does not exist: {}", node_address_info_output);
        log_and_emit(error_message.clone(), backend_communicator.clone());
        return Err(error_message);
    } else {
        let error_message = format!(
            "Unknown error in node_address_exists: based on node_address_info_output {}",
            node_address_info_output
        );
        log_and_emit(error_message.clone(), backend_communicator);
        return Err(error_message);
    }
}
