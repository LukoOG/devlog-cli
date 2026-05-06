use crate::models::LogEntry;

pub fn handle_add(logs: &mut Vec<LogEntry>, args: &[String]) {
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
    println!("Added log: {}", &message);
    logs.push(LogEntry { id, message, tags });
}

pub fn handle_list(logs: &[LogEntry], tag: Option<&str>) {
    match tag {
        Some(t) => {
            logs.iter()
                .filter(|&log| log.tags.iter().any(|tag| tag == t))
                .for_each(|log| println!("{}. {}", log.id, log.message));
        }
        None => {
            logs.iter()
                .for_each(|log| println!("{}. {}", log.id, log.message));
        }
    }
}