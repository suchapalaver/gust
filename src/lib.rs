/*
macro_rules! deref_to_vec {
    ($t:ty, $target:ty) => {
        impl Deref for $t {
            type Target = Vec<$target>;

            fn deref(&self) -> &Self::Target {
                &self.collection
            }
        }
    }
}

deref_to_vec!(Groceries, GroceriesItem);
deref_to_vec!(Recipes, Recipe);
*/

// Note: Loads the contents of the module square_content from another file
//       with the same name as the module. Read more at
//       https://doc.rust-lang.org/book/ch07-05-separating-modules-into-different-files.html
mod errors;
mod groceries;
mod groceriesitem;
mod helpers;
mod recipe;
mod shoppinglist;

use std::error::Error;
use std::path::Path;

// Note: Re-exports the content of the square_content module to keep paths short.
//       Read more at https://doc.rust-lang.org/reference/items/use-declarations.html#use-visibility
pub use crate::errors::*;
pub use crate::groceries::*;
pub use crate::groceriesitem::*;
pub use crate::helpers::*;
pub use crate::recipe::*;
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
        //"r" => Ok(run_recipes()?),
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

// Prompt user whether to use a saved or new list and return their choice
fn get_saved_or_new_list() -> Result<ShoppingList, Box<dyn Error>> {
    let mut shopping_list = ShoppingList::new();

    eprintln!(
        "\n\
	     Use saved list?\n\
	     --y\n\
	     --any other key for new list"
    );

    if prompt_for_y()? {
        let path = "list.json";

        shopping_list = read_list(path).map_err(|e| {
            format!(
                "Failed to read list file '{}':\n\
		     '{}'",
                path, e
            )
        })?;
    }
    Ok(shopping_list)
}

// Open and deserialize a shopping list JSON file from given path
fn read_list<P: AsRef<Path> + Copy>(path: P) -> Result<ShoppingList, Box<dyn Error>> {
    let reader = read(path)?;

    let shopping_list = serde_json::from_reader(reader).map_err(ReadError::DeserializingError)?;

    Ok(shopping_list)
}

// Prints list
fn print_list() -> Result<(), Box<dyn Error>> {
    eprintln!(
        "\n\
	     Print out shopping list?\n\
	     --y\n\
	     --any other key to continue"
    );

    if prompt_for_y()? {
        let path = "list.json";

        let shopping_list = read_list(path).map_err(|e| {
            format!(
                "Failed to read list file '{}':\n\
		     {}",
                path, e
            )
        })?;

        // Avoid printing empty lists
        if !shopping_list.checklist.is_empty()
            && !shopping_list.recipes.is_empty()
            && !shopping_list.groceries.is_empty()
        {
            println!("Here's what we have:\n");
        }
        if !shopping_list.checklist.is_empty() {
            println!("Check if we need:");

            shopping_list.checklist.iter().for_each(|item| {
                println!("\t{}", item.name.0.to_lowercase());
            });
        }
        if !shopping_list.recipes.is_empty() {
            println!("We're making these recipes:");

            shopping_list.recipes.iter().for_each(|recipe| {
                println!("\t{}", recipe);
            });
        }
        if !shopping_list.groceries.is_empty() {
            println!("Here's what we need:");

            shopping_list.groceries.iter().for_each(|item| {
                println!("\t{}", item.name.0.to_lowercase());
            });
        }
        // Print a new line at end of output
        println!();
    }
    Ok(())
}

// Adds recipe ingredients to a shopping list
fn add_recipes_to_list(mut shopping_list: ShoppingList) -> Result<ShoppingList, Box<dyn Error>> {
    eprintln!(
        "Add recipe ingredients to our list?\n\
	 --y\n\
	 --any other key to continue"
    );

    while prompt_for_y()? {
        let path = "recipes.json";

        let recipes = Recipes::from_path(path).map_err(|e| {
            format!(
                "Failed to read recipes file '{}':\n\
		 '{}'",
                path, e
            )
        })?;

        for recipe in recipes.iter() {
            eprintln!(
                "Shall we add ...\n\
		 {}?\n\
		 --y\n\
		 --s to skip to end of recipes\n\
		 --any other key for next recipe",
                recipe
            );

            match input()?.as_str() {
                "y" => shopping_list = add_recipe_to_list(shopping_list, recipe)?,
                "s" => break,
                &_ => continue,
            }
        }
        eprintln!(
            "Add any more recipe ingredients to our list?\n\
	     --y\n\
	     --any other key to continue"
        );
    }
    Ok(shopping_list)
}

