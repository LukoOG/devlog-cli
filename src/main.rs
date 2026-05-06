use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let args = dbg!(args);

    let command = &args[1][..];
    match command {
        "add" => {
            if !args.get(2).is_none() {
                println!("added {}", args[2]);
            } else {
               eprintln!("no arguemnts provided for add")
            }
        }

        "list" => println!("list all"),

        _ => eprintln!("Unidentified command")
    }
}
