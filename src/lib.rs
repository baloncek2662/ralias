use std::error::Error;
use std::io::prelude::*;

use std::{fs::OpenOptions, io::Write, path::PathBuf};

use clap::{Parser, Subcommand};
use log::{debug, warn};

const ALIAS_PREFIX: &str = "alias";

#[derive(Parser)]
pub struct Args {
    /// operation to do on the alias
    #[clap(subcommand)]
    pub operation: Operation,
}

#[derive(Subcommand)]
pub enum Operation {
    /// <name>            Show a specific alias or all aliases
    Show {
        /// The name of the alias to show (optional)
        name: Option<String>,
    },
    /// <name> <command>  Add a new alias
    Add {
        /// The alias name
        name: String,
        /// The command to be executed
        command: String,
    },
    /// <name>            Remove an alias
    Remove {
        /// The name of the alias to remove
        name: String,
    },
    /// <name> <command>  Edit an alias
    Edit {
        /// The name of the alias to edit
        name: String,
        /// The new command to be executed
        command: String,
    },
}

// Box<dyn Error> means the function will return a type that implements the Error trait,
// but we don’t have to specify what particular type the return value will be.
pub fn run(bashrc_path: &PathBuf, args: Args) -> Result<(), Box<dyn Error>> {
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

fn show_alias(path: &PathBuf, name: Option<String>) -> Result<(), Box<dyn Error>> {
    let contents = std::fs::read_to_string(path).unwrap();
    let aliases = search(ALIAS_PREFIX, &contents);
    match name {
        Some(name) => {
            debug!("Show alias: {name}");
            if let Some(alias) = aliases.iter().find(|a| a.contains(&name)) {
                println!("{}", alias);
            } else {
                println!("Alias not found");
                Err("Alias not found")?
            }
        }
        None => {
            debug!("Show all aliases");
            for alias in aliases {
                println!("{}", alias);
            }
        }
    }
    Ok(())
}

fn add_alias(path: &PathBuf, name: String, command: String) -> Result<(), Box<dyn Error>> {
    debug!("Add alias: {name}, command={command}");

    if command.contains('\'') {
        println!("Command cannot contain single quotes");
        Err("Command cannot contain single quotes")?;
    }

    let mut contents: String = String::new();
    let mut file = OpenOptions::new().read(true).write(true).open(path)?;
    file.read_to_string(&mut contents)?;
    let name_alias_str = format!("{ALIAS_PREFIX} {name}=");
    if search(&name_alias_str, &contents).len() > 0 {
        println!("Alias already exists");
        return Err("Alias already exists")?;
    }

    let new_alias = format!("alias {name}='{command}'");
    let mut file = OpenOptions::new().append(true).open(&path)?;
    writeln!(file, "{}", new_alias)?;
    println!("New alias added: {new_alias}");
    Ok(())
}

fn remove_alias(path: &PathBuf, name: String) -> Result<(), Box<dyn Error>> {
    debug!("Remove alias: {}", name);
    helper_modify_alias(path, name, None)?;
    Ok(())
}

fn edit_alias(path: &PathBuf, name: String, command: String) -> Result<(), Box<dyn Error>> {
    debug!("Edit alias: {} with new command: {}", name, command);
    helper_modify_alias(path, name, Some(command))?;
    Ok(())
}

// Removes alias if command is not provided and updates alias if command is provided
fn helper_modify_alias(path: &PathBuf, name: String, command: Option<String>) -> Result<(), Box<dyn Error>> {
    let mut file = OpenOptions::new().read(true).write(true).open(path)?;
    let mut contents: String = String::new();
    file.read_to_string(&mut contents)?;

    let mut new_contents = String::new();
    let mut found = 0;
    let name_alias_str = format!("{ALIAS_PREFIX} {name}=");
    for line in contents.lines() {
        if line.starts_with(&name_alias_str) {
            if let Some(command) = &command {
                new_contents.push_str(&format!("{}'{}'\n", name_alias_str, command));
                println!("Editing alias: {}", line);
            } else {
                println!("Removing alias: {}", line);
            }
            found += 1;
            continue;
        }
        new_contents.push_str(line);
        new_contents.push('\n');
    }
    file.set_len(0)?;
    file.write_all(new_contents.as_bytes())?;

    if found == 0 {
        println!("Alias '{}' not found", name);
        Err("Alias not found")?;
    } else if found > 1 {
        warn!("Found more than one alias with name '{}'", name);
    }

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
    use tempfile::NamedTempFile;

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

    #[test]
    fn test_add_alias() {
        let temp_file = NamedTempFile::new().unwrap();
        let tmp_path = temp_file.path().to_path_buf();
        let name = "g";
        let command = "git";
        let expected = "alias g='git'\n";

        add_alias(tmp_path.clone(), name.to_string(), command.to_string()).unwrap();

        let contents = std::fs::read_to_string(tmp_path).unwrap();
        assert_eq!(expected, contents);
    }

    fn helper_create_temp_bashrc() -> NamedTempFile {
        let temp_file = NamedTempFile::new().unwrap();
        let tmp_path = temp_file.path().to_path_buf();
        let contents = "alias g='git'\n";
        std::fs::write(tmp_path.clone(), contents).unwrap();
        temp_file
    }

    #[test]
    fn test_show_alias() {
        let temp_file = helper_create_temp_bashrc();
        assert!(show_alias(temp_file.path().to_path_buf(), None).is_ok());
        assert!(show_alias(temp_file.path().to_path_buf(), Some(String::from("g"))).is_ok());
    }
}
