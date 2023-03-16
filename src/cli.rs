use clap::{Arg, Command};
use crossterm::{
    event::{read, Event, KeyCode},
};

/// Builds the CLI configuration using clap
///
/// # Returns
///
/// * A Command instance with the defined CLI arguments.
pub fn create_cli() -> clap::Command {
    Command::new("Light Evm Interpreter")
        .version("0.0.1")
        .author("Julio4")
        .about("A minimal EVM interpreter in Rust (for educational purpose)")
        .arg(
            Arg::new("bytecode")
                .short('b')
                .long("bytecode")
                .value_name("BYTECODE")
                .help("The bytecode to be executed")
                .required(true),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .default_value("true")
                .num_args(0)
                .help("Enable logging of EVM operations"),
        )
        .arg(
            Arg::new("step")
                .short('s')
                .long("step")
                .default_value("true")
                .num_args(0)
                .help("Enable step by step execution (useful with -v)"),
        )
}

pub fn wait_for_press(key: &char) {
    loop {
        if let Ok(event) = read() {
            match event {
                Event::Key(event) if event.code == KeyCode::Char(*key) => return,
                _ => (),
            }
        }
    }
}
