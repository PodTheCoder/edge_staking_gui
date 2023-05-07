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

/// Gets bitness eg. 64
fn get_os_bitness() -> String {
    let info = os_info::get();
    let os_bitness_str = String::from(format!("{}", info.bitness()));
    return os_bitness_str;
}

/// Check if the bitness is supported
fn is_bitness_supported(bitness: String) -> bool {
    let supported_bitness = String::from("64");

    if bitness == supported_bitness {
        return true;
    } else {
        return false;
    }
}

/// Gets OS name eg. Windows
fn get_raw_os_name() -> String {
    let info = os_info::get();
    let os_name = String::from(format!("{}", info.os_type()));
    return os_name;
}
/// If supported, returns OS type eg. Windows
fn is_os_name_supported(os_type: String) -> Result<String, String> {
    // Simple strings are used for downloading the correct Edge binary
    let windows_simple_string = format!("windows");
    let linux_simple_string = format!("linux");
    let macos_simple_string = format!("macos");

    // Assumption is that the Edge Binary functions the same on all linux distributions.

    let supported_os_dict: HashMap<String, String> = HashMap::from([
        (format!("Alpaquita"), linux_simple_string.clone()),
        (format!("Alpine"), linux_simple_string.clone()),
        (format!("Amazon"), linux_simple_string.clone()),
        (format!("Arch"), linux_simple_string.clone()),
        (format!("Artix"), linux_simple_string.clone()),
        (format!("CentOS"), linux_simple_string.clone()),
        (format!("Debian"), linux_simple_string.clone()),
        (format!("DragonFly"), linux_simple_string.clone()),
        (format!("Emscripten"), linux_simple_string.clone()),
        (format!("EndeavourOS"), linux_simple_string.clone()),
        (format!("Fedora"), linux_simple_string.clone()),
        (format!("FreeBSD"), linux_simple_string.clone()),
        (format!("Garuda"), linux_simple_string.clone()),
        (format!("Gentoo"), linux_simple_string.clone()),
        (format!("HardenedBSD"), linux_simple_string.clone()),
        (format!("Illumos"), linux_simple_string.clone()),
        (format!("Linux"), linux_simple_string.clone()),
        (format!("Mabox"), linux_simple_string.clone()),
        (format!("Mac OS"), macos_simple_string),
        (format!("Manjaro"), linux_simple_string.clone()),
        (format!("Mariner"), linux_simple_string.clone()),
        (format!("MidnightBSD"), linux_simple_string.clone()),
        (format!("Mint"), linux_simple_string.clone()),
        (format!("NetBSD"), linux_simple_string.clone()),
        (format!("NixOS"), linux_simple_string.clone()),
        (format!("OpenBSD"), linux_simple_string.clone()),
        (format!("OpenCloudOS"), linux_simple_string.clone()),
        (format!("openEuler"), linux_simple_string.clone()),
        (format!("openSUSE"), linux_simple_string.clone()),
        (format!("OracleLinux"), linux_simple_string.clone()),
        (format!("Pop"), linux_simple_string.clone()),
        (format!("Raspbian"), linux_simple_string.clone()),
        (format!("Redhat"), linux_simple_string.clone()),
        (format!("RedHatEnterprise"), linux_simple_string.clone()),
        (format!("Redox"), linux_simple_string.clone()),
        (format!("Solus"), linux_simple_string.clone()),
        (format!("SUSE"), linux_simple_string.clone()),
        (format!("Ubuntu"), linux_simple_string.clone()),
        (format!("Windows"), windows_simple_string),
    ]);

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

/// Get Operating System info
pub fn get_os_info(_backend_communicator: &BackendCommunicator) -> OsInfo {
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
pub fn get_processor_info(_backend_communicator: &BackendCommunicator) -> ProcessorInfo {
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
