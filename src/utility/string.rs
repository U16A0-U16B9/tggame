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

pub fn find_username(string: &str) -> Option<String> {
    let start = string.find('@');
    if let None = start {
        return None;
    }
    let str_from_start: &str = string.get(start.unwrap() + 1..).unwrap();
    let end = str_from_start.find(' ');
    if let None = end {
        return Some(str_from_start.to_string());
    }

    Some(str_from_start.get(..end.unwrap()).unwrap().to_string())
}
