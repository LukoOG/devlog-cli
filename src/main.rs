use std::{env, process};

mod commands;
mod models;
mod storage;

use commands::*;
use models::LogEntry;
use storage::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    //  let args = dbg!(args);
    let mut logs: Vec<LogEntry> = load_logs();

    let command = match args.get(1) {
        Some(val) => val.as_str(),
        None => {
            eprintln!("No command given");
            process::exit(1)
        }
    };

    match command {
        "add" => {
            let length = args.len();
            // print!("{}",length);
            if length >= 3 {
                let message_parts = &args[2..];
                handle_add(&mut logs, message_parts);

                if let Err(e) = save_logs(&logs) {
                    eprint!("Failed to save logs: {}", e);
                    process::exit(1);
                };
            } else {
                eprintln!("Missing message for 'add' command");
                process::exit(1);
            }
        }

        "list" => {
            if logs.is_empty() {
                println!("no logs yet!");
                return;
            }
            let args = &args[2..];
            let mut tag = None;
            let mut index = 0;

            while index < args.len() {
                if args[index] == "--tag" {
                    if let Some(t) = args.get(index + 1) {
                        tag = Some(t.as_str());
                        break;
                    }
                }
                index += 1
            }
            handle_list(&logs, tag)
        }

        _ => eprintln!("Unknown command: {}", command),
    }
}