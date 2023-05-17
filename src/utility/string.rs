use std::str;

pub fn parse_delimiter(string: &str) -> (&str, &str) {
    let parsed = string.splitn(2, "|").collect::<Vec<&str>>();
    let pred_delimiter = *parsed.first().expect("no callback data found");
    let post_delimiter = *parsed.get(1).unwrap_or(&"");

    (pred_delimiter, post_delimiter)
}

pub fn truncate_last_delimiter(string: &str) -> String {
    let mut parsed = string.split("|").collect::<Vec<&str>>();
    parsed.truncate(parsed.len() - 1);
    parsed.join("|")
}
