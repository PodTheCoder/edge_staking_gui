use crate::utility_events;
use crate::utility_events::log_and_emit;
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
            utility_events::log_and_emit(errormessage.clone(), backend_communicator);
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
                backend_communicator,
            );

            if exit_code == docker_installed_and_running_code {
                return Ok(String::from("Docker installed & ready."));
            } else if exit_code == docker_installed_not_running_code {
                return Err(String::from("Docker installed but not running/ready."));
            } else {
                return Err(String::from(format!("Docker exit code not recognized")));
            }
        }
        None => Err(String::from("Docker running status could not be checked.")),
    }

    // output Docker log.

    // io::stdout().write_all(&output.stdout).unwrap();
    // io::stderr().write_all(&output.stderr).unwrap();
}
