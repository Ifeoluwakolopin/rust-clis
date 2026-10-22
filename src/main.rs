use clap::{builder::OsStringValueParser, Arg, ArgAction, Command};
use std::ffi::OsString;

fn main() {
    let matches = Command::new("echor")
        .version("0.1.0")
        .author("Ifeoluwakolopin Are <areifeoluwa10@gmail.com>")
        .about("Echor is a simple echo app built with Rust")
        .arg(
            Arg::new("text")
                .value_name("TEXT")
                .help("The text to echo")
                .required(true)
                .num_args(1..)
                .value_parser(OsStringValueParser::new()),
        )
        .arg(
            Arg::new("omit_newline")
                .short('n')
                .help("Do not print newline")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    let text = matches
        .get_many::<OsString>("text")
        .expect("text is required by the argument parser")
        .map(|value| value.to_string_lossy())
        .collect::<Vec<_>>();
    let omit_newline = matches.get_flag("omit_newline");

    print!("{}{}", text.join(" "), if omit_newline { "" } else { "\n" });
}
