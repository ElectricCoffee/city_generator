#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate rand;
use rand::Rng; // extension trait
use std::env;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
enum ParsingError {
    PrefixEmptyError,
    SuffixEmptyError,
}

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    prefixes: Vec<String>,
    suffixes: Vec<String>,
}

impl Data {
    fn generate_random(&self) -> Result<String, ParsingError> {
        use ParsingError::*;
        let mut random = rand::thread_rng();
        let prefix = random.choose(&self.prefixes).ok_or(PrefixEmptyError)?;
        let suffix = random.choose(&self.suffixes).ok_or(SuffixEmptyError)?;
        Ok(prefix.clone() + &suffix)
    }

    fn generate_all(&self) -> Vec<String> {
        let mut results = Vec::new();
        for prefix in self.prefixes.iter() {
            for suffix in self.suffixes.iter() {
                results.push(prefix.clone() + &suffix);
            }
        }
        results
    }
}

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
