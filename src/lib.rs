use regex::Regex;
use std::io::{self, BufRead};

/// Extracts a pattern specified by `regex` from `input`.
pub fn urlex(mut input: impl BufRead, regex: Regex) -> io::Result<Vec<String>> {
    let mut buf = String::new();
    let mut ret = Vec::new();

    loop {
        buf.clear();
        if input.read_line(&mut buf)? == 0 {
            break;
        }
        ret.extend(
            regex
                .find_iter(buf.trim_end())
                .map(|m| m.as_str().to_string()),
        );
    }
    Ok(ret)
}