// Adds ingredients of an individual recipe to a shopping list
// AGAIN, TOTAL RESTRUCTURING NEEDED
fn add_recipe_to_list(
    mut shopping_list: ShoppingList,
    recipe: &Recipe,
) -> Result<ShoppingList, Box<dyn Error>> {
    shopping_list.recipes.push(recipe.clone());
    /*
       eprintln!(
           "Do we need ... ?\n\
        --y\n\
        --c to remind to check\n\
        --a to add this and all remaining ingredients\n\
        --any other key for next ingredient"
       );
    */

    //let recipe_items = recipe.items;
    /*
        for ingredient in &recipe_items.0 {
            eprintln!("{}?", ingredient.0.to_lowercase());

            match input()?.as_str() {
                "y" => shopping_list = add_ingredient_to_list(shopping_list, ingredient)?,
                "c" => shopping_list = add_ingredient_to_checklist(shopping_list, ingredient)?,
                "a" => {
                    shopping_list = add_all_ingredients_to_list(shopping_list, recipe_items.0)?;
                    break;
                }
                &_ => continue,
            }
        }
    */
    Ok(shopping_list)
}
/*
// Adds individual ingredients to a shopping list
    fn add_ingredient_to_list(
        mut shopping_list: ShoppingList,
        ingredient: &GroceriesItem,
    ) -> Result<ShoppingList, Box<dyn Error>> {
        if !shopping_list
            .items
            .0
            .contains(&GroceriesItem(ingredient.0.to_lowercase()))
        {
            shopping_list
                .items
                .0
                .push(GroceriesItem(ingredient.0.to_lowercase()));
        }
        Ok(shopping_list)
    }

// Adds all ingredients in a single recipe to list
    fn add_all_ingredients_to_list(
        mut shopping_list: ShoppingList,
        recipe_items: Vec<GroceriesItem>,
    ) -> Result<ShoppingList, Box<dyn Error>> {
        for ingredient in recipe_items {
            // Avoid adding repeat items to list
            if !shopping_list
                .items
                .0
                .contains(&GroceriesItem(ingredient.0.to_lowercase()))
            {
                shopping_list.items.0.push(ingredient);
            }
        }
        Ok(shopping_list)
    }

// Adds ingredients to checklist on shopping list
    fn add_ingredient_to_checklist(
        mut shopping_list: ShoppingList,
        ingredient: &GroceriesItem,
    ) -> Result<ShoppingList, Box<dyn Error>> {
        shopping_list
            .checklist
            .0
            .push(GroceriesItem(ingredient.0.to_lowercase()));

        Ok(shopping_list)
    }
*/

// Adds groceries to list
// NEEDS TO ADD *ALL* RECIPE INGREDIENTS FIRST
fn add_groceries_to_list(mut shopping_list: ShoppingList) -> Result<ShoppingList, Box<dyn Error>> {
    // BEST TO COMPILE THE LIST AT THE END
    let path = "groceries.json";

    let groceries = Groceries::from_path(path)?;

    for groceriesitem in groceries.iter() {
        if !shopping_list.groceries.contains(groceriesitem) && groceriesitem.is_recipe_ingredient {
            for recipe in &groceriesitem.recipes {
                if shopping_list.recipes.contains(recipe) {
                    shopping_list.groceries.push(groceriesitem.clone());
                }
            }
        }
    }

    eprintln!(
        "Add groceries to shopping list?\n\
	 --y\n\
	 --any other key to skip"
    );

    while prompt_for_y()? {
        //let path = "groceries.json";

        let groceries = Groceries::from_path(path).map_err(|e| {
            format!(
                "Failed to read groceries file '{}':\n\
		 '{}'",
                path, e
            )
        })?;

        shopping_list = add_grocery_sections_to_list(shopping_list, groceries)?;

        eprintln!(
            "Add more groceries to shopping list?\n\
	     --y\n\
	     --any other key to skip"
        );
    }
    Ok(shopping_list)
}

fn add_grocery_sections_to_list(
    shopping_list: ShoppingList,
    groceries: Groceries,
) -> Result<ShoppingList, Box<dyn Error>> {
    let sections = vec!["fresh", "pantry", "dairy", "protein", "freezer"];

    let groceries_by_section: Vec<Vec<GroceriesItem>> = {
        sections
            .into_iter()
            .map(|x| fum(&groceries, &shopping_list, x))
            .collect()
    };

    let shopping_list = foo(groceries_by_section, shopping_list)?;

    Ok(shopping_list)
}

fn fum(groceries: &Groceries, shopping_list: &ShoppingList, section: &str) -> Vec<GroceriesItem> {
    groceries
        .iter()
        .cloned()
        .filter(|x| !shopping_list.groceries.contains(x) && x.section.0 == section)
        .collect()
}

fn foo(
    groceries_sections: Vec<Vec<GroceriesItem>>,
    mut shopping_list: ShoppingList,
) -> Result<ShoppingList, Box<dyn Error>> {
    for groceries_section in groceries_sections {
        if !groceries_section.is_empty() {
            for groceriesitem in groceries_section {
                eprintln!(
                    "Do we need {}?\n\
		     --y\n\
		     --s to skip remaining sections\n\
		     --any other key to continue",
                    &groceriesitem.name.0.to_lowercase()
                );

                match input()?.as_str() {
                    "y" => {
                        //shopping_list = add_grocery_section_to_list(shopping_list, groceries_section)?
                        shopping_list.groceries.push(groceriesitem.clone());
                    }
                    "s" => break,
                    &_ => continue,
                }
            }
        }
    }
    Ok(shopping_list)
}

