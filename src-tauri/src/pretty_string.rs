
/// Prepend string with nice green checkmark
pub fn ok_str(ok_str : &String) -> String{
    let mut prettified_string = String::from("✔️");
    prettified_string.push_str(&ok_str);
    return prettified_string
}

/// Prpeend string with sad big red cross
pub fn err_str(err_str : &String) -> String{
    let mut prettified_string = String::from("❌");
    prettified_string.push_str(&err_str);
    return prettified_string
}
