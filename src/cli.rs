// pub fn build_cli() -> Command {
//     Command::new("compl")
//         .about("Tests completions")
//         .arg(Arg::new("file")
//             .help("some input file"))
//         .subcommand(Command::new("test")
//             .about("tests things")
//             .arg(Arg::new("case")
//                 .long("case")
//                 .action(ArgAction::Set)
//                 .help("the case to test")))
// }
use clap::{Command, Arg, ValueHint, value_parser, ArgAction};
use clap_complete::aot::{generate, Generator, Shell};
use std::io;

// src/cli.rs
pub fn build_cli() -> Command {
    Command::new("compl")
        .about("Tests completions")
        .arg(Arg::new("file")
            .help("some input file"))
        .subcommand(Command::new("test")
            .about("tests things")
            .arg(Arg::new("case")
                .long("case")
                .action(ArgAction::Set)
                .help("the case to test")))
}

pub fn print_completions<G: Generator>(generator: G, cmd: &mut Command) {
    generate(generator, cmd, cmd.get_name().to_string(), &mut io::stdout());
}
