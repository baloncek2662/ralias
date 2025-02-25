use std::error::Error;
use std::{fs, path::PathBuf};

use clap::Parser;
use log::info;

#[derive(Parser)]
pub struct Args {
    /// name of alias
    pub name: String,
    /// system command to be executed by alias
    pub command: String,
}

// Box<dyn Error> means the function will return a type that implements the Error trait,
// but we don’t have to specify what particular type the return value will be.
pub fn run(bashrc_path: PathBuf, _args: Args) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(bashrc_path)?;

    info!("Content:\n{content}");

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }

    result
}

// This attribute ensures that the tests module is only compiled and included when running tests (cargo test).
#[cfg(test)]
// This defines a test module named tests.
mod tests {
    // This imports everything (*) from the parent module
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
