use crate::models::LogEntry;

pub enum Command {
    Add {
        message: Vec<String>,
        tags: Vec<String>,
    },
    List {
        // tag: Option<String>, simple implementation
        tags: Vec<String>,
    },
    Clear,
    Help, // Unknown(String),

          // None,
}

pub fn handle_add(logs: &mut Vec<LogEntry>, message: Vec<String>, tags: Vec<String>) {
    let id = logs.len() as u32 + 1;

    //catches where length >= 3 but no message provided 
    //from cli.rs
    if message.is_empty(){
        eprintln!("No arguments provided for 'add'"); 
        return
    }
    let message = message.join(" ");
    println!("Added log: {}", &message);
    logs.push(LogEntry { id, message, tags });
}

pub fn handle_list(logs: &[LogEntry], tags: Vec<String>) {
    if logs.is_empty() {
        println!("No logs yet!");
        return
    }

    if tags.is_empty() {
        logs.iter()
            .for_each(|log| println!("{}. {}", log.id, log.message));
    } else {
        logs.iter()
            // .filter(|&log| log.tags.iter().any(|tag| tags.contains(tag)))
            .filter(|&log| tags.iter().all(|tag| log.tags.iter().any(|t| t == tag)))
            .for_each(|log| println!("{}. {}", log.id, log.message));
    }
}

pub fn handle_help() {
    println!("devlog - simple developer logging CLI\n");
    println!("USAGE:");
    println!("  devlog add <message> [--tag <tag>...]");
    println!("  devlog list [--tag <tag>...]");
    println!("  devlog help\n");

    println!("EXAMPLES:");
    println!("  devlog add \"learned ownership\" --tag rust");
    println!("  devlog list --tag rust");
}
