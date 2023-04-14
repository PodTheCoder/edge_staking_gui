use os_info::{self, Info};
use std::collections::HashMap;

use crate::get_cpu_info;

fn get_full_os_info() -> Info{
    let info = os_info::get();
    return info
}

/// Gets bitness eg. 64
fn get_os_bitness() -> String{
    let full_os_info = &get_full_os_info();
    let os_bitness_str = String::from(format!("{}", full_os_info.bitness()));
    return os_bitness_str
}

/// Gets OS type eg. Windows
fn get_os_type() -> String{
    let full_os_info = &get_full_os_info();
    let os_type_str = String::from(format!("{}", full_os_info.os_type()));
    return os_type_str
}

/// Check OS type eg. Windows
pub fn check_os_type() -> Result<String, String>{
    let os_type = get_os_type();

    //TODO: Add more supported OS types (eg. specific distros) https://crates.io/crates/os_info.
    let supported_os_list = vec!["Windows", "Linux", "Mac OS"];
    
    let mut supported_os_dict: HashMap<String, Option<String>> = HashMap::new();

    for os in supported_os_list{
        supported_os_dict.insert(String::from(os), None);
    }

    if supported_os_dict.contains_key(&os_type){
        return Ok(os_type)
    }
    else{
        return Err(format!("Not a supported OS. Your OS = {}", os_type))
    }
}


/// Check processor architecture with bitness. eg. arm64
pub fn check_processor_architecture() -> Result<String, String> {    
    let processor_brand = get_cpu_info::main();

    match processor_brand{
        Ok(processor_brand) => {
            let mut processor_architecture_str = String::from(processor_brand);
            let bitness_str = get_os_bitness();
            if !(bitness_str == "64-bit"){
                return Err(format!("Not a 64 bit processor. Your bitness = {}", bitness_str))
            }

            processor_architecture_str.push_str(&bitness_str);

            return Ok(processor_architecture_str)
        }
        Err(err) => {return Err(err)}
    }
}