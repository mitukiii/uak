use std::error::Error;
use std::io::Read;
use std::{env, fs::File};
use std::{fs, io};

use regex::Regex;

static USAGE: &str = "Usage: uak <authorized_keys path> <remote authorized_keys url>";

static HEADER: &str = "\n# UAK BEGIN: The following lines is added by uak.\n\n";
static FOOTER: &str = "\n\n# UAK END: The above lines is added by uak.\n";

type Result<T, E = Box<dyn Error + Send + Sync + 'static>> = core::result::Result<T, E>;

#[async_std::main]
async fn main() -> Result<()> {
    let authorized_keys_path = env::args().nth(1).ok_or(USAGE)?;
    let remote_authorized_keys_url = env::args().nth(2).ok_or(USAGE)?;
    let current_content = read_file(&authorized_keys_path)?;
    let content_to_add = read_url(&remote_authorized_keys_url).await?;
    let edited_content = inject(&current_content, &content_to_add);
    write_file(&authorized_keys_path, &edited_content)?;
    Ok(())
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

async fn read_url(url: &str) -> Result<String> {
    let mut res = surf::get(url).await?;
    let status = res.status().as_u16();
    let body = res.body_string().await?;
    if status == 200 {
        Ok(body)
    } else {
        let message = format!("status = {}", status);
        Err(error(io::ErrorKind::InvalidData, &message))
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

fn error(kind: io::ErrorKind, message: &str) -> Box<io::Error> {
    Box::new(io::Error::new(kind, message))
}
