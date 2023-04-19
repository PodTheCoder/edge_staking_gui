use crate::BackendCommunicator;
use os_info;
use raw_cpuid::CpuId;
use std::collections::HashMap;

pub struct OsInfo {
    pub raw_os_name: String, // eg. Windows
    pub os_name_supported: bool,
    pub cli_os_name: String, // eg. windows, matches CLI naming
}

pub struct ProcessorInfo {
    pub raw_processor_brand: String, // eg. GenuineIntel
    pub processor_brand_supported: bool,
    pub bitness: String, // eg. 64
    pub bitness_supported: bool,
    pub cli_architecture_name: String, // eg. x64 or arm64
    pub full_architecture_supported: bool,
}

// TODO: Find bitness with sysinfo library
/// Gets bitness eg. 64
fn get_os_bitness() -> String {
    let info = os_info::get();
    let os_bitness_str = String::from(format!("{}", info.bitness()));
    return os_bitness_str;
}

fn is_bitness_supported(bitness: String) -> bool {
    let supported_bitness = String::from("64");

    if bitness == supported_bitness {
        return true;
    } else {
        return false;
    }
}

// TODO: Recreate get os type using sysinfo lib
/// Gets OS name eg. Windows
fn get_raw_os_name() -> String {
    let info = os_info::get();
    let os_name = String::from(format!("{}", info.os_type()));
    return os_name;
}
/// If supported, returns OS type eg. Windows
fn is_os_name_supported(os_type: String) -> Result<String, String> {
    //TODO: Add more supported OS types (eg. specific distros) https://crates.io/crates/os_info.
    let mut supported_os_dict: HashMap<String, String> = HashMap::new();
    supported_os_dict.insert(String::from("Windows"), String::from("windows"));
    supported_os_dict.insert(String::from("Linux"), String::from("linux"));
    supported_os_dict.insert(String::from("Mac OS"), String::from("macos"));

    match supported_os_dict.get(&os_type) {
        Some(simple_name_value) => {
            let cli_os_name = String::from(simple_name_value);
            return Ok(cli_os_name);
        }
        None => {
            let error_str = String::from(format!("OS not recognized. Your OS: {}", os_type));
            return Err(error_str);
        }
    }
}

pub fn get_os_info(_backend_communicator: BackendCommunicator) -> OsInfo {
    let raw_os_name = get_raw_os_name();
    let os_name_supported: bool;
    let cli_os_name: String;

    match is_os_name_supported(raw_os_name.clone()) {
        Ok(simple_os_name) => {
            cli_os_name = simple_os_name; // eg. Windows will have become windows to match the CLI
            os_name_supported = true;
        }
        Err(wrong_os_name) => {
            cli_os_name = wrong_os_name;
            os_name_supported = false;
        }
    }

    let os_info = OsInfo {
        raw_os_name,
        os_name_supported,
        cli_os_name,
    };

    return os_info;
}
//
// get raw brand, check if raw brand supported, convert raw to simple brand, get bitness, check bitness supported, compose full_arch name

/// Gets the raw brand name of the processor. eg. GenuineIntel
fn get_raw_processor_brand() -> String {
    let cpuid = CpuId::new();

    if let Some(vf) = cpuid.get_vendor_info() {
        let processor_brand = String::from(vf.as_str());
        return processor_brand;
    } else {
        let error_str = String::from("No processor recognized.");
        return error_str;
    }
}

/// Checks if processor brand is supported, and converts to a more accessible naming with space for bitness
fn is_processor_brand_supported(raw_processor_brand: String) -> Result<String, String> {
    let mut supported_brand_name_dict: HashMap<String, String> = HashMap::new();
    supported_brand_name_dict.insert(String::from("GenuineIntel"), String::from("x"));
    supported_brand_name_dict.insert(String::from("AuthenticAMD"), String::from("arm"));

    match supported_brand_name_dict.get(&raw_processor_brand) {
        Some(simple_name_value) => {
            let simple_brand_name = String::from(simple_name_value);
            return Ok(simple_brand_name);
        }
        None => {
            let error_str = String::from(format!(
                "Processor brand not recognized. Your processor brand: {}",
                raw_processor_brand
            ));
            return Err(error_str);
        }
    }
}

