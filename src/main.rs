use serde::{self, Deserialize, Serialize};
use std::{env, error::Error, fs, process};

#[derive(Debug, Serialize, Deserialize)]
struct LogEntry {
    id: u32,
    message: String,
    tags: Vec<String>,
}

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
            let mut tag = String::new();
            let mut index = 0;

            while index < args.len() {
                if args[index] == "--tag" {
                    if let Some(t) = args.get(index + 1) {
                        tag = t.to_string();
                    }
                }
                index += 1
            }
            handle_list(&logs, &tag)
        }

        _ => eprintln!("Unknown command: {}", command),
    }
}

fn handle_add(logs: &mut Vec<LogEntry>, args: &[String]) {
    let id = logs.len() as u32 + 1;
    let mut message_parts: Vec<&str> = Vec::new();
    let mut tags: Vec<String> = Vec::new();
    let mut args = args.iter();

    while let Some(c) = args.next() {
        if c == "--tag" {
            if let Some(next) = args.next() {
                tags.push(next.clone());
            };
        } else {
            message_parts.push(c);
        }
    }

    let message = message_parts.join(" ");
    logs.push(LogEntry { id, message, tags });
}

fn handle_list(logs: &[LogEntry], tag: &str) {
    if !tag.is_empty() {
        logs.iter()
            .for_each(|log| println!("{}. {}", log.id, log.message));
    } else {
        logs.iter()
            .filter(|&x| x.tags.contains(&tag.to_string()))
            .for_each(|log| println!("{}. {}", log.id, log.message));
    }
}

fn load_logs() -> Vec<LogEntry> {
    let contents = fs::read_to_string("devlog.json");

    let parsed = match contents {
        Ok(val) => match serde_json::from_str(val.as_str()) {
            Ok(logs) => logs,
            Err(e) => {
                eprintln!("Failed to parse logs: {}", e);
                Vec::new()
            }
        },

        Err(_) => return Vec::new(),
    };

    parsed
}

fn save_logs(logs: &[LogEntry]) -> Result<(), Box<dyn Error>> {
    let json = serde_json::to_string_pretty(logs)?;
    fs::write("devlog.json", json)?;
    Ok(())
}
