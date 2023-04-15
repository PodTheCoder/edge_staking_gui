use chrono::{DateTime, Utc};

mod check_specifications;
mod pretty_check_string;

// Runs all requirement checks
pub fn main() -> String {
    let mut result_string = String::from("");
    // TODO: Create wrapper for last checked.
    let dt: DateTime<Utc> = Utc::now();

    // TODO: Check OS Name eg. Windows
    // Check Processor Arch eg. Arch64
    let processor_info = check_specifications::get_processor_info();
    if processor_info.full_architecture_supported {
        result_string.push_str(&pretty_check_string::pretty_ok_str(
            &processor_info.full_architecture_name,
        ))
    } else {
        result_string.push_str(&pretty_check_string::pretty_err_str(&format!(
            "Processor Architecture not supported. Processor = {} Bitness = {}",
            processor_info.raw_processor_brand, processor_info.bitness
        )))
    }

    // TODO: Check Docker
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

    return result_string;
}
