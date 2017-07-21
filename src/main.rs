#[macro_use]
extern crate clap;

mod c;
mod shell;
mod xml;
mod blanklines;

use std::fs;
use std::io::{self, Read, Write, BufReader, BufWriter};
use std::path::Path;
use clap::App;

#[derive(Debug)]
enum Input {
    Standard(io::Stdin),
    File(fs::File)
}

impl Input {
    fn stdin() -> Input {
        Input::Standard(io::stdin())
    }
    fn file<P: AsRef<Path>>(path: P) -> io::Result<Input> {
        Ok(Input::File(try!(fs::File::open(path))))
    }
    fn from_arg<P: AsRef<Path>>(arg: Option<P>) -> io::Result<Input> {
        Ok(match arg {
            None       => Input::stdin(),
            Some(path) => try!(Input::file(path))
        })
    }
}

impl io::Read for Input {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match *self {
            Input::Standard(ref mut s) => s.read(buf),
            Input::File(ref mut f)     => f.read(buf),
        }
    }
}

#[derive(Debug)]
enum Output {
    Standard(io::Stdout),
    File(fs::File)
}

impl Output {
    fn stdout() -> Output {
        Output::Standard(io::stdout())
    }
    fn file<P: AsRef<Path>>(path: P) -> io::Result<Output> {
        Ok(Output::File(try!(fs::File::create(path))))
    }
    fn from_arg<P: AsRef<Path>>(arg: Option<P>) -> io::Result<Output> {
        Ok(match arg {
            None       => Output::stdout(),
            Some(path) => try!(Output::file(path))
        })
    }
}

impl io::Write for Output {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match *self {
            Output::Standard(ref mut s) => s.write(buf),
            Output::File(ref mut f)     => f.write(buf),
        }
    }
    fn flush(&mut self) -> io::Result<()> {
        match *self {
            Output::Standard(ref mut s) => s.flush(),
            Output::File(ref mut f)     => f.flush(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommentStyle {
    C,
    XML,
    Shell
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CommentMatch {
    pub from: usize,
    pub to: usize
}

fn find_comments(input: &str, style: &CommentStyle) -> Result<Vec<CommentMatch>, &'static str> {
    match style {
        &CommentStyle::C => c::find_comments(input),
        &CommentStyle::Shell => shell::find_comments(input),
        &CommentStyle::XML => xml::find_comments(input)
    }
}

fn strip_comments(input: String, matches: Vec<CommentMatch>) -> Result<String, &'static str> {
    let mut input = input;
    let mut matches = matches;
    matches.sort_by_key(|m| m.from);
    /* must come before reversing */
    let _ = try!(check_sorted_matches(input.as_str(), &matches));
    matches.reverse();
    for m in matches {
        input.drain((m.from)..(m.to));
    }
    Ok(input.to_owned())
}

fn check_sorted_matches(input: &str, matches: &Vec<CommentMatch>) -> Result<(), &'static str> {
    if matches.iter().any(|m| m.from >= input.len() || m.to > input.len()) {
        return Err("match out of range");
    }
    if matches.iter().zip(matches.iter().skip(1)).any(|(m, n)| m.to > n.from) {
        return Err("matches overlapping");
    }
    Ok(())
}

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let input = Input::from_arg(matches.value_of("INPUT"))
        .expect("Could not open INPUT file");
    let output = Output::from_arg(matches.value_of("output"))
        .expect("Could not open or create output file");
    let style_arg = (matches.is_present("c-style"),
                    matches.is_present("xml-style"),
                    matches.is_present("shell-style"));
    let comment_style = match style_arg {
        (true, _, _) => CommentStyle::C,
        (_, true, _) => CommentStyle::XML,
        (_, _, true) => CommentStyle::Shell,
        _ => CommentStyle::Shell
    };
    let remove_blanks = !matches.is_present("no-remove-blank-lines");
    //println!("Using input {:?}", &input);
    //println!("Using output {:?}", &output);
    //println!("Stripping {:?} style", &comment_style);
    let mut data = String::new();
    let mut br = BufReader::new(input);
    br.read_to_string(&mut data)
        .expect("Could not read data");
    let comment_matches = find_comments(data.as_str(), &comment_style)
        .expect("Could not parse comments");
    let mut stripped = strip_comments(data, comment_matches)
        .expect("Could not strip out comments");
    if remove_blanks {
        let blank_matches = blanklines::find_blanklines(stripped.as_str())
            .expect("Could not parse blank lines");
        stripped = strip_comments(stripped, blank_matches)
            .expect("Could not strip blank lines");
    }
    let mut bw = BufWriter::new(output);
    bw.write_all(stripped.as_bytes())
        .expect("Could not write data");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stripping_removes_correctly() {
        let s = "012345#789\n#abcd\nefghi#jkl\n".to_owned();
        let matches = vec![
            CommentMatch{from:6, to:10},
            CommentMatch{from:11, to:16},
            CommentMatch{from:22, to:26}];
        let stripped = strip_comments(s, matches);
        assert_eq!(Ok("012345\n\nefghi\n".to_owned()), stripped);
    }

    #[test]
    fn stripping_finds_overlapping() {
        let s = "1234567890".to_owned();
        let matches = vec![
            CommentMatch{from:0, to:5},
            CommentMatch{from:3, to:7}];
        let checked = check_sorted_matches(s.as_str(), &matches);
        assert!(checked.is_err());
        let stripped = strip_comments(s, matches);
        assert!(stripped.is_err());
    }

    #[test]
    fn stripping_finds_out_of_range() {
        let s = "12345".to_owned();
        let matches = vec![
            CommentMatch{from:3, to:10},
            CommentMatch{from:11, to:16}];
        let checked = check_sorted_matches(s.as_str(), &matches);
        assert!(checked.is_err());
        let stripped = strip_comments(s, matches);
        assert!(stripped.is_err());
    }

}
