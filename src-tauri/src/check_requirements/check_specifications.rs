use os_info;
use raw_cpuid::CpuId;
use std::collections::HashMap;

// pub struct OsInfo {
//     is_os_name_supported: bool,
//     os_name_basic: String, // eg. Windows, Mac OS
// }

pub struct ProcessorInfo {
    pub raw_processor_brand: String, // eg. GenuineIntel
    pub processor_brand_supported: bool,
    pub bitness: String, // eg. 64-bit
    pub bitness_supported: bool,
    pub full_architecture_name: String, // eg. Intel x64 or AMD arm64
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
    let supported_bitness = String::from("64-bit");

    if bitness == supported_bitness {
        return true;
    } else {
        return false;
    }
}

/// Gets OS type eg. Windows
// TODO: Recreate get os type using sysinfo lib

/// If supported, returns OS type eg. Windows
// pub fn is_os_type_supported() -> Result<String, String>{
//     let os_type = get_os_type();

//     //TODO: Add more supported OS types (eg. specific distros) https://crates.io/crates/os_info.
//     let supported_os_list = vec!["Windows", "Linux", "Mac OS"];

//     let mut supported_os_dict: HashMap<String, Option<String>> = HashMap::new();
// for os in supported_os_list{
//         supported_os_dict.insert(String::from(os), None);
//     }

//     if supported_os_dict.contains_key(&os_type){
//         return Ok(os_type)
//     }
//     else{
//         return Err(format!("Not a supported OS. Your OS = {}", os_type))
//     }
// }

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
    supported_brand_name_dict.insert(String::from("GenuineIntel"), String::from("Intel x"));
    supported_brand_name_dict.insert(String::from("AuthenticAMD"), String::from("AMD arm"));

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

pub fn get_processor_info() -> ProcessorInfo {
    let raw_processor_brand = get_raw_processor_brand();

    let mut processor_brand_supported = false;
    let mut processor_brand_simple_without_bitness = String::from("");

    match is_processor_brand_supported(raw_processor_brand.clone()) {
        Ok(supported_processor) => {
            processor_brand_supported = true;
            processor_brand_simple_without_bitness = supported_processor; // eg. Intel x
        }
        Err(not_supported_processor) => {
            processor_brand_supported = false;
            processor_brand_simple_without_bitness = not_supported_processor;
        }
    }

    let bitness = get_os_bitness();
    // assumption, the operating system bitness matches the processor bitness.
    let bitness_supported = is_bitness_supported(bitness.clone());

    let mut full_architecture_name = String::from("");
    let mut full_architecture_supported = false;
    if bitness_supported && processor_brand_supported {
        full_architecture_supported = true;
        full_architecture_name.push_str(&format!(
            "{}{}",
            &processor_brand_simple_without_bitness, &bitness
        ));
    } else {
        full_architecture_supported = false;
        full_architecture_name.push_str("Full processor architecture not recognized.")
    }

    let processor_info = ProcessorInfo {
        raw_processor_brand,
        processor_brand_supported: processor_brand_supported,
        bitness: bitness,
        bitness_supported: bitness_supported,
        full_architecture_name: full_architecture_name,
        full_architecture_supported: full_architecture_supported,
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
