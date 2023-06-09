/// Prepend string with nice green checkmark
pub fn pretty_ok_str(ok_str: &str, add_v_bar: bool) -> String {
    let mut prettified_string = String::from("✔️");
    prettified_string.push_str(ok_str);
    if add_v_bar {
        prettified_string.push_str(" | ");
    }
    prettified_string
}

/// Prpeend string with sad big red cross
pub fn pretty_err_str(err_str: &str, add_v_bar: bool) -> String {
    let mut prettified_string = String::from("❌");
    prettified_string.push_str(err_str);
    if add_v_bar {
        prettified_string.push_str(" | ");
    }
    prettified_string
}
