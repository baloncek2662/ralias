use std::process;

use clap::Parser;
use home::home_dir;
use log::{debug, error};
use rashrc::Args;

const BASHRC_FILENAME: &str = ".bashrc";

fn main() {
    simple_logger::init_with_level(log::Level::Debug).unwrap();

    let args = Args::parse();
    debug!("Arguments: name={}, command={}", args.name, args.command);

    let mut bashrc_path = match home_dir() {
        Some(path) => path,
        _ => {
            error!("Unable to get your home dir!");
            return;
        }
    };
    bashrc_path.push(BASHRC_FILENAME);

    if let Err(e) = rashrc::run(bashrc_path, args) {
        error!("Application error: {e}");
        process::exit(1);
    }
}
