use crate::models::LogEntry;

pub fn handle_add(logs: &mut Vec<LogEntry>, message: Vec<String>, tags: Vec<String>) {
    let id = logs.len() as u32 + 1;
    let message = message.join(" ");
    println!("Added log: {}", &message);
    logs.push(LogEntry { id, message, tags });
}

pub fn handle_list(logs: &[LogEntry], tag: Option<String>) {
    match tag {
        Some(t) => {
            logs.iter()
                .filter(|&log| log.tags.iter().any(|tag| tag == &t))
                .for_each(|log| println!("{}. {}", log.id, log.message));
        }
        None => {
            logs.iter()
                .for_each(|log| println!("{}. {}", log.id, log.message));
        }
    }
}
