use std::process;

fn main() {
    let greeting = "How are you?!\nLet's get the shopping done!";
    eprintln!("{}", greeting);

    if let Err(e) = groceries::run() {
        eprintln!("Problem running application: {}", e);
        process::exit(1);
    }
}
