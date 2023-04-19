use chrono::{DateTime, Utc};

use crate::check_requirements::{
    check_docker::get_docker_status,
    pretty_check_string::{pretty_err_str, pretty_ok_str},
};
use crate::BackendCommunicator;

mod check_docker;
pub mod check_edge;
mod check_specifications;
mod pretty_check_string;

/// Runs all system requirement checks
pub fn main(backend_communicator: BackendCommunicator) -> Result<String, String> {
    let mut result_string = String::from("");
    // TODO: Create wrapper for last checked.
    let dt: DateTime<Utc> = Utc::now();

    // Type inference lets us omit an explicit type signature (which
    // would be `HashMap<String, String>` in this example).

    let mut all_requirements_passed = true; // stays true unless at least one does not pass

    let os_info = check_specifications::get_os_info(backend_communicator.clone());
    if os_info.os_name_supported {
        result_string.push_str(&pretty_check_string::pretty_ok_str(&os_info.cli_os_name));
    } else {
        all_requirements_passed = false;
        result_string.push_str(&pretty_check_string::pretty_err_str(&os_info.cli_os_name));
    }

    // Check Processor Arch eg. Arch64
    let processor_info = check_specifications::get_processor_info(backend_communicator.clone());
    if processor_info.full_architecture_supported {
        result_string.push_str(&pretty_check_string::pretty_ok_str(
            &processor_info.cli_architecture_name,
        ))
    } else {
        all_requirements_passed = false;
        result_string.push_str(&pretty_check_string::pretty_err_str(&format!(
            "Processor Architecture not supported. Processor = {} Bitness = {}",
            processor_info.raw_processor_brand, processor_info.bitness
        )))
    }

    match get_docker_status(backend_communicator.clone()) {
        Ok(docker_ok_string) => result_string.push_str(&pretty_ok_str(&docker_ok_string)),
        Err(docker_not_ok_string) => {
            all_requirements_passed = false;
            result_string.push_str(&pretty_err_str(&docker_not_ok_string))
        }
    }

    match check_edge::is_edge_correctly_downloaded(backend_communicator.clone()) {
        Ok(edge_downloaded_correctly) => {
            result_string.push_str(&pretty_ok_str(&edge_downloaded_correctly))
        }
        Err(not_downloaded_correctly) => {
            all_requirements_passed = false;
            result_string.push_str(&pretty_err_str(&not_downloaded_correctly))
        }
    }
    // check_edge::get_edge_cli(net.clone(), os.clone(), arch.clone(), version.clone());

    // TODO: Implement more system checks.
    // Check Memory
    // Check CPU
    // Check Disks

    // OS info

    println!("Checked all requirements.");

    result_string.push_str(&format!(
        "Requirements last checked on: {} ",
        dt.format("%d %B %Y %H:%M:%S %Z").to_string()
    ));

    if all_requirements_passed {
        return Ok(result_string);
    } else {
        return Err(result_string);
    }
}
