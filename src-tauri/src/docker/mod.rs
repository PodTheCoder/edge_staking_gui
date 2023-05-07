use crate::utility;
use crate::utility::log_and_emit;
use crate::BackendCommunicator;
use std::process::Command;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

pub(crate) fn remove_temporary_container(
    backend_communicator: &BackendCommunicator,
) -> Result<String, String> {
    let args = ["rm", "temp_container_for_copying_edge_device_data"];

    

    #[cfg(target_os = "windows")]
    const WINDOWS_CREATE_NO_WINDOW: u32 = 0x08000000;

    #[cfg(target_os = "windows")]
    let command = Command::new("docker")
        .args(args)
        .creation_flags(WINDOWS_CREATE_NO_WINDOW)
        .output();

    #[cfg(not(target_os = "windows"))]
    let command = Command::new("docker").args(args).output();

    let output = match command {
        Ok(command_completed_result) => command_completed_result,
        Err(command_not_completed) => {
            let err_message = format!(
                "Could not run docker close container command. Error: {}",
                command_not_completed
            );
            log_and_emit(err_message.clone(), backend_communicator);
            return Err(err_message);
        }
    };

    let exit_code;
    let success_exit_code = 0;

    match output.status.code() {
        Some(code) => {
            exit_code = code;
            log_and_emit(
                format!("Docker Exit code = {}", exit_code),
                backend_communicator,
            );
            if exit_code == success_exit_code {
                let ok_message = "Closed docker container".to_string();
                log_and_emit(ok_message.clone(), backend_communicator);
                Ok(ok_message)
            } else {
                let err_message = format!(
                    "Did not recognize error code for closing container: {}",
                    exit_code
                );
                log_and_emit(err_message.clone(), backend_communicator);
                Err(err_message)
            }
        }
        None => {
            let error_message = "Could not find error code for Docker output.".to_string();
            log_and_emit(error_message.clone(), backend_communicator);
            Err(error_message)
        }
    }
}

/// Copy the local edge-device-data files to the running container
pub(crate) fn copy_data_to_running_container(
    file_to_copy_path: String,
    backend_communicator: &BackendCommunicator,
) -> Result<String, String> {
    let args = [
        "cp",
        &file_to_copy_path,
        "temp_container_for_copying_edge_device_data:/data",
    ];

    

    #[cfg(target_os = "windows")]
    const WINDOWS_CREATE_NO_WINDOW: u32 = 0x08000000;

    #[cfg(target_os = "windows")]
    let command = Command::new("docker")
        .args(args)
        .creation_flags(WINDOWS_CREATE_NO_WINDOW)
        .output();

    #[cfg(not(target_os = "windows"))]
    let command = Command::new("docker").args(args).output();

    let output = match command {
        Ok(command_completed_result) => command_completed_result,
        Err(command_not_completed) => {
            let err_message = format!(
                "Could not run copy file command. Error: {}",
                command_not_completed
            );
            log_and_emit(err_message.clone(), backend_communicator);
            return Err(err_message);
        }
    };

    let exit_code;
    let success_exit_code = 0;

    match output.status.code() {
        Some(code) => {
            exit_code = code;
            log_and_emit(
                format!("Docker Exit code = {}", exit_code),
                backend_communicator,
            );
            if exit_code == success_exit_code {
                let ok_message = "Copied file to edge device data volume.".to_string();
                log_and_emit(ok_message.clone(), backend_communicator);
                Ok(ok_message)
            } else {
                let err_message = format!(
                    "Did not recognize error code for copying data to volume: {}",
                    exit_code
                );
                log_and_emit(err_message.clone(), backend_communicator);
                Err(err_message)
            }
        }
        None => {
            let error_message = "Could not find error code for Docker output.".to_string();
            log_and_emit(error_message.clone(), backend_communicator);
            Err(error_message)
        }
    }
}

/// Start a temporary docker container for the purpose of copying data
pub(crate) fn start_docker_container_for_copying_data(
    backend_communicator: &BackendCommunicator,
) -> Result<String, String> {
    
    let args = [
        "container",
        "create",
        "--name",
        "temp_container_for_copying_edge_device_data",
        "-v",
        "edge-device-data:/data",
        "alpine",
    ];

    #[cfg(target_os = "windows")]
    const WINDOWS_CREATE_NO_WINDOW: u32 = 0x08000000;

    #[cfg(target_os = "windows")]
    let command = Command::new("docker")
        .args(args)
        .creation_flags(WINDOWS_CREATE_NO_WINDOW)
        .output();

    #[cfg(not(target_os = "windows"))]
    let command = Command::new("docker").args(args).output();

    let output = match command {
        Ok(command_completed_result) => command_completed_result,
        Err(command_not_completed) => {
            let err_message = format!(
                "Could not create temporary container. Error: {}",
                command_not_completed
            );
            log_and_emit(err_message.clone(), backend_communicator);
            return Err(err_message);
        }
    };

    let exit_code;
    let success_exit_code = 0;

    match output.status.code() {
        Some(code) => {
            exit_code = code;
            log_and_emit(
                format!("Docker Exit code = {}", exit_code),
                backend_communicator,
            );
            if exit_code == success_exit_code {
                let ok_message = "Created temporary container for copying data.".to_string();
                log_and_emit(ok_message.clone(), backend_communicator);
                Ok(ok_message)
            } else {
                let err_message = format!(
                    "Did not recognize error code for starting temporary container: {}",
                    exit_code
                );
                log_and_emit(err_message.clone(), backend_communicator);
                Err(err_message)
            }
        }
        None => {
            let error_message = "Could not find error code for Docker output.".to_string();
            log_and_emit(error_message.clone(), backend_communicator);
            Err(error_message)
        }
    }
}

/// Os-independent docker status check based on https://docs.docker.com/config/daemon/troubleshoot/#check-whether-docker-is-running
pub(crate) fn get_docker_status(
    backend_communicator: &BackendCommunicator,
) -> Result<String, String> {
    

    // OS-independent info method implemented

    #[cfg(target_os = "windows")]
    const WINDOWS_CREATE_NO_WINDOW: u32 = 0x08000000;

    #[cfg(target_os = "windows")]
    let command = Command::new("docker")
        .arg("info")
        .creation_flags(WINDOWS_CREATE_NO_WINDOW)
        .output();

    #[cfg(not(target_os = "windows"))]
    let command = Command::new("docker").arg("info").output();

    let output = match command {
        Ok(command_completed_result) => command_completed_result,
        Err(command_not_completed) => {
            let errormessage = format!(
                "Is Docker installed & did you restart your computer? Docker installation link: https://www.docker.com/products/docker-desktop/ Error = {}",
                command_not_completed
            );
            utility::log_and_emit(errormessage.clone(), backend_communicator);
            return Err(errormessage);
        }
    };

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
                let ok_string = String::from("Docker installed & ready.");
                log_and_emit(ok_string.clone(), backend_communicator);
                Ok(ok_string)
            } else if exit_code == docker_installed_not_running_code {
                let err_string = "Docker installed but not running/ready. Docker loading time can be several minutes.".to_string();
                log_and_emit(err_string.clone(), backend_communicator);
                return Err(err_string);
            } else {
                let err_string = "Docker exit code not recognized".to_string();
                log_and_emit(err_string.clone(), backend_communicator);
                return Err(err_string);
            }
        }
        None => {
            let err_string = "Docker running status could not be checked".to_string();
            log_and_emit(err_string.clone(), backend_communicator);
            Err(err_string)
        }
    }

    // output Docker log.

    // io::stdout().write_all(&output.stdout).unwrap();
    // io::stderr().write_all(&output.stderr).unwrap();
}
