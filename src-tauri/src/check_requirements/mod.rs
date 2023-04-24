use chrono::{DateTime, Utc};

use crate::docker::get_docker_status;
use crate::BackendCommunicator;
use crate::{
    check_requirements::pretty_check_string::{pretty_err_str, pretty_ok_str},
    utility::log_and_emit,
};

use self::check_edge::is_edge_correctly_downloaded;

mod check_docker;
pub mod check_edge;
mod check_specifications;
mod pretty_check_string;

/// Runs all system requirement checks
pub async fn main(backend_communicator: BackendCommunicator) -> Result<String, String> {
    let mut result_string = String::from("");
    let dt: DateTime<Utc> = Utc::now();

    // Type inference lets us omit an explicit type signature (which
    // would be `HashMap<String, String>` in this example).

    let mut all_requirements_passed = true; // stays true unless at least one does not pass

    // Check OS
    log_and_emit(
        format!("Checking if Operating System (OS) is supported"),
        backend_communicator.clone(),
    );
    let os_info = check_specifications::get_os_info(backend_communicator.clone());
    if os_info.os_name_supported {
        let ok_os_name_str = &os_info.cli_os_name;
        let pretty_ok_os_name_str = pretty_check_string::pretty_ok_str(ok_os_name_str);
        log_and_emit(pretty_ok_os_name_str.clone(), backend_communicator.clone());
        result_string.push_str(&pretty_ok_os_name_str);
    } else {
        all_requirements_passed = false;
        let err_os_name_str = &os_info.cli_os_name;
        let pretty_err_os_name = pretty_check_string::pretty_err_str(err_os_name_str);
        log_and_emit(pretty_err_os_name.clone(), backend_communicator.clone());
        result_string.push_str(&pretty_err_os_name);
    }

    // Check Processor Arch eg. Arch64
    log_and_emit(
        format!("Checking if processor is supported"),
        backend_communicator.clone(),
    );
    let processor_info = check_specifications::get_processor_info(backend_communicator.clone());
    if processor_info.full_architecture_supported {
        let ok_cli_arch_str = &processor_info.cli_architecture_name;
        let pretty_ok_cli_arch_str = pretty_check_string::pretty_ok_str(ok_cli_arch_str);
        log_and_emit(pretty_ok_cli_arch_str.clone(), backend_communicator.clone());
        result_string.push_str(&pretty_ok_cli_arch_str);
    } else {
        all_requirements_passed = false;
        let err_cli_arch_str = format!(
            "Processor Architecture not supported. Processor = {} Bitness = {}",
            processor_info.raw_processor_brand, processor_info.bitness
        );
        let pretty_err_cli_arch_str = pretty_check_string::pretty_err_str(&err_cli_arch_str);
        log_and_emit(
            pretty_err_cli_arch_str.clone(),
            backend_communicator.clone(),
        );
        result_string.push_str(&pretty_err_cli_arch_str);
    }

    // Check Docker
    log_and_emit(
        format!("Checking if Docker is installed & running correctly"),
        backend_communicator.clone(),
    );
    match get_docker_status(backend_communicator.clone()) {
        Ok(docker_ok_string) => {
            let pretty_docker_ok_string = pretty_check_string::pretty_ok_str(&docker_ok_string);
            log_and_emit(
                pretty_docker_ok_string.clone(),
                backend_communicator.clone(),
            );
            result_string.push_str(&pretty_docker_ok_string);
        }
        Err(docker_not_ok_string) => {
            all_requirements_passed = false;
            let pretty_docker_not_ok_string =
                pretty_check_string::pretty_err_str(&docker_not_ok_string);
            log_and_emit(
                pretty_docker_not_ok_string.clone(),
                backend_communicator.clone(),
            );
            result_string.push_str(&pretty_docker_not_ok_string);
        }
    }

    // Check Edge CLI Download
    log_and_emit(
        format!("Checking if Edge is downloaded correctly"),
        backend_communicator.clone(),
    );
    let is_edge_downloaded_correctly_future =
        is_edge_correctly_downloaded(backend_communicator.clone()).await;
    match is_edge_downloaded_correctly_future {
        Ok(edge_downloaded_correctly) => {
            let pretty_edge_downloaded_correctly = pretty_ok_str(&edge_downloaded_correctly);
            log_and_emit(
                pretty_edge_downloaded_correctly.clone(),
                backend_communicator.clone(),
            );
            result_string.push_str(&pretty_edge_downloaded_correctly);
        }
        Err(edge_not_downloaded_correctly) => {
            all_requirements_passed = false;
            let pretty_edge_not_downloaded_correctly =
                pretty_err_str(&edge_not_downloaded_correctly.clone());
            log_and_emit(
                pretty_edge_not_downloaded_correctly.clone(),
                backend_communicator.clone(),
            );
            result_string.push_str(&pretty_edge_not_downloaded_correctly);
        }
    }
    // check_edge::get_edge_cli(net.clone(), os.clone(), arch.clone(), version.clone());

    // TODO: Implement more system checks.
    // Check Memory
    // Check CPU
    // Check Disks

    // OS info

    log_and_emit(
        format!("Checked all requirements."),
        backend_communicator.clone(),
    );

    let requirements_last_checked = format!(
        "Requirements last checked on: {} ",
        dt.format("%d %B %Y %H:%M:%S %Z").to_string()
    );
    log_and_emit(
        requirements_last_checked.clone(),
        backend_communicator.clone(),
    );
    result_string.push_str(&requirements_last_checked);

    if all_requirements_passed {
        let ok_all_requirements_passed = format!("Passed all requirements.");
        let pretty_ok_all_requirements_passed = pretty_ok_str(&ok_all_requirements_passed);
        log_and_emit(
            pretty_ok_all_requirements_passed.clone(),
            backend_communicator.clone(),
        );
        return Ok(result_string);
    } else {
        let err_all_requirements_passed = format!("Did not pass all requirements.");
        let pretty_err_all_requirements_passed = pretty_err_str(&err_all_requirements_passed);
        log_and_emit(
            pretty_err_all_requirements_passed.clone(),
            backend_communicator.clone(),
        );
        return Err(result_string);
    }
}
