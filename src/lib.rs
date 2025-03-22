use std::error::Error;
use std::io::prelude::*;

use std::{fs::OpenOptions, io::Write, path::PathBuf};
use std::collections::HashMap;

use clap::{Parser, Subcommand};
use log::{debug, warn};
use regex::Regex;
use colored::Colorize;

const ALIAS_PREFIX: &str = "alias";

#[derive(Parser)]
pub struct Args {
    /// operation to do on the alias
    #[clap(subcommand)]
    pub operation: Operation,
}

#[derive(Subcommand)]
pub enum Operation {
    /// <name>            Show all aliases or all aliases which contain <name>
    Show {
        /// The name of the alias to show (optional).
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
            let _ = show_aliases(bashrc_path, &name);
            let _ = show_funcs(bashrc_path, &name);
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

// Highlights in red only the part before the separator that matches the search pattern
fn highlight_search_pattern(line: &str, search_pattern: &str, separator: char) -> String {
    let parts: Vec<&str> = line.splitn(2, separator).collect();
    if parts.len() == 2 {
        let colored = parts[0].replace(search_pattern, &search_pattern.red().bold().to_string());
        let rest = parts[1];
        format!("{}{}{}", colored, separator, rest)
    } else {
        line.to_string()
    }
}

fn show_aliases(path: &PathBuf, name: &Option<String>) -> Result<(), Box<dyn Error>> {
    let contents = std::fs::read_to_string(path).unwrap();
    // Regex to find all lines starting with 'alias', followed by any characters and an equal sign
    let aliases = search(&format!(r"^\s*{ALIAS_PREFIX}.*=.*\s*"), &contents);
    match name {
        Some(name) => {
            debug!("Show alias containing: {name}");
            let mut found = false;
            for alias in aliases {
                // only match left-hand side of the alias - which is the alias name
                let alias_name = alias.split('=').collect::<Vec<&str>>()[0];

                if alias_name.contains(name) {
                    let alias = highlight_search_pattern(alias, name, '=');
                    println!("{}", alias);
                    found = true;
                }
            }
            if !found {
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

fn search_bash_functions(contents: &str) -> HashMap<String, String> {
    let mut result = HashMap::new();
    let mut curr_func_name = String::new();
    let mut curr_func: Vec<String> = Vec::new();
    let mut in_func = false;
    let query = Regex::new(r"^\s*[a-zA-Z_][a-zA-Z0-9_]*\s*\(\s*\)\s*\{").unwrap();
    for line in contents.lines() {
        if query.is_match(line) {
            // Found a function, what follows is the function body until the closing brace
            in_func = true;
            // Get just the function name which will be used as the key in the result hashmap
            curr_func_name = line.split('(').collect::<Vec<&str>>()[0].trim().to_string();
            curr_func.push(line.to_string());
        } else if in_func {
            curr_func.push(line.to_string());
            if line.contains("}") {
                in_func = false;
                result.insert(curr_func_name.clone(), curr_func.join("\n"));
                curr_func.clear();
            }
        }
    }
    result
}

fn show_funcs(path: &PathBuf, name: &Option<String>) -> Result<(), Box<dyn Error>> {
    let contents = std::fs::read_to_string(path).unwrap();
    let funcs = search_bash_functions(contents.as_str());
    match name {
        Some(name) => {
            debug!("Show function containing: {name}");
            for (func_name, func) in funcs {
                if func_name.contains(name) {
                    let func = highlight_search_pattern(&func, name, '{');
                    println!("{}", func);
                }
            }
        }
        None => {
            debug!("Show all functions");
            for (_, func_body) in funcs {
                println!("{}", func_body);
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
    for line in contents.lines() {
        // Regex to find if the line starting with 'alias', followed by whitespace and the name of the alias exists
        let line_match = search(&format!(r"^\s*{ALIAS_PREFIX}\s*{name}=.*\s*"), line);
        if line_match.len() > 0 {
            if let Some(command) = &command {
                // Format the command properly
                let name_alias_str = format!("{ALIAS_PREFIX} {name}=");
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
    println!("{}", new_contents);
    let mut file = OpenOptions::new().write(true).truncate(true).open(path)?;

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

    let query = Regex::new(query).unwrap();
    for line in contents.lines() {
        if query.is_match(line) {
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

        add_alias(&tmp_path.clone(), name.to_string(), command.to_string()).unwrap();

        let contents = std::fs::read_to_string(tmp_path).unwrap();
        assert_eq!(expected, contents);
    }

    fn helper_create_temp_bashrc() -> NamedTempFile {
        let temp_file = NamedTempFile::new().unwrap();
        let tmp_path = temp_file.path().to_path_buf();
        let contents = "alias g='git'\n";
        std::fs::write(&tmp_path, contents).unwrap();
        temp_file
    }

    #[test]
    fn test_show_alias() {
        let temp_file = helper_create_temp_bashrc();
        assert!(show_aliases(&temp_file.path().to_path_buf(), &None).is_ok());
        assert!(show_aliases(&temp_file.path().to_path_buf(), &Some(String::from("g"))).is_ok());
        assert!(show_aliases(&temp_file.path().to_path_buf(), &Some(String::from("non_existent"))).is_err());
    }

    #[test]
    fn test_remove_alias() {
        let temp_file = helper_create_temp_bashrc();
        assert!(show_aliases(&temp_file.path().to_path_buf(), &Some(String::from("g"))).is_ok());
        assert!(remove_alias(&temp_file.path().to_path_buf(), String::from("g")).is_ok());
        assert!(show_aliases(&temp_file.path().to_path_buf(), &Some(String::from("g"))).is_err());
    }
}
