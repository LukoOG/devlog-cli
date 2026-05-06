use std::{env, process};

mod cli;
mod commands;
mod models;
mod storage;

use cli::{Command, parse_args};
use commands::{handle_add, handle_list};
use models::LogEntry;
use storage::{load_logs, save_logs};

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

        Command::List { tag } => handle_list(&logs, tag),

        // Command::Unknown(command) => eprintln!("Unknown command: {}", command),

        // Command::None => eprintln!("No command provided"),
    }
}
