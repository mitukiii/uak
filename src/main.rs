use std::error::Error;
use std::io::Read;
use std::{env, fs::File};
use std::{fs, io};

use regex::Regex;
use reqwest::StatusCode;

static USAGE: &str = "Usage: uak <authorized keys path> <public keys url>";

static HEADER: &str = "\n# UAK BEGIN: The following lines is added by uak.\n\n";
static FOOTER: &str = "\n\n# UAK END: The above lines is added by uak.\n";

type Result<T, E = Box<dyn Error + Send + Sync + 'static>> = core::result::Result<T, E>;

fn main() {
    let authorized_keys_path = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("{}", USAGE);
        std::process::exit(1);
    });
    let public_keys_url = env::args().nth(2).unwrap_or_else(|| {
        eprintln!("{}", USAGE);
        std::process::exit(1);
    });

    run(&authorized_keys_path, &public_keys_url).unwrap_or_else(|err| {
        eprintln!("uak: {}", err);
        std::process::exit(1);
    });
}

fn run(authorized_keys_path: &str, public_keys_url: &str) -> Result<()> {
    let current_content = read_file(authorized_keys_path)?;
    let content_to_add = read_url(public_keys_url)?;
    let edited_content = inject(&current_content, &content_to_add);
    write_file(authorized_keys_path, &edited_content)
}

fn read_file(path: &str) -> Result<String> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn write_file(path: &str, content: &str) -> Result<()> {
    fs::write(path, content)?;
    Ok(())
}

fn read_url(url: &str) -> Result<String> {
    let mut content = String::new();
    let mut response = reqwest::blocking::get(url)?;
    if response.status() == StatusCode::OK {
        response.read_to_string(&mut content)?;
        Ok(content)
    } else {
        Err(Box::new(io::Error::new(
            io::ErrorKind::Other,
            format!("{}", response.status()),
        )))
    }
}

fn inject(current_content: &str, content_to_add: &str) -> String {
    let regex_string = format!("(?ms){}.+{}", HEADER, FOOTER);
    let regex = Regex::new(regex_string.as_str()).unwrap();

    if regex.is_match(current_content) {
        regex
            .replace_all(current_content, [HEADER, content_to_add, FOOTER].concat())
            .to_string()
    } else {
        [current_content, HEADER, content_to_add, FOOTER].concat()
    }
}
