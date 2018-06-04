extern crate rand;
use data::rand::Rng; // extension trait
use std;

/// `ParsingError` represents every possible thing that can go wrong during parsing
#[derive(Fail, Debug)]
pub enum ParsingError {
    /// `NoArguments` for when the user forgets to supply a file
    #[fail(display = "Please supply at least one argument in the form of a .json file.")]
    NoArguments,

    /// `PrefixEmpty` for when the user forgets to add any city prefixes
    #[fail(display = "The prefix array is empty, please supply some values.")]
    PrefixEmpty,

    /// `SuffixEmpty` for when the user forgets to add any city suffixes
    #[fail(display = "The suffix array is empty, please supply some values.")]
    SuffixEmpty,

    /// `IndexOutOfBound` for when the user wants more cities than are available
    #[fail(display = "Unable to return the requested amount.")]
    IndexOutOfBound,
}

/// Alias for the result type in Data
pub type Result<T> = std::result::Result<T, ParsingError>;

/// The struct-representation of the .json file
#[derive(Deserialize, Debug)]
pub struct Data {
    pub prefixes: Vec<String>,
    pub suffixes: Vec<String>,
}

use ParsingError::{PrefixEmpty, SuffixEmpty};

impl Data {
    /// Generates a single random city based on the list of available prefixes and suffixes.
    pub fn generate_random(&self) -> Result<String> {
        let mut random = rand::thread_rng();
        let prefix = random.choose(&self.prefixes).ok_or(PrefixEmpty)?;
        let suffix = random.choose(&self.suffixes).ok_or(SuffixEmpty)?;
        Ok(prefix.clone() + &suffix)
    }

    /// Generates all the possible cities sourced from the  available prefixes or suffixes
    /// by taking the cartesian product of the two vectors.
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

    /// Generates `count` number of cities sourced at random from the total list of cities.
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