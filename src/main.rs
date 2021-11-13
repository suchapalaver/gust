use std::process;

fn main() {
    if let Err(e) = grusterylist::run() {
        eprintln!("Problem running application:\n{}", e);
        process::exit(1);
    }
}
