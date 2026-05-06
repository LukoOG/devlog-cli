use std::{env, process};

mod cli;
mod commands;
mod models;
mod storage;

use cli::{parse_args};
use commands::{Command, handle_add, handle_list};
use storage::{load_logs, save_logs};

use crate::commands::handle_help;

fn main() {
    let args: Vec<String> = env::args().collect();

    let command = parse_args(&args);
    let mut logs= load_logs();

    match command {
        Command::Add { message, tags } => {
            handle_add(&mut logs, message, tags);
            if let Err(e) = save_logs(&logs) {
                eprintln!("Failed to save logs: {}", e);
                process::exit(1)
            };
        }

        Command::List { tags } => handle_list(&logs, tags),

        Command::Help => handle_help(),

        // Command::Unknown(command) => eprintln!("Unknown command: {}", command),

        // Command::None => eprintln!("No command provided"),
    }
}
