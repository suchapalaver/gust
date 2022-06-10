use crate::ReadError;

use clap::{App, Arg};

// Run application with one of three subcommands:
// cargo run -- l
//   "    "  -- g
//   "    "  -- r
pub fn run() -> Result<(), ReadError> {
    let args = App::new("grusterylist")
        .override_help(
            "\n\
	     grusterylist 0.1.0\n\
	     Makes grocery lists in Rust\n\
	     (C) https://github.com/suchapalaver/\n\n\
	     Usage: cargo run -- <opts>\n\n\
	     OPTIONS:\n    \
	     -h, --help       Print help information\n    \
	     -V, --version    Print version information\n    \
	     \n\
	     SUBCOMMANDS:\n    \
	     g     Add groceries to library\n    \
	     r     Add recipes to library\n    \
	     l     Make a shopping list\n\
	     \n\
	     EXAMPLE:\n    \
	     cargo run -- l",
        )
        .arg(Arg::new("subcommands").required(true).max_values(1))
        .get_matches();

    let subcommand = args.value_of("subcommands").unwrap_or("-");

    match subcommand {
        "l" => Ok(crate::run_shopping_list::run()?),
        "g" => Ok(crate::run_groceries::run()?),
        "r" => Ok(crate::run_recipes::run()?),
        &_ => Err(ReadError::ParseInputError),
    }
}
