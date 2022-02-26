// Try this:
// Note: Loads the contents of the module from another file
//       with the same name as the module. Read more at
//       https://doc.rust-lang.org/book/ch07-05-separating-modules-into-different-files.html
mod errors;
mod groceries;
mod helpers;
mod shoppinglist;

use std::error::Error;

// Note: Re-exports the content of the square_content module to keep paths short.
//       Read more at https://doc.rust-lang.org/reference/items/use-declarations.html#use-visibility
pub use crate::errors::*;
pub use crate::groceries::*;
pub use crate::helpers::*;
pub use crate::shoppinglist::*;

use clap::{App, Arg};

// Using `clap` to parse command line arguments
// Run application with one of three subcommands:
// cargo run -- l
//   "    "  -- g
//   "    "  -- r
pub fn run() -> Result<(), Box<dyn Error>> {
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
	     g     Add groceries to groceries library\n    \
	     r     Add recipes to recipes library\n    \
	     l     Make a shopping list\n\
	     \n\
	     EXAMPLE:\n    \
	     cargo run -- l",
        )
        .arg(Arg::new("subcommands").required(true).max_values(1))
        .get_matches();

    let subcommand = args.value_of("subcommands").unwrap_or("-");

    match subcommand {
        "l" => Ok(make_list()?),
        "g" => Ok(run_groceries()?),
        "r" => Ok(run_recipes()?),
        &_ => Err("Invalid command.\n\
		   For help, try:\n\
		   cargo run -- -h"
            .into()),
    }
}

// Like run() for the shopping-list-making function in grusterylist
pub fn make_list() -> Result<(), Box<dyn Error>> {
    // Open a saved or new list
    let mut shopping_list = get_saved_or_new_list()?;

    // view list if using saved list
    if !shopping_list.groceries.is_empty() {
        print_list()?;
    }

    // add recipes
    shopping_list = add_recipes_to_list(shopping_list)?;

    // add individual groceries
    shopping_list = add_groceries_to_list(shopping_list)?;

    // overwrite saved list with current list
    save_list(shopping_list)?;

    // view list
    print_list()?;

    Ok(())
}

////////////////////////////////////////////////////////////////////////
pub fn run_groceries() -> Result<(), Box<dyn Error>> {
    print_groceries()?;

    add_groceries()?;

    Ok(())
}

////////////////////////////////////////////////////////////////////////
pub fn run_recipes() -> Result<(), Box<dyn Error>> {
    eprintln!(
        "View the recipes we have \
	 in our library?\n\
	 --y\n\
	 --any other key to continue"
    );

    if prompt_for_y()? {
        eprintln!("Here are our recipes:");

        let _ = print_recipes()?;
    }

    eprintln!(
        "Add a recipe to our library?\n\
         --y\n\
         --any other key to continue"
    );

    if prompt_for_y()? {
        eprintln!("What's the name of the recipe?");

        let recipe_name = input()?;

        eprintln!("Enter each ingredient separated by a comma");

        let ingredients = input()?;

        add_recipe(recipe_name, ingredients)?;
    }
    Ok(())
}
