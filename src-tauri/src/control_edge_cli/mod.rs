use crate::check_requirements;
use crate::config::getters::{get_edge_binary_name, get_stake_id};
use crate::utility::log_and_emit;
use crate::BackendCommunicator;
use std::process::Command;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

/// Run a command in the Edge Cli
/// You can choose whether the edge binary must be the latest version to run the command.
async fn command_edge_cli(
    cli_command: String,
    checklatestbinary: bool,
    backend_communicator: &BackendCommunicator,
) -> Result<String, String> {
    let arglist: Vec<&str> = cli_command.split(' ').collect();
    let output;

    // Requirements must first be met before commands can be run.
    let check_requirements_future =
        check_requirements::main(true, true, true, checklatestbinary, backend_communicator).await;
    match check_requirements_future {
        Ok(_) => {}
        Err(err) => {
            let error_message = format!("You need to pass all system checks before running an Edge CLI command. Your system check results: {}", err);
            log_and_emit(error_message.clone(), backend_communicator);
            return Err(error_message);
        }
    }

    let bin_name = get_edge_binary_name(backend_communicator);
    let bin_path = format!("{}{}", backend_communicator.data_dir.clone(), bin_name);

    log_and_emit(
        format!("Invoking command in Edge CLI = {}", cli_command),
        backend_communicator,
    );
    #[cfg(target_os = "windows")]
    const WINDOWS_CREATE_NO_WINDOW: u32 = 0x08000000;

    #[cfg(target_os = "windows")]
    let command = Command::new(bin_path)
        .args(arglist)
        .creation_flags(WINDOWS_CREATE_NO_WINDOW)
        .env("PATH", "/bin")
        .output();

    #[cfg(not(target_os = "windows"))]
    let command = Command::new(bin_path)
        .args(arglist)
        .env("PATH", "/bin")
        .output();

    match command {
        Ok(command_completed_result) => {
            output = command_completed_result;
            log_and_emit(
                format!(
                    "Edge CLI command `{}` completed.",
                    cli_command.clone(),
                    // output.clone().status.code().unwrap()
                ),
                backend_communicator,
            );
        }
        Err(command_not_completed) => {
            let error_message = format!(
                "Command {} did not complete successfully. Error = {}",
                cli_command.clone(),
                command_not_completed
            );
            log_and_emit(error_message.clone(), backend_communicator);
            return Err(error_message);
        }
    }

    // Convert stdout strings

    let stdout_output_str: String;
    match String::from_utf8(output.stdout.to_vec()) {
        Ok(ok_converted_str) => stdout_output_str = ok_converted_str,
        Err(_) => {
            let err_message = "Unable to convert stdout.".to_string();
            log_and_emit(err_message.clone(), backend_communicator);
            return Err(err_message);
        }
    };

    let stderr_output_str: String;
    match String::from_utf8(output.stderr.to_vec()) {
        Ok(ok_converted_str) => stderr_output_str = ok_converted_str,
        Err(_) => {
            stderr_output_str = "Unable to convert stderr.".to_string();
            log_and_emit(stderr_output_str.clone(), backend_communicator);
            return Err(stderr_output_str);
        }
    };

    let exit_code: i32;
    let cli_found_successful_command = 0;
    let cli_found_failed_command = 1;
    match output.status.code() {
        Some(code) => {
            exit_code = code;
            log_and_emit(
                format!("Edge CLI exit code = {}", exit_code),
                backend_communicator,
            );

            if exit_code == cli_found_successful_command {
                let log_message = format!("Stdout: {}", stdout_output_str);
                log_and_emit(log_message, backend_communicator);
                Ok(stdout_output_str)
            } else if exit_code == cli_found_failed_command {
                let err_message =
                    format!("CLI installed but ran with error: {}", stderr_output_str);
                log_and_emit(err_message.clone(), backend_communicator);
                return Err(err_message);
            } else {
                let err_message = format!("Edge exit code {} not recognized.", exit_code);
                log_and_emit(err_message.clone(), backend_communicator);
                return Err(err_message);
            }
        }
        None => {
            let err_message = "Edge CLI running status could not be checked.".to_string();
            log_and_emit(err_message.clone(), backend_communicator);
            Err(err_message)
        }
    }
}

/// Stop Edge device
pub async fn device_stop_from_frontend(
    checklatestbinary: bool,
    backend_communicator: &BackendCommunicator,
) -> bool {
    let stake_id: String = get_stake_id(backend_communicator);
    let cli_command = format!("device stop --stake={}", stake_id);
    let command_edge_cli_future =
        command_edge_cli(cli_command, checklatestbinary, backend_communicator).await;
    match command_edge_cli_future {
        Ok(_) => {
            let ok_message = "Device stopped successfully.".to_string();
            log_and_emit(ok_message, backend_communicator);
            true
        }
        Err(stderr_str) => {
            log_and_emit(stderr_str, backend_communicator);
            false
        }
    }
}

/// Start Edge device; true if started successfully, otherwise false
pub async fn device_start_from_frontend(
    checklatestbinary: bool,
    backend_communicator: &BackendCommunicator,
) -> bool {
    let stake_id: String = get_stake_id(backend_communicator);
    let cli_command = format!("device start --stake={}", stake_id);
    let command_edge_cli_future =
        command_edge_cli(cli_command, checklatestbinary, backend_communicator).await;
    match command_edge_cli_future {
        Ok(stdout_str) => {
            let ok_message = format!("Device successfully started! Ok msg: {}", stdout_str);
            log_and_emit(ok_message, backend_communicator);
            true
        }
        Err(stderr_str) => {
            let error_message = format!("Could not start device. Error: {}", stderr_str);
            log_and_emit(error_message, backend_communicator);
            false
        }
    }
}
