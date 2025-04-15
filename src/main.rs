use std::process;

use clap::Parser;
use home::home_dir;
use log::error;
use ralias::Args;

const BASHRC_FILENAME: &str = ".bashrc";
const GITALIAS_FILENAME: &str = ".gitconfig";

fn main() {
    #[cfg(debug_assertions)]
    simple_logger::init_with_level(log::Level::Debug).unwrap();

    #[cfg(not(debug_assertions))]
    simple_logger::init_with_level(log::Level::Warn).unwrap();

    let args = Args::parse();

    let mut file_path = match home_dir() {
        Some(path) => path,
        _ => {
            error!("Unable to get your home dir!");
            return;
        }
    };

    let file_name = if args.git {
        GITALIAS_FILENAME
    } else {
        BASHRC_FILENAME
    };
    file_path.push(file_name);

    if let Err(e) = ralias::run(&file_path, args) {
        error!("Application error: {e}");
        process::exit(1);
    }
}
