use std::error::Error;
use std::{fs::OpenOptions, io::Write, path::PathBuf};

use clap::{Parser, Subcommand};
use log::debug;

#[derive(Parser)]
pub struct Args {
    /// operation to do on the alias `name`
    #[clap(subcommand)]
    pub operation: Operation,
}

#[derive(Subcommand)]
pub enum Operation {
    /// Show a specific alias or all aliases
    Show {
        /// The name of the alias to show (optional)
        name: Option<String>,
    },
    Add {
        /// The alias name
        name: String,
        /// The command to be executed
        command: String,
    },
    Remove {
        /// The name of the alias to remove
        name: String,
    },
    Edit {
        /// The name of the alias to edit
        name: String,
        /// The new command to be executed
        command: String,
    },
}

// Box<dyn Error> means the function will return a type that implements the Error trait,
// but we don’t have to specify what particular type the return value will be.
pub fn run(bashrc_path: PathBuf, args: Args) -> Result<(), Box<dyn Error>> {
    match args.operation {
        Operation::Show { name } => {
            show_alias(bashrc_path, name)?;
        }
        Operation::Add { name, command } => {
            add_alias(bashrc_path, name, command)?;
        }
        Operation::Remove { name } => {
            remove_alias(bashrc_path, name)?;
        }
        Operation::Edit { name, command } => {
            edit_alias(bashrc_path, name, command)?;
        }
    }
    Ok(())
}

fn show_alias(bashrc_path: PathBuf, name: Option<String>) -> Result<(), Box<dyn Error>> {
    match name {
        Some(name) => {
            debug!("Show alias: {}", name);
            // Implement logic to show a specific alias
        }
        None => {
            debug!("Show all aliases");
            // Implement logic to show all aliases
        }
    }
    Ok(())
}

fn add_alias(bashrc_path: PathBuf, name: String, command: String) -> Result<(), Box<dyn Error>> {
    let new_alias = format!("alias {}='{}'\n", name, command);

    let mut file = OpenOptions::new()
        .append(true)
        .open(&bashrc_path)?;

    writeln!(file, "{}", new_alias)?;

    debug!("New alias added: {}", new_alias);

    Ok(())
}

fn remove_alias(bashrc_path: PathBuf, name: String) -> Result<(), Box<dyn Error>> {
    // Implement logic to remove an alias
    debug!("Remove alias: {}", name);
    Ok(())
}

fn edit_alias(bashrc_path: PathBuf, name: String, command: String) -> Result<(), Box<dyn Error>> {
    // Implement logic to edit an alias
    debug!("Edit alias: {} with new command: {}", name, command);
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

        assert_eq!(
            vec!["safe, fast, productive.", "fast, efficient, powerful."],
            search(query, contents)
        );
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
}
