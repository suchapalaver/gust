use std::process;

fn main() {
    if let Err(e) = grusterylist::run() {
        eprintln!("Problem running application: {}", e);
        process::exit(1);
    }
}
