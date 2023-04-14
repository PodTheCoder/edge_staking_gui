mod check_cpu;
mod check_os;
mod pretty_check_string;

// Runs all requirement checks
pub fn main() -> String {
    // TODO: Make each string into an async event
    let mut test_string = String::from("");

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
    test_string.push_str(&format!("OS: {} | ", &os_type));
    test_string.push_str(&format!("Processor: {} | ", &cpu_arch));
    //TODO: Docker check. 
    test_string.push_str("Did you have docker installed & running? \n");

    println!("Checked all requirements.");

    return test_string

    // Check OS
    // Check Processor Arch
    // Check Docker
}