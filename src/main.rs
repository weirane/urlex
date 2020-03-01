use regex::Regex;
use std::io::stdin;
use urlex::urlex;

const URL_REGEX: &str = r#"((https?|ftp)://[^ <>"\t]*|(www|ftp)[0-9]?\.[-a-z0-9.]+)[^ .,;\t\n\r<">\):]?[^, <>"\t]*[^ .,;\t\n\r<">\):]"#;

fn main() -> anyhow::Result<()> {
    let stdin = stdin();
    let url_regex = Regex::new(URL_REGEX).unwrap();
    for url in urlex(stdin.lock(), url_regex)? {
        println!("{}", url);
    }
    Ok(())
}
