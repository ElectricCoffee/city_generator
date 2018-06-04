extern crate rand;
use data::rand::Rng; // extension trait
use std;

#[derive(Fail, Debug)]
pub enum ParsingError {
    #[fail(display = "Please supply at least one argument in the form of a .json file.")]
    NoArguments,

    #[fail(display = "The prefix array is empty, please supply some values.")]
    PrefixEmpty,

    #[fail(display = "The suffix array is empty, please supply some values.")]
    SuffixEmpty,

    #[fail(display = "Unable to return the requested amount.")]
    IndexOutOfBound,
}

pub type Result<T> = std::result::Result<T, ParsingError>;

#[derive(Deserialize, Debug)]
pub struct Data {
    pub prefixes: Vec<String>,
    pub suffixes: Vec<String>,
}

use ParsingError::{PrefixEmpty, SuffixEmpty};

impl Data {
    pub fn generate_random(&self) -> Result<String> {
        let mut random = rand::thread_rng();
        let prefix = random.choose(&self.prefixes).ok_or(PrefixEmpty)?;
        let suffix = random.choose(&self.suffixes).ok_or(SuffixEmpty)?;
        Ok(prefix.clone() + &suffix)
    }

    pub fn generate_all(&self) -> Result<Vec<String>> {
        let mut results = Vec::new();

        if self.prefixes.is_empty() {
            return Err(PrefixEmpty)
        } else if self.suffixes.is_empty() {
            return Err(SuffixEmpty)
        }

        for prefix in self.prefixes.iter() {
            for suffix in self.suffixes.iter() {
                results.push(prefix.clone() + &suffix);
            }
        }

        Ok(results)
    }

    pub fn generate_amount(&self, count: usize) -> Result<Vec<String>> {
        let mut random = rand::thread_rng();
        let mut towns = self.generate_all()?;

        if towns.len() < count {
            return Err(ParsingError::IndexOutOfBound);
        }

        random.shuffle(&mut towns);
        
        let mut result = Vec::new();
        for e in towns[.. count].iter() {
            result.push(e.clone());
        }

        Ok(result)
    }
}