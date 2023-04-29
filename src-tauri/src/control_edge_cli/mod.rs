use crate::check_requirements;
use crate::utility::{config_set_device_initialization_status, log_and_emit};
use crate::BackendCommunicator;
use std::process::Command;

/// Run a command in the Edge Cli
async fn command_edge_cli(
    cli_command: String,
    check_edge_binary_latest_version: bool,
    backend_communicator: BackendCommunicator,
) -> Result<String, String> {
    let arglist: Vec<&str> = cli_command.split(' ').collect();
    let output;

    // Requirements must first be met before commands can be run.
    let check_requirements_future = check_requirements::main(
        true,
        true,
        true,
        check_edge_binary_latest_version,
        backend_communicator.clone(),
    )
    .await;
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
pub async fn device_stop(backend_communicator: BackendCommunicator) {
    let cli_command = String::from("device stop");
    let command_edge_cli_future =
        command_edge_cli(cli_command, false, backend_communicator.clone()).await;
    match command_edge_cli_future {
        Ok(_) => {
            let ok_message = format!("Device stopped successfully.");
            log_and_emit(ok_message, backend_communicator.clone());
        }
        Err(stderr_str) => {
            log_and_emit(stderr_str, backend_communicator.clone());
        }
    }
}

/// Start Edge device; true if started successfully, otherwise false
pub async fn device_start(backend_communicator: BackendCommunicator) -> bool {
    let cli_command = String::from("device start");
    let command_edge_cli_future =
        command_edge_cli(cli_command, true, backend_communicator.clone()).await;
    match command_edge_cli_future {
        Ok(stdout_str) => {
            match config_set_device_initialization_status(true, backend_communicator.clone()) {
                Ok(_) => {
                    let ok_message = format!("Started device! {}", stdout_str);
                    log_and_emit(ok_message, backend_communicator.clone());
                    return true;
                }
                Err(_) => {
                    let error_message = format!("{}, however could not change config. This has no impact on your running node.", stdout_str);
                    log_and_emit(error_message.clone(), backend_communicator.clone());
                    return false;
                }
            }
        }
        Err(stderr_str) => {
            let error_message = format!("Could not start device. Err: {}", stderr_str);
            log_and_emit(error_message, backend_communicator.clone());
            return false;
        }
    }
}

/// Get local Edge device info
pub async fn device_info(backend_communicator: BackendCommunicator) -> String {
    let cli_command = String::from("device info");
    let command_edge_cli_future =
        command_edge_cli(cli_command, false, backend_communicator.clone()).await;
    match command_edge_cli_future {
        Ok(stdout_str) => {
            let ok_message = format!("Received device info successfully");
            log_and_emit(ok_message, backend_communicator.clone());
            return stdout_str;
        }
        Err(stderr_str) => {
            let err_message = format!("Failed to receive device info.");
            log_and_emit(err_message, backend_communicator.clone());
            return stderr_str;
        }
    }
}

/// Update Edge CLI to latest version via CMD
pub async fn update_edge_cli(backend_communicator: BackendCommunicator) -> bool {
    let cli_command = String::from("update");
    let command_edge_cli_future =
        command_edge_cli(cli_command, false, backend_communicator.clone()).await;
    match command_edge_cli_future {
        Ok(ok_msg) => {
            let ok_message = format!("Edge CLI updated successfully: {}", ok_msg);
            log_and_emit(ok_message, backend_communicator.clone());
            return true;
        }
        Err(err_msg) => {
            let err_message = format!("Edge CLI failed to update: {}", err_msg);
            log_and_emit(err_message, backend_communicator.clone());
            return false;
        }
    }
}
