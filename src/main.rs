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

use std::env;
use std::fs::File;
use std::io::prelude::*;

fn read_file(path: &str) -> std::io::Result<String> {
    let mut file = File::open(path)?;
    let mut result = String::new();
    file.read_to_string(&mut result)?;
    Ok(result)
}

macro_rules! try_print {
    ($e:expr) => ({
        match $e {
            Ok(k) => k,
            Err(err) => {
                eprintln!("{}", err);
                return;
            }
        }
    })
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if let Some(path) = args.get(1) {
        let result = try_print!{read_file(path)};
        let data: Data = try_print!{serde_json::from_str(&result)};
        let town = data.generate_random().unwrap();
        println!("{}", town);
    } else {
        println!("Error: Please supply a file path to a .json file");
    }
}
