use std::process;

fn main() {
    if let Err(e) = grusterylist::run() {
        eprintln!("Problem running application:\n{}", e);
	drop(e); // From the [docs](https://doc.rust-lang.org/std/process/fn.exit.html)\
	         // If a clean shutdown is needed it is recommended to only call this function at a known point
	         // where there are no more destructors left to run.
	process::exit(1);
    }
}
