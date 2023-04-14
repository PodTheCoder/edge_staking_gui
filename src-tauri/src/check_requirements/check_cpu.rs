use raw_cpuid::CpuId;
use std::collections::HashMap;

use crate::check_requirements::check_os;

/// Checks if processor brand is supported, and converts to a more accessible naming
fn check_if_processor_supported(processor_brand: String) -> Result<String, String> {
    let mut supported_brand_name_dict: HashMap<String, String> = HashMap::new();
    supported_brand_name_dict.insert(String::from("GenuineIntel"), String::from("Intel x"));
    supported_brand_name_dict.insert(String::from("AuthenticAMD") , String::from("AMD arm"));

    match supported_brand_name_dict.get(&processor_brand) {
        Some(simple_name_value) => {
            let simple_brand_name = String::from(simple_name_value);
            return Ok(simple_brand_name)
        }
        None => {
            let error_str = String::from(format!("Processor brand not recognized. Your processor brand: {}", processor_brand));
            return Err(error_str)
        }
    }
}

/// Gets the name of the processor. Can be complex. 
fn get_processor_brand() -> Result<String,String> {
    let cpuid = CpuId::new();

    if let Some(vf) = cpuid.get_vendor_info() {
        let processor_brand = String::from(vf.as_str()); 
        return Ok(processor_brand)
    }
    let error_str = String::from("No processor recognized.");
    return Err(error_str)
    

}

/// If found, returns a pretty processor name with space for bitness.
pub(crate) fn check_processor() -> Result<String,String> {
    match get_processor_brand() {
        Ok(brand_str) => {
            let processor_brand = brand_str;
            
            match check_if_processor_supported(processor_brand){
                Ok(simple_brand_str) => {return Ok(simple_brand_str)}
                Err(err) => {return Err(err)}
            }
        }
        Err(err) => return Err(err)
    }
}

/// If valid, return processor architecture with bitness. eg. arm64
pub fn check_processor_architecture() -> Result<String, String> {    
    match check_processor(){
        Ok(processor_brand) => {
            let mut processor_architecture_str = String::from(processor_brand);

            match check_os::check_bitness(){
                Ok(bitness) => {processor_architecture_str.push_str(&bitness)}
                Err(err) => {return Err(err)}
            }


            return Ok(processor_architecture_str)
        }
        Err(err) => {return Err(err)}
    }
}