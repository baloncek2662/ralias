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
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_search() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn test_search_case_sensitive() {
        let query = "Rust";
        let contents = "\
Rust:
safe, fast, productive.
Trust me.";

        assert_eq!(vec!["Rust:"], search(query, contents));
    }

    #[test]
    fn test_search_case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Trust me.";

        assert_eq!(Vec::<&str>::new(), search(query, contents));
    }

    #[test]
    fn test_search_multiple_results() {
        let query = "fast";
        let contents = "\
Rust:
safe, fast, productive.
C++:
fast, efficient, powerful.";

        assert_eq!(vec!["safe, fast, productive.", "fast, efficient, powerful."], search(query, contents));
    }

    #[test]
    fn test_search_no_results() {
        let query = "slow";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(Vec::<&str>::new(), search(query, contents));
    }

    #[test]
    fn test_run() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join(".bashrc");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "alias ll='ls -la'").unwrap();

        let args = Args {
            name: "ll".to_string(),
            command: "ls -la".to_string(),
        };

        let result = run(file_path, args);
        assert!(result.is_ok());
    }

    #[test]
    fn test_run_file_not_found() {
        let file_path = PathBuf::from("non_existent_file");
        let args = Args {
            name: "ll".to_string(),
            command: "ls -la".to_string(),
        };

        let result = run(file_path, args);
        assert!(result.is_err());
    }
}
