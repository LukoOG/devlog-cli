use std::process;
use crate::commands::Command;

pub fn parse_args(args: &[String]) -> Command {
    let length = args.len();
    if length < 2 {
        eprintln!("No arguments provided");
        process::exit(1);
    }

    let command = args[1].as_str();

    match command {
        "add" => {
            if length < 3 {
                eprintln!("No arguments provided for 'add'");
                process::exit(1);
            } else {
                let mut message_parts: Vec<String> = Vec::new();
                let mut tags: Vec<String> = Vec::new();
                let mut iter = args[2..].iter();

                while let Some(c) = iter.next() {
                    if c == "--tag" {
                        if let Some(next) = iter.next() {
                            tags.push(next.clone());
                        };
                    } else {
                        message_parts.push(c.clone());
                    }
                }

                Command::Add {
                    message: message_parts,
                    tags,
                }
            }
        }

        "list" => {
            let mut tags = vec![];
            let mut iter = args[2..].iter();

            while let Some(arg) = iter.next() {
                if arg == "--tag" {
                    if let Some(next) = iter.next() {
                        tags.push(next.clone());
                    }
                }
            }

            Command::List { tags }
        },

        "clear" => {
            Command::Clear
        },

        "help" => {
            Command::Help
        },

        _ => {
            eprintln!("Unknown command {}", command);
            process::exit(1)
        }
    }
}
