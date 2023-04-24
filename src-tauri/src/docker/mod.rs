use crate::utility;
use crate::utility::log_and_emit;
use crate::BackendCommunicator;
use std::process::Command;

/// Start a temporary docker container for the purpose of copying data
pub(crate) fn start_docker_container_for_copying_data(
    backend_communicator: BackendCommunicator,
) -> Result<String, String> {
    let output;
    let args = [
        "container",
        "create",
        "--name",
        "temp_container_for_copying_edge_device_data",
        "-v",
        "edge-device-data:/data",
        "alpine",
    ];

    match Command::new("docker").args(args).output() {
        Ok(command_completed_result) => output = command_completed_result,
        Err(command_not_completed) => {
            let err_message = format!(
                "Could not create temporary container. Error: {}",
                command_not_completed.to_string()
            );
            log_and_emit(err_message.clone(), backend_communicator.clone());
            return Err(err_message);
        }
    }

    let exit_code;
    let success_exit_code = 0;

    match output.status.code() {
        Some(code) => {
            exit_code = code;
            log_and_emit(
                format!("Docker Exit code = {}", exit_code),
                backend_communicator.clone(),
            );
            if exit_code == success_exit_code {
                let ok_message = format!("Created temporary container for copying data.");
                log_and_emit(ok_message.clone(), backend_communicator.clone());
                return Ok(ok_message);
            } else {
                let err_message = format!(
                    "Did not recognize error code for starting temporary container: {}",
                    exit_code
                );
                log_and_emit(err_message.clone(), backend_communicator.clone());
                return Err(err_message);
            }
        }
        None => {
            let error_message = format!("Could not find error code for Docker output.");
            log_and_emit(error_message.clone(), backend_communicator.clone());
            return Err(error_message);
        }
    }
}

/// Os-independent docker status check based on https://docs.docker.com/config/daemon/troubleshoot/#check-whether-docker-is-running
pub(crate) fn get_docker_status(
    backend_communicator: BackendCommunicator,
) -> Result<String, String> {
    let output;

    // OS-independent method implemented
    match Command::new("docker").arg("info").output() {
        Ok(command_completed_result) => output = command_completed_result,
        Err(command_not_completed) => {
            let errormessage = format!(
                "Docker command was not completed. Is Docker installed & did you restart your computer? Docker installation link: https://www.docker.com/products/docker-desktop/ Error = {}",
                command_not_completed.to_string()
            );
            utility::log_and_emit(errormessage.clone(), backend_communicator);
            return Err(errormessage);
        }
    }

    let exit_code: i32;
    let docker_installed_and_running_code = 0;
    let docker_installed_not_running_code = 1;
    match output.status.code() {
        Some(code) => {
            exit_code = code;
            log_and_emit(
                format!("Docker Exit code = {}", exit_code),
                backend_communicator.clone(),
            );

            if exit_code == docker_installed_and_running_code {
                let ok_string = String::from("Docker installed & ready.");
                log_and_emit(ok_string.clone(), backend_communicator.clone());
                return Ok(ok_string);
            } else if exit_code == docker_installed_not_running_code {
                let err_string = format!("Docker installed but not running");
                log_and_emit(err_string.clone(), backend_communicator.clone());
                return Err(err_string);
            } else {
                let err_string = format!("Docker exit code not recognized");
                log_and_emit(err_string.clone(), backend_communicator.clone());
                return Err(err_string);
            }
        }
        None => {
            let err_string = format!("Docker running status could not be checked");
            log_and_emit(err_string.clone(), backend_communicator.clone());
            return Err(err_string);
        }
    }

    // output Docker log.

    // io::stdout().write_all(&output.stdout).unwrap();
    // io::stderr().write_all(&output.stderr).unwrap();
}
