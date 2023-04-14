use chrono::{Utc, DateTime};

mod check_cpu;
mod check_os;
mod pretty_check_string;

// Runs all requirement checks
pub fn main() -> String {
    // TODO: Make each string into an async event
    let mut result_string = String::from("");
    // TODO: Create wrapper for last checked.
    let dt: DateTime<Utc> = Utc::now();

    let mut cpu_arch = String::from("");
    match check_cpu::check_processor_architecture(){
        Ok(found_cpu_arch) => {
            let ok_cpu = pretty_check_string::pretty_ok_str(&found_cpu_arch);
            cpu_arch.push_str(&ok_cpu)}
        Err(not_found_cpu_arch) => {
            let err_cpu = pretty_check_string::pretty_err_str(&not_found_cpu_arch);
            cpu_arch.push_str(&err_cpu)}
    }

    let mut os_type= String::from("");
    match check_os::check_os_type(){
        Ok(found_os) => {
            let ok_os = pretty_check_string::pretty_ok_str(&found_os);
            os_type.push_str(&ok_os)
        }
        Err(err_os) => {
            let err_os = pretty_check_string::pretty_err_str(&err_os);
            os_type.push_str(&err_os)
        }
    }

    // Check System Infrastructure
    result_string.push_str(&format!("OS: {} | ", &os_type));
    result_string.push_str(&format!("Processor: {} | ", &cpu_arch));
    //TODO: Docker check. 
    result_string.push_str("DO you have Docker installed & running? Docker can take a while to start up.");

    println!("Checked all requirements.");

    result_string.push_str(&format!("Requirements last checked on: {} ", dt.format("%d %B %Y %H:%M:%S %Z").to_string()));
    return result_string

    // Check OS
    // Check Processor Arch
    // Check Docker
}