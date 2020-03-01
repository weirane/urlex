use regex::Regex;
use std::io::{self, BufRead};

pub fn urlex(mut input: impl BufRead, regex: Regex) -> io::Result<Vec<String>> {
    let mut buf = String::new();
    let mut ret = Vec::new();

    loop {
        buf.clear();
        if input.read_line(&mut buf)? == 0 {
            break;
        }
        let buf = buf.trim_end();
        for m in regex.find_iter(&buf) {
            let s = m.as_str();
            ret.push(s.to_string());
        }
    }
    Ok(ret)
}
