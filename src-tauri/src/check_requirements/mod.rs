use chrono::{DateTime, Utc};

use crate::docker::get_docker_status;
use crate::BackendCommunicator;
use crate::{
    check_requirements::pretty_check_string::{pretty_err_str, pretty_ok_str},
    utility::log_and_emit,
};

use self::check_edge::is_edge_correctly_downloaded;

pub mod check_edge;
mod check_specifications;
pub mod pretty_check_string;

/// Runs all system requirement checks
pub async fn main(
    check_os: bool,
    check_processor: bool,
    check_docker: bool,
    check_edge_cli_binary: bool,
    backend_communicator: &BackendCommunicator,
) -> Result<String, String> {
    let mut result_string = String::from("");
    let dt: DateTime<Utc> = Utc::now();
    let mut all_requirements_passed = true; // stays true unless at least one does not pass

    // Check OS
    if check_os {
        log_and_emit(
            "Checking if Operating System (OS) is supported".to_string(),
            backend_communicator,
        );
        let os_info = check_specifications::get_os_info(backend_communicator);
        if os_info.os_name_supported {
            let ok_os_name_str = &os_info.cli_os_name;
            let pretty_ok_os_name_str = pretty_check_string::pretty_ok_str(ok_os_name_str, true);
            log_and_emit(pretty_ok_os_name_str.clone(), backend_communicator);
            result_string.push_str(&pretty_ok_os_name_str);
        } else {
            all_requirements_passed = false;
            let err_os_name_str = &os_info.cli_os_name;
            let pretty_err_os_name = pretty_check_string::pretty_err_str(err_os_name_str, true);
            log_and_emit(pretty_err_os_name.clone(), backend_communicator);
            result_string.push_str(&pretty_err_os_name);
        }
    }

    // Check Processor Arch eg. Arch64
    if check_processor {
        log_and_emit(
            "Checking if processor is supported".to_string(),
            backend_communicator,
        );
        let processor_info = check_specifications::get_processor_info(backend_communicator);
        if processor_info.full_architecture_supported {
            let ok_cli_arch_str = &processor_info.cli_architecture_name;
            let pretty_ok_cli_arch_str = pretty_check_string::pretty_ok_str(ok_cli_arch_str, true);
            log_and_emit(pretty_ok_cli_arch_str.clone(), backend_communicator);
            result_string.push_str(&pretty_ok_cli_arch_str);
        } else {
            all_requirements_passed = false;
            let err_cli_arch_str = format!(
                "Processor Architecture not supported. Processor = {} Bitness = {}",
                processor_info.raw_processor_brand, processor_info.bitness
            );
            let pretty_err_cli_arch_str =
                pretty_check_string::pretty_err_str(&err_cli_arch_str, true);
            log_and_emit(pretty_err_cli_arch_str.clone(), backend_communicator);
            result_string.push_str(&pretty_err_cli_arch_str);
        }
    }

    // Check Docker
    if check_docker {
        log_and_emit(
            "Checking if Docker is installed & running correctly".to_string(),
            backend_communicator,
        );
        match get_docker_status(backend_communicator) {
            Ok(docker_ok_string) => {
                let pretty_docker_ok_string =
                    pretty_check_string::pretty_ok_str(&docker_ok_string, true);
                log_and_emit(pretty_docker_ok_string.clone(), backend_communicator);
                result_string.push_str(&pretty_docker_ok_string);
            }
            Err(docker_not_ok_string) => {
                all_requirements_passed = false;
                let pretty_docker_not_ok_string =
                    pretty_check_string::pretty_err_str(&docker_not_ok_string, true);
                log_and_emit(pretty_docker_not_ok_string.clone(), backend_communicator);
                result_string.push_str(&pretty_docker_not_ok_string);
            }
        }
    }

    // Check Edge CLI Download
    if check_edge_cli_binary {
        log_and_emit(
            "Checking if Edge is downloaded correctly".to_string(),
            backend_communicator,
        );
        let is_edge_downloaded_correctly_future =
            is_edge_correctly_downloaded(backend_communicator).await;
        match is_edge_downloaded_correctly_future {
            Ok(edge_downloaded_correctly) => {
                let pretty_edge_downloaded_correctly =
                    pretty_ok_str(&edge_downloaded_correctly, false);
                log_and_emit(
                    pretty_edge_downloaded_correctly.clone(),
                    backend_communicator,
                );
                result_string.push_str(&pretty_edge_downloaded_correctly);
            }
            Err(edge_not_downloaded_correctly) => {
                all_requirements_passed = false;
                let pretty_edge_not_downloaded_correctly =
                    pretty_err_str(&edge_not_downloaded_correctly, false);
                log_and_emit(
                    pretty_edge_not_downloaded_correctly.clone(),
                    backend_communicator,
                );
                result_string.push_str(&pretty_edge_not_downloaded_correctly);
            }
        }
    }

    log_and_emit("Checked requirements.".to_string(), backend_communicator);

    let requirements_last_checked = format!(
        "Requirements last checked on: {} ",
        dt.format("%d %B %Y %H:%M:%S %Z")
    );
    log_and_emit(requirements_last_checked.clone(), backend_communicator);
    result_string.push_str(&requirements_last_checked);

    if all_requirements_passed {
        let ok_all_requirements_passed = "Passed requirements.".to_string();
        let pretty_ok_all_requirements_passed = pretty_ok_str(&ok_all_requirements_passed, false);
        log_and_emit(pretty_ok_all_requirements_passed, backend_communicator);
        Ok(result_string)
    } else {
        let err_all_requirements_passed = "Did not pass all requirements.".to_string();
        let pretty_err_all_requirements_passed =
            pretty_err_str(&err_all_requirements_passed, false);
        log_and_emit(pretty_err_all_requirements_passed, backend_communicator);
        Err(result_string)
    }
}