/// Gets info about user's processor. eg. bitness = 86, raw_processor_brand = GenuineIntel
pub fn get_processor_info(_backend_communicator: BackendCommunicator) -> ProcessorInfo {
    let raw_processor_brand = get_raw_processor_brand();

    let processor_brand_supported;
    let processor_brand_simple_without_bitness;

    match is_processor_brand_supported(raw_processor_brand.clone()) {
        Ok(supported_processor) => {
            processor_brand_supported = true;
            processor_brand_simple_without_bitness = supported_processor; // eg. arm
        }
        Err(not_supported_processor) => {
            processor_brand_supported = false;
            processor_brand_simple_without_bitness = not_supported_processor;
        }
    }

    let bitness = get_os_bitness();
    let bitness_truncated = String::from(&bitness[0..2]);
    // assumption, the operating system bitness matches the processor bitness.
    let bitness_supported = is_bitness_supported(bitness_truncated.clone());

    let full_architecture_supported;
    let full_architecture_name: String;
    if bitness_supported && processor_brand_supported {
        full_architecture_supported = true;
        full_architecture_name = String::from(&format!(
            "{}{}",
            &processor_brand_simple_without_bitness, bitness_truncated
        ));
    } else {
        full_architecture_supported = false;
        full_architecture_name = String::from("Full processor architecture not recognized.");
    }

    let processor_info = ProcessorInfo {
        raw_processor_brand,
        processor_brand_supported,
        bitness,
        bitness_supported,
        cli_architecture_name: full_architecture_name,
        full_architecture_supported,
    };
    return processor_info;
}

// TODO: Check System Infrastructure using sysinfo

// use sysinfo::{CpuExt, CpuRefreshKind, DiskExt, RefreshKind, System, SystemExt};
// Min specs of Edge:

// CPU: 1x quad-core 1.2 GHZ+
// RAM: 1GB RAM
// Disk: 50GB HDD+
// result_string.push_str(&format!("OS: {} | ", &os_type));
// result_string.push_str(&format!(
//     "Requirements last checked on: {} ",
//     dt.format("%d %B %Y %H:%M:%S %Z").to_string()
// ));

// // Testing sysinfo
// // Please note that we use "new_all" to ensure that all list of
// // components, network interfaces, disks and users are already
// // filled!

// // Choose what to refresh
// let refresh_kind_specific = RefreshKind::new(); // starts with all flags disabled

// let refresh_kind_specific = refresh_kind_specific.with_disks();
// let refresh_kind_specific = refresh_kind_specific.with_disks_list();
// let refresh_kind_specific = refresh_kind_specific.with_memory();

// let cpu_refresh_kind_specific = CpuRefreshKind::new();
// let cpu_refresh_kind_specific = cpu_refresh_kind_specific.with_frequency();

// let refresh_kind_specific = refresh_kind_specific.with_cpu(cpu_refresh_kind_specific);

// let mut sys = System::new_with_specifics(refresh_kind_specific);

// sys.refresh_specifics(refresh_kind_specific);

// // We display all disks' information:
// println!("=> disks:");
// for disk in sys.disks() {
//     println!("{:?}", disk.name());
//     println!("{:?}", disk.type_());
//     println!("{}", disk.available_space());
// }

// // RAM information:
// println!("=> memory:");
// println!("total RAM memory: {} bytes", sys.total_memory());

// // CPU information (lists each core)
// // TODO: Check how many cores meet the requirement.
// println!("=> cpu");
// for cpu in sys.cpus() {
//     println!("{}", cpu.vendor_id()); //TODO: Check if all same vendor. Then convert.
//     println!("{}", cpu.frequency()); // in MegaHertz (Ghz * 1000)
// }

// // Number of CPUs:
// println!("NB CPUs: {}", sys.cpus().len());

// // Display system information:
// println!("OS name: {:?}", sys.name());
// println!("System OS version: {:?}", sys.os_version());
