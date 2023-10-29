use crate::BackendCommunicator;

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

pub struct OsSimpleStrings {
    pub windows: String,
    pub macos: String,
    pub linux: String,
}

/// Gets the simple os strings used in the url path eg. windows. Useful for describing options.
pub fn get_os_simple_strings() -> OsSimpleStrings {
    let windows_simple_string = "windows".to_string();
    let linux_simple_string = "linux".to_string();
    let macos_simple_string = "macos".to_string();

    OsSimpleStrings {
        windows: (windows_simple_string),
        macos: (macos_simple_string),
        linux: (linux_simple_string),
    }
}

/// Gets bitness eg. 64
fn get_os_bitness() -> String {
    let info = os_info::get();
    let os_bitness_str = format!("{}", info.bitness());
    os_bitness_str
}

/// Check if the bitness is supported
fn is_bitness_supported(bitness: String) -> bool {
    let supported_bitness = String::from("64");

    bitness == supported_bitness
}

/// Gets OS name eg. Windows
fn get_raw_os_name() -> String {
    let info = os_info::get();
    let os_name = format!("{}", info.os_type());
    os_name
}
/// If supported, returns OS type eg. Windows
fn is_os_name_supported(os_type: String) -> Result<String, String> {
    // Simple strings are used for downloading the correct Edge binary
    let simple_os_strings = get_os_simple_strings();

    let windows_simple_string = simple_os_strings.windows;
    let linux_simple_string = simple_os_strings.linux;
    let macos_simple_string = simple_os_strings.macos;

    // Assumption is that the Edge Binary functions the same on all linux distributions.

    let supported_os_dict: HashMap<String, String> = HashMap::from([
        ("Alpaquita".to_string(), linux_simple_string.clone()),
        ("Alpine".to_string(), linux_simple_string.clone()),
        ("Amazon".to_string(), linux_simple_string.clone()),
        ("Arch".to_string(), linux_simple_string.clone()),
        ("Artix".to_string(), linux_simple_string.clone()),
        ("CentOS".to_string(), linux_simple_string.clone()),
        ("Debian".to_string(), linux_simple_string.clone()),
        ("DragonFly".to_string(), linux_simple_string.clone()),
        ("Emscripten".to_string(), linux_simple_string.clone()),
        ("EndeavourOS".to_string(), linux_simple_string.clone()),
        ("Fedora".to_string(), linux_simple_string.clone()),
        ("FreeBSD".to_string(), linux_simple_string.clone()),
        ("Garuda".to_string(), linux_simple_string.clone()),
        ("Gentoo".to_string(), linux_simple_string.clone()),
        ("HardenedBSD".to_string(), linux_simple_string.clone()),
        ("Illumos".to_string(), linux_simple_string.clone()),
        ("Linux".to_string(), linux_simple_string.clone()),
        ("Mabox".to_string(), linux_simple_string.clone()),
        ("Mac OS".to_string(), macos_simple_string),
        ("Manjaro".to_string(), linux_simple_string.clone()),
        ("Mariner".to_string(), linux_simple_string.clone()),
        ("MidnightBSD".to_string(), linux_simple_string.clone()),
        ("Mint".to_string(), linux_simple_string.clone()),
        ("NetBSD".to_string(), linux_simple_string.clone()),
        ("NixOS".to_string(), linux_simple_string.clone()),
        ("OpenBSD".to_string(), linux_simple_string.clone()),
        ("OpenCloudOS".to_string(), linux_simple_string.clone()),
        ("openEuler".to_string(), linux_simple_string.clone()),
        ("openSUSE".to_string(), linux_simple_string.clone()),
        ("OracleLinux".to_string(), linux_simple_string.clone()),
        ("Pop".to_string(), linux_simple_string.clone()),
        ("Raspbian".to_string(), linux_simple_string.clone()),
        ("Redhat".to_string(), linux_simple_string.clone()),
        ("RedHatEnterprise".to_string(), linux_simple_string.clone()),
        ("Redox".to_string(), linux_simple_string.clone()),
        ("Solus".to_string(), linux_simple_string.clone()),
        ("SUSE".to_string(), linux_simple_string.clone()),
        ("Ubuntu".to_string(), linux_simple_string),
        ("Windows".to_string(), windows_simple_string),
    ]);

    match supported_os_dict.get(&os_type) {
        Some(simple_name_value) => {
            let cli_os_name = String::from(simple_name_value);
            Ok(cli_os_name)
        }
        None => {
            let error_str = format!("OS not recognized. Your OS: {}", os_type);
            Err(error_str)
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

    OsInfo {
        raw_os_name,
        os_name_supported,
        cli_os_name,
    }
}

/// Gets the raw brand name of the processor. eg. GenuineIntel
fn get_raw_processor_brand() -> String {
    let cpuid = CpuId::new();

    if let Some(vf) = cpuid.get_vendor_info() {
        let processor_brand = String::from(vf.as_str());
        processor_brand
    } else {
        String::from("No processor recognized.")
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
            Ok(simple_brand_name)
        }
        None => {
            let error_str = format!(
                "Processor brand not recognized. Your processor brand: {}",
                raw_processor_brand
            );
            Err(error_str)
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

    ProcessorInfo {
        raw_processor_brand,
        processor_brand_supported,
        bitness,
        bitness_supported,
        cli_architecture_name: full_architecture_name,
        full_architecture_supported,
    }
}
