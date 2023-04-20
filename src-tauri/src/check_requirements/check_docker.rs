use crate::utility;
use crate::utility::log_and_emit;
use crate::BackendCommunicator;
use std::process::Command;

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
