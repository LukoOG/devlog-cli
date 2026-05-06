use std::env;
use std::process;

#[derive(Debug)]
struct LogEntry {
    id: u32,
    message: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    //  let args = dbg!(args);
    let mut logs: Vec<LogEntry> = Vec::new();

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
            } else {
                eprintln!("Missing message for 'add' command");
                process::exit(1);
            }
        }

        "list" => handle_list(&logs),

        _ => eprintln!("Unknown command: {}", command),
    }

    println!("{:?}", logs)
}

fn handle_add(logs: &mut Vec<LogEntry>, args: &[String]) {
    let id = logs.len() as u32 + 1;
    let message = args.join(" ");
    println!("Added log: {}", &message);
    logs.push(LogEntry { id, message });
}

fn handle_list(logs: &[LogEntry]) {
   for log in logs{
      println!("{}. {}", log.id, log.message)
   }
}
