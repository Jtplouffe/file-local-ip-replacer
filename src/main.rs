use clap::Parser;
use lazy_static::lazy_static;
use local_ip_address::local_ip;
use regex::Regex;

use crate::cli_args::CliArgs;

mod cli_args;

lazy_static! {
    static ref IP_REGEX: Regex = Regex::new(
        r"(\b25[0-5]|\b2[0-4][0-9]|\b[01]?[0-9][0-9]?)(\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)){3}"
    )
    .unwrap();
}

const LOCALHOST: &str = "localhost";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = CliArgs::parse();

    let file_content = std::fs::read_to_string(&args.path)?;
    let local_ip = local_ip()?.to_string();

    let mut modified_file_content = IP_REGEX.replace_all(&file_content, &local_ip).to_string();
    if args.replace_localhost {
        modified_file_content = modified_file_content.replace(LOCALHOST, &local_ip);
    }

    std::fs::write(&args.path, modified_file_content)?;

    Ok(())
}
