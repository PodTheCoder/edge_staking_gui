use crate::check_requirements;
use crate::BackendCommunicator;
use std::process::Command;

/// Run a command in the Edge Cli
fn command_edge_cli(
    cli_command: String,
    backend_communicator: BackendCommunicator,
) -> Result<String, String> {
    let arglist: Vec<&str> = cli_command.split(' ').collect();
    let output;

    println!("{}", backend_communicator.data_dir.clone());
    // Requirements must first be met before commands can be run.
    match check_requirements::main(backend_communicator.clone()) {
        Ok(_) => {}
        Err(err) => {
            let error_message = format!("You need to pass all system checks before running an Edge CLI command. Your system check results: {}", err);
            return Err(error_message);
        }
    }
    println!("Invoking command in Edge CLI = {}", cli_command);
    // TODO: Add edge binary program path as arg
    let bin_name = "edge.exe";
    let bin_path = format!("{}{}", backend_communicator.data_dir.clone(), bin_name);

    match Command::new(bin_path).args(arglist).output() {
        Ok(command_completed_result) => {
            output = command_completed_result;
            println!(
                "Edge CLI command {} completed with status code:{}",
                cli_command.clone(),
                output.clone().status.code().unwrap()
            )
        }
        Err(command_not_completed) => {
            return Err(format!(
                "Command did not complete successfully. Error = {}",
                command_not_completed.to_string()
            ))
        }
    }

    // Convert stdout strings
    let stdout_output_str: String;
    match String::from_utf8(output.stdout.to_vec()) {
        Ok(ok_converted_str) => stdout_output_str = ok_converted_str,
        Err(_) => return Err(format!("Unable to convert stdout.")),
    }
    let stderr_output_str: String;
    match String::from_utf8(output.stderr.to_vec()) {
        Ok(ok_converted_str) => stderr_output_str = ok_converted_str,
        Err(_) => stderr_output_str = format!("Unable to convert stderr."),
    }

    let exit_code: i32;
    let cli_found_successful_command = 0;
    let cli_found_failed_command = 1;
    match output.status.code() {
        Some(code) => {
            exit_code = code;
            println!("Edge CLI exit code = {}", exit_code);

            if exit_code == cli_found_successful_command {
                return Ok(format!("{}", stdout_output_str));
            } else if exit_code == cli_found_failed_command {
                return Err(format!(
                    "CLI installed but ran with error: {}",
                    stderr_output_str
                ));
            } else {
                return Err(String::from(format!("Edge exit code not recognized")));
            }
        }
        None => Err(String::from("Edge running status could not be checked.")),
    }
}
pub fn device_stop(backend_communicator: BackendCommunicator) -> String {
    let cli_command = String::from("device stop");
    match command_edge_cli(cli_command, backend_communicator) {
        Ok(ok_str) => return ok_str,
        Err(err_str) => return err_str,
    }
}

pub fn device_start(backend_communicator: BackendCommunicator) -> String {
    let cli_command = String::from("device start");
    match command_edge_cli(cli_command, backend_communicator) {
        Ok(ok_str) => return ok_str,
        Err(err_str) => return err_str,
    }
}

pub fn device_info(backend_communicator: BackendCommunicator) -> String {
    let cli_command = String::from("device info");
    match command_edge_cli(cli_command, backend_communicator) {
        Ok(ok_str) => return ok_str,
        Err(err_str) => return err_str,
    }
}
