// Gets the string the way it is parsed.
pub fn make_string(src_string: &str) -> String {
    let mut output_string = "".to_string();

    // Triming the double quotes at the begining.
    if src_string.starts_with("\"") && src_string.ends_with("\"") {
        output_string = src_string[1..src_string.len() - 1].to_string();
    }
    output_string = output_string.replace("\\n", "\n");
    output_string = output_string.replace("\\t", "\t");
    output_string = output_string.replace("\\r", "\r");
    output_string = output_string.replace("\x5c\x22", "\"");

    return output_string;
}
