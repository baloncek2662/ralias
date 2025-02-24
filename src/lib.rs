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
