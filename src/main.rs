use std::env;

enum Command {
   Add { message: String },
   List
}
fn main() {
   let args: Vec<String> = env::args().collect();
   let _args = dbg!(args);
}
