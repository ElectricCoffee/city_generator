mod data;
use data::*;
// SerDe
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
// Failure
extern crate failure;
#[macro_use]
extern crate failure_derive;
use failure::Error;
// Std
use std::env;
use std::fs::File;
use std::io::prelude::*;

/// Reads a file as given by the input path.
///
/// Returns its contents as a `String`.
fn read_file(path: &str) -> std::io::Result<String> {
    let mut file = File::open(path)?;
    let mut result = String::new();
    file.read_to_string(&mut result)?;
    Ok(result)
}

/// Runs the whole thing and returns the result.
///
/// Returns a `String` of generated names if it succeeds,
/// or a `failure::Error` if it fails.
fn run() -> std::result::Result<String, Error> {
    let args: Vec<String> = env::args().collect();
    let path = args.get(1).ok_or(ParsingError::NoArguments)?;
    let file = read_file(path)?;
    let data = serde_json::from_str::<Data>(&file)?;
    let result = data.generate_amount(20usize)?;

    let mut string = String::new();
    for s in result.iter() {
        string += &format!("{}\n", s);
    }
    Ok(string)
}

fn main() {
    match run() {
        Ok(result) => println!("{}", result),
        Err(error) => eprintln!("{}", error),
    }
}
