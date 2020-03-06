#![cfg(feature = "cli")]
use regex::Regex;
use std::io::stdin;
use structopt::StructOpt;
use urlex::urlex;

const URL_REGEX: &str = r#"((https?|ftp)://[^ <>"\t]*|(www|ftp)[0-9]?\.[-a-z0-9.]+)[^ .,;\t\n\r<">\):]?[^, <>"\t]*[^ .,;\t\n\r<">\):]"#;

#[derive(Debug, StructOpt)]
struct Opt {
    /// Reverse the output
    #[structopt(short, long)]
    reverse: bool,

    /// The regex used for extracting
    #[structopt(short = "x", long, default_value = URL_REGEX)]
    regex: String,
}

fn main() -> anyhow::Result<()> {
    let opt = Opt::from_args();
    let url_regex = Regex::new(&opt.regex)?;

    let stdin = stdin();
    let urls = urlex(stdin.lock(), url_regex)?;

    if opt.reverse {
        for url in urls.iter().rev() {
            println!("{}", url);
        }
    } else {
        for url in urls {
            println!("{}", url);
        }
    }
    Ok(())
}