// Saves shopping list
fn save_list(shopping_list: ShoppingList) -> Result<(), Box<dyn Error>> {
    eprintln!(
        "Save current list?\n\
	 --y\n\
	 --any other key to continue"
    );

    if prompt_for_y()? {
        let json = serde_json::to_string(&shopping_list)?;
        // Put trace here
        write("list.json", json)?;
    }
    Ok(())
}

/*
pub fn run_recipes() -> Result<(), Box<dyn Error>> {
    let _ = view_recipes()?;

    let _ = new_recipes()?; // ***

    Ok(())
}

pub fn view_recipes() -> Result<(), Box<dyn Error>> {
    eprintln!(
        "View the recipes we have \
     in our library?\n\
     --y\n\
     --any other key to continue"
    );

    if prompt_for_y()? {
        print_recipes()?;
    }
    Ok(())
}
*/

pub fn run_groceries() -> Result<(), Box<dyn Error>> {
    let _ = update_groceries()?;
    Ok(())
}

fn update_groceries() -> Result<(), Box<dyn Error>> {
    eprintln!(
        "Add groceries to our library?\n\
         --y\n\
         --any other key to exit"
    );

    while prompt_for_y()? {
        let path = "groceries.json";

        let groceries = Groceries::from_path(path).map_err(|e| {
            format!(
                "Failed to read groceries file '{}':\n\
             '{}'\n",
                path, e
            )
        })?;

        let groceries = update_groceries_sections(groceries)?;

        let json = serde_json::to_string(&groceries)?;

        write(path, json)?;

        eprintln!(
            "Add more groceries to our library?\n\
		 --y\n\
		 --any other key to exit"
        );
    }
    Ok(())
}

fn update_groceries_sections(mut groceries: Groceries) -> Result<Groceries, Box<dyn Error>> {
    groceries = list_input(groceries)?;

    /*
    let mut updated_groceries_sections = Vec::new();

    let groceries_sections = groceries.sections;

    for groceries_section in groceries_sections.0 {
        eprintln!(
            "Add to our {} section?\n\
         --y\n\
         --any other key to continue",
            groceries_section.name
        );

        if prompt_for_y()? {
            let items = list_input(groceries_section.items.0)?;

            updated_groceries_sections.push(GroceriesSection {
                name: groceries_section.name,
                items: GroceriesItems(items),
            });
        } else {
            updated_groceries_sections.push(GroceriesSection {
                name: groceries_section.name,
                items: groceries_section.items,
            });
        }
    }
     */

    Ok(groceries)
}

// Input a list and return it ...
pub fn list_input(mut groceries: Groceries) -> Result<Groceries, Box<dyn Error>> {
    /*
    eprintln!(
    "Enter the items, \
         separated by commas"
    );
     */
    eprintln!(
        "Enter the name, and\n\
	 1 for fresh\n\
         2 for pantry\n\
         3 for protein\n\
	 4 for dairy \n\
	 5 for freezer \n\
         separated by commas,\n\
	 e.g. 'sausages, protein'"
    );
    let input = input()?;

    let input_list: Vec<String> = input
        .split(',')
        .map(|item| item.trim().to_lowercase())
        .collect();

    //input_list.iter().for_each(|item| {
    if groceries.iter().all(|x| x.name.0 != input_list[0]) {
        //if !groceries.contains(&input_list[0]) {
        groceries.push(GroceriesItem::new(input_list)?);
    }
    //});

    Ok(groceries)
}

/*
    fn add_grocery_section_to_list(
        mut shopping_list: ShoppingList,
        groceries_section: GroceriesSection,
    ) -> Result<ShoppingList, Box<dyn Error>> {
        eprintln!(
            "Do we need ...?\n\
         --y\n\
         --c to check later\n\
         --s to skip to next section\n\
         --any other key to continue"
        );

        for item in groceries_section.items.0 {
            // https://stackoverflow.com/questions/45624813/how-can-i-unpack-a-tuple-struct-like-i-would-a-classic-tuple/45624862
            // the .0. is indexing the String wrapped in the tuple struct
            if !shopping_list
                .items
                .0
                .contains(&GroceriesItem(item.0.to_lowercase()))
            {
                eprintln!("{}?", item.0.to_lowercase());

                match input()?.as_str() {
                    // unpack the tuple, mutate the contents,
                    // rewrap the changes in the tuple struct
                    "y" => shopping_list
                        .items
                        .0
                        .push(GroceriesItem(item.0.to_lowercase())),
                    "c" => shopping_list
                        .checklist
                        .0
                        .push(GroceriesItem(item.0.to_lowercase())),
                    // skip remaining sections
                    "s" => break,
                    &_ => continue,
                }
            }
        }
        Ok(shopping_list)
    }

*/
