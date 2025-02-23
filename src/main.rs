use clap::Parser;
use home::home_dir;

const BASHRC_FILENAME: &str = ".bashrc";

#[derive(Parser)]
struct Args {
    // name of alias
    name: String,
    // system command to be executed by alias
    command: String,
}

fn main() {
    let args = Args::parse();
    println!("name={}, pattern={}", args.name, args.command);

    let mut bashrc_path = match home_dir() {
        Some(path) => path,
        _ => {
            println!("Unable to get your home dir!");
            return;
        },
    };
    bashrc_path.push(BASHRC_FILENAME);
    println!("{:?}", bashrc_path);
    let content = std::fs::read_to_string(bashrc_path).expect("Could not read bashrc file");
    println!("{content}");
}
