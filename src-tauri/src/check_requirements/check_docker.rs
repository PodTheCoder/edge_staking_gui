use std::process::Command;

pub(crate) fn get_docker_status() -> Result<String, String> {
    let output;

    // OS-independent method implemented https://docs.docker.com/config/daemon/troubleshoot/#check-whether-docker-is-running
    match Command::new("docker").arg("info").output() {
        Ok(command_completed_result) => output = command_completed_result,
        Err(command_not_completed) => {
            return Err(format!(
                "Docker command was not completed. Error = {}",
                command_not_completed.to_string()
            ))
        }
    }

    let exit_code: i32;
    let docker_installed_and_running_code = 0;
    let docker_installed_not_running_code = 1;
    match output.status.code() {
        Some(code) => {
            exit_code = code;
            println!("Docker Exit code = {}", exit_code);

            if exit_code == docker_installed_and_running_code {
                return Ok(String::from("Docker installed & ready."));
            } else if exit_code == docker_installed_not_running_code {
                return Err(String::from("Docker installed but not running."));
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
