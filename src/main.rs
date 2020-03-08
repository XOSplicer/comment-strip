mod config;

use crate::config::Config;
use clap::{load_yaml, App};
use comment_strip::{strip_comments, AppError};
use std::io::{BufReader, BufWriter, Read, Write};

fn main() -> Result<(), AppError> {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let config = Config::from_matches(&matches)?;

    let mut data = String::new();
    let mut br = BufReader::new(config.input);
    br.read_to_string(&mut data)?;

    let stripped = strip_comments(data, config.style, config.remove_blanks)?;

    let mut bw = BufWriter::new(config.output);
    bw.write_all(stripped.as_bytes())?;

    Ok(())
}
