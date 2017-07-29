#[macro_use]
extern crate clap;
extern crate comment_strip;

mod config;

use std::io::{Read, Write, BufReader, BufWriter};
use clap::App;
use comment_strip::*;
use config::Config;

fn doit(config: Config) -> Result<(), AppError> {
    let mut data = String::new();
    let mut br = BufReader::new(config.input);
    br.read_to_string(&mut data)?;
    let stripped = strip_comments(data, config.style, config.remove_blanks)?;
    let mut bw = BufWriter::new(config.output);
    bw.write_all(stripped.as_bytes())?;
    Ok(())
}

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let config = Config::from_matches(&matches).unwrap();
    doit(config).unwrap();
}
