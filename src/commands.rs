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
    Help, // Unknown(String),

          // None,
}

pub fn handle_add(logs: &mut Vec<LogEntry>, message: Vec<String>, tags: Vec<String>) {
    let id = logs.len() as u32 + 1;
    let message = message.join(" ");
    println!("Added log: {}", &message);
    logs.push(LogEntry { id, message, tags });
}

pub fn handle_list(logs: &[LogEntry], tags: Vec<String>) {
    if logs.is_empty() {
        println!("No logs yet!")
    }
    // match tag {
    //     Some(t) => {
    //         logs.iter()
    //             .filter(|&log| log.tags.iter().any(|tag| tag == &t))
    //             .for_each(|log| println!("{}. {}", log.id, log.message));
    //     }
    //     None => {
    //         logs.iter()
    //             .for_each(|log| println!("{}. {}", log.id, log.message));
    //     }
    // }

    if tags.is_empty() {
        logs.iter()
            .for_each(|log| println!("{}. {}", log.id, log.message));
    } else {
        logs.iter()
            .filter(|&log| log.tags.iter().any(|tag| tags.contains(tag)))
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
