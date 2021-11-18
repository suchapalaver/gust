use clap::{App, Arg};
use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fmt,
    fs::{self, File},
    io::{stdin, stdout, BufReader, Write},
    path::Path,
};

pub fn run() -> Result<(), Box<dyn Error>> {
    let matches = App::new("grusterylist")
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
	.arg(Arg::new("subcommands")
	     .required(true)
	     .max_values(1)
	)
	.get_matches();
    
    match matches.value_of("subcommands").unwrap() {
	"l" => Ok(make_list()?),
	"g" => Ok(update_groceries()?),
	"r" => Ok(new_recipes()?),
	&_ => Err("Invalid command.\n\
		   For help, try:\n\
		   cargo run -- -h".into()),
    }
}

// Customized handling of file reading errors
#[derive(Debug)]
pub enum ReadError {
    DeserializingError(serde_json::Error),
    PathError(Box<dyn Error>),
}

// Yup, you can't just return some string as an error message
impl fmt::Display for ReadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ReadError::DeserializingError(e) => write!(
                f,
                "Error deserializing from JSON file:\n\
                 '{}'!\n\
		 Something's wrong with the JSON file?\n\
		 See the example json files in the \
		 grusterylist repository to see \
		 how things should look.\n",
                e
            ),
            ReadError::PathError(e) => write!(
                f,
                "Error: '{}'!\n\
		 Make sure file with that path \
		 can be accessed by the \
		 present working directory",
                e
            ),
        }
    }
}

// This is to make compatibility with the chain of Box<dyn Error> messaging
impl Error for ReadError {
    fn description(&self) -> &str {
        match *self {
            ReadError::DeserializingError(_) => "Error deserializing from JSON file!",
            ReadError::PathError(_) => "File does not exist!",
        }
    }
}

// used to serialize and deserialize a
// database of groceries we buy organized
// by kind of by kitchen section
#[derive(Serialize, Deserialize, Debug)]
pub struct Groceries {
    groceries_sections: Vec<GroceriesSection>,
}

// works with structure of Groceries struct
#[derive(Serialize, Deserialize, Debug)]
pub struct GroceriesSection {
    name: String,
    items: Vec<String>,
}

// to serialize and deserialize a database of recipes
#[derive(Serialize, Deserialize, Debug)]
pub struct Recipes {
    recipes_library: Vec<Recipe>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Recipe {
    name: String,
    items: Vec<String>,
}

// used to serialize and deserialize the
// most recently saved list or to create a
// new grocery list that can be saved as JSON
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShoppingList {
    recipes_msg: String,
    recipes: Vec<String>,
    checklist_msg: String,
    checklist: Vec<String>,
    items_msg: String,
    items: Vec<String>,
}

// This is what we want to happen each time we create
// a new shopping list
impl ShoppingList {
    pub fn new() -> Result<ShoppingList, Box<dyn Error>> {
        Ok(ShoppingList {
            recipes_msg: "We're making ...".to_string(),
            recipes: Vec::new(),
            checklist_msg: "Check ...".to_string(),
            checklist: Vec::new(),
            items_msg: "We need ...".to_string(),
            items: Vec::new(),
        })
    }
}

use crate::groceries::*;
mod groceries {
    use super::*;

    pub fn read_groceries<P: AsRef<Path> + Copy>(path: P) -> Result<Groceries, Box<dyn Error>> {
        let reader = read(path)?;

        let groceries = serde_json::from_reader(reader)
            .map_err(ReadError::DeserializingError)?;

        Ok(groceries)
    }

    pub fn update_groceries() -> Result<(), Box<dyn Error>> {
        eprintln!(
            "Add groceries to our library?\n\
	     --y\n\
	     --any other key to exit"
        );

        while prompt_for_y()? {
            let path = "groceries.json";

            let groceries = read_groceries(path).map_err(|e| {
                format!(
                    "Failed to read groceries file '{}':\n\
		     {}\n",
                    path, e
                )
            })?;

            let updated_groceries_sections = update_groceries_sections(groceries)?;

            let groceries = updated_groceries_sections;

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

    fn update_groceries_sections(
        groceries: Groceries,
    ) -> Result<Vec<GroceriesSection>, Box<dyn Error>> {
        let mut updated_groceries_sections = Vec::new();

	let groceries_sections = groceries.groceries_sections;
	
        for groceries_section in groceries_sections {
            eprintln!(
                "Add to our {} section?\n\
		 --y\n\
		 --any other key to continue",
                groceries_section.name
            );

            if prompt_for_y()? {
                let items = list_input(groceries_section.items)?;

                updated_groceries_sections.push(GroceriesSection {
                    name: groceries_section.name,
                    items,
                });
            } else {
                updated_groceries_sections.push(GroceriesSection {
                    name: groceries_section.name,
                    items: groceries_section.items,
                });
            }
        }
        Ok(updated_groceries_sections)
    }
}

use crate::recipes::*;
mod recipes {
    use super::*;

    pub fn read_recipes<P: AsRef<Path> + Copy>(path: P) -> Result<Recipes, Box<dyn Error>> {
        let reader = read(path)?;

        let recipes = serde_json::from_reader(reader)
            .map_err(ReadError::DeserializingError)?;

        Ok(recipes)
    }

    pub fn new_recipes() -> Result<(), Box<dyn Error>> {
	let _ = view_recipes()?;
	
        eprintln!(
            "Add recipes to our library?\n\
	     --y\n\
	     --any other key to continue"
        );

        while prompt_for_y()? {
            let path = "recipes.json";

            let recipes = read_recipes(path).map_err(|e| {
                format!(
                    "Failed to read recipes file '{}':\n\
		     {}",
                    path, e
                )
            })?;

            let mut updated = recipes.recipes_library;

            let new_recipe = get_new_recipe()?;

            updated.push(new_recipe);

            let recipes = Recipes { recipes_library: updated };

            save_recipes(recipes)?;

            eprintln!(
                "Add more recipes to our library?\n\
		 --y\n\
		 --any other key to exit"
            );
        }
        Ok(())
    }

    fn view_recipes() -> Result<(), Box<dyn Error>> {
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

    fn print_recipes() -> Result<(), Box<dyn Error>> {
        let path = "recipes.json";

        let recipes = read_recipes(path).map_err(|e| {
            format!(
                "Failed to read recipes file '{}':\n\
		 {}",
                path, e
            )
        })?;

        eprintln!("Here are our recipes:");

        for recipe in recipes.recipes_library {
            eprintln!("- {}", recipe.name.to_string());
        }
        eprintln!();

        Ok(())
    }

    // Gets a new recipe from user
    // and returns it as a Recipe
    fn get_new_recipe() -> Result<Recipe, Box<dyn Error>> {
        eprintln!("What's the recipe?");

        let mut name = input()?;

        name.pop();

        let items_list = Vec::new();

        let items = list_input(items_list)?;

	Ok(Recipe { name, items })
    }

    fn save_recipes(recipes: Recipes) -> Result<(), Box<dyn Error>> {
        let json = serde_json::to_string(&recipes)?;

        write("recipes.json", json)?;

        Ok(())
    }
}

use crate::list::*;
mod list {
    use super::*;

    // Like run() for the shopping-list-making function in grusterylist
    pub fn make_list() -> Result<(), Box<dyn Error>> {
	// Open a saved or new list
        let mut shopping_list = get_saved_or_new_list()?;

	// view list if using saved list
	if !shopping_list.items.is_empty() {
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
        let mut shopping_list = ShoppingList::new()?;

        eprintln!(
            "Use saved list?\n\
	     --y\n\
	     --any other key for new list"
        );
        if prompt_for_y()? {
            let path = "list.json";
            shopping_list = read_list(path).map_err(|e| {
                format!(
                    "Failed to read list file '{}':\n\
		     {}",
                    path, e
                )
            })?;
        }
        Ok(shopping_list)
    }

    // Open and deserialize a shopping list JSON file from given path
    fn read_list<P: AsRef<Path> + Copy>(path: P) -> Result<ShoppingList, Box<dyn Error>> {
        let reader = read(path)?;

        let shopping_list = serde_json::from_reader(reader)
            .map_err(ReadError::DeserializingError)?;

        Ok(shopping_list)
    }

    // Adds recipe ingredients to a shopping list
    fn add_recipes_to_list(
        mut shopping_list: ShoppingList,
    ) -> Result<ShoppingList, Box<dyn Error>> {
        eprintln!(
            "Add recipe ingredients to our list?\n\
	     --y\n\
	     --any other key to continue"
        );
        while prompt_for_y()? {
            let path = "recipes.json";

            let recipes = read_recipes(path).map_err(|e| {
                format!(
                    "Failed to read recipes file '{}':\n\
		     {}",
                    path, e
                )
            })?;

            for recipe in recipes.recipes_library {
                eprintln!(
                    "Shall we add ...\n\
		     {}?\n\
		     --y\n\
		     --s to skip to end of recipes\n\
		     --any other key for next recipe",
                    recipe.name
                );

                match input()?.trim() {
                    "y" => shopping_list = add_recipe_to_list(shopping_list, recipe)?,
                    "s" => break,
                    &_ => {}
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
    fn add_recipe_to_list(
        mut shopping_list: ShoppingList,
        recipe: Recipe,
    ) -> Result<ShoppingList, Box<dyn Error>> {
        shopping_list.recipes.push(recipe.name);

        eprintln!(
            "Do we need ... ?\n\
	     --y\n\
	     --c to remind to check\n\
	     --a to add this and all remaining ingredients\n\
	     --any other key for next ingredient"
        );

        let recipe_items = recipe.items;

        for ingredient in &recipe_items {
            eprintln!("{}?", ingredient.to_lowercase());

            match input()?.trim() {
                "y" => shopping_list = add_ingredient_to_list(shopping_list, ingredient)?,
                "c" => shopping_list = add_ingredient_to_checklist(shopping_list, ingredient)?,
                "a" => {
                    shopping_list = add_all_ingredients_to_list(shopping_list, recipe_items)?;
                    break;
                }
                &_ => {}
            }
        }
        Ok(shopping_list)
    }

    // Adds individual ingredients to a shopping list
    fn add_ingredient_to_list(
        mut shopping_list: ShoppingList,
        ingredient: &str,
    ) -> Result<ShoppingList, Box<dyn Error>> {
        if !shopping_list.items.contains(&ingredient.to_lowercase()) {
            shopping_list.items.push(ingredient.to_lowercase());
        }
        Ok(shopping_list)
    }

    // Adds all ingredients in a single recipe to list
    fn add_all_ingredients_to_list(
        mut shopping_list: ShoppingList,
        recipe_items: Vec<String>,
    ) -> Result<ShoppingList, Box<dyn Error>> {
        for ingredient in recipe_items {
	    // Avoid adding repeat items to list
            if !shopping_list.items.contains(&ingredient.to_lowercase()) {
                shopping_list.items.push(ingredient);
            }
        }
        Ok(shopping_list)
    }

    // Adds ingredients to checklist on shopping list
    fn add_ingredient_to_checklist(
        mut shopping_list: ShoppingList,
        ingredient: &str,
    ) -> Result<ShoppingList, Box<dyn Error>> {
        shopping_list.checklist.push(ingredient.to_lowercase());

        Ok(shopping_list)
    }

    // Adds groceries to list
    fn add_groceries_to_list(
        mut shopping_list: ShoppingList,
    ) -> Result<ShoppingList, Box<dyn Error>> {
        eprintln!(
            "Add groceries to shopping list?\n\
	     --y\n\
	     --any other key to skip"
        );

        while prompt_for_y()? {
            let path = "groceries.json";

            let groceries = read_groceries(path).map_err(|e| {
                format!(
                    "Failed to read groceries file '{}':\n\
		     {}",
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
        mut shopping_list: ShoppingList,
        groceries: Groceries,
    ) -> Result<ShoppingList, Box<dyn Error>> {
        let groceries_sections = groceries.groceries_sections;

        for groceries_section in groceries_sections {
            eprintln!(
                "Do we need {}?\n\
		 --y\n\
		 --s to skip remaining sections\n\
		 --any other key to continue",
                groceries_section.name.to_lowercase()
            );

            match input()?.trim() {
                "y" => {
                    shopping_list = add_grocery_section_to_list(shopping_list, groceries_section)?
                }
                "s" => break,
                &_ => {}
            }
        }
        Ok(shopping_list)
    }

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

        for item in groceries_section.items {
            if !shopping_list.items.contains(&item.to_lowercase()) {
                eprintln!("{}?", item.to_lowercase());

                match input()?.trim() {
                    "y" => shopping_list.items.push(item.to_lowercase().to_string()),
                    "c" => shopping_list
                        .checklist
                        .push(item.to_lowercase().to_string()),
                    "s" => break,
                    &_ => {}
                }
            }
        }
        Ok(shopping_list)
    }

    // Prints list
    fn print_list() -> Result<(), Box<dyn Error>> {
        eprintln!(
            "Print out shopping list?\n\
	     --y\n\
	     --any other key to continue"
        );

        if prompt_for_y()? {
            let path = "list.json";

	    // Open shopping list
            let shopping_list = read_list(path).map_err(|e| {
                format!(
                    "Failed to read list file '{}':\n\
		     {}",
                    path, e
                )
            })?;

	    // Avoid printing things if they're empty
            if !shopping_list.checklist.is_empty()
                && !shopping_list.recipes.is_empty()
                && !shopping_list.items.is_empty()
            {
                println!("Here's what we have:\n");
            }
	    if !shopping_list.checklist.is_empty() {
                println!("{}", shopping_list.checklist_msg);

                shopping_list.checklist.iter().for_each(|item| {
                    println!("\t{}", item.to_lowercase());
                });
            }
	    if !shopping_list.recipes.is_empty() {
                println!("{}", shopping_list.recipes_msg);

                shopping_list.recipes.iter().for_each(|recipe| {
                    println!("\t{}", recipe);
                });
            }
	    if !shopping_list.items.is_empty() {
                println!("{}", shopping_list.items_msg);

                shopping_list.items.iter().for_each(|item| {
                    println!("\t{}", item);
                });
            }
            println!();
        }
        Ok(())
    }

    // Saves shopping list
    pub fn save_list(shopping_list: ShoppingList) -> Result<(), Box<dyn Error>> {
        eprintln!(
            "Save current list?\n\
	     --y\n\
	     --any other key to continue"
        );

        if prompt_for_y()? {
            let json = serde_json::to_string(&shopping_list)?;

            write("list.json", json)?;
        }
        Ok(())
    }
}

use crate::helpers::*;
mod helpers {
    use super::*;

    // Gets user input when it's 'y' or anything else
    pub fn prompt_for_y() -> Result<bool, Box<dyn Error>> {
        Ok("y" == input()?.trim())
    }

    // Function for getting user input
    pub fn input() -> Result<String, Box<dyn Error>> {
        let _ = Write::flush(&mut stdout())?;

        let mut input = String::new();

        stdin().read_line(&mut input)?;

        Ok(input)
    }

    // Input a list and return it having added a list of user input strings 
    pub fn list_input(mut items_list: Vec<String>) -> Result<Vec<String>, Box<dyn Error>> {
        eprintln!(
            "Enter the items, \
	     separated by commas"
        );

        let mut input_string = input()?;

        input_string.pop();

        let input_list: Vec<&str> = input_string.split(',').collect();

        input_list.iter().for_each(|i| {
            if !items_list.contains(&i.to_string()) {
                items_list.push(i.to_string());
            }
        });

        Ok(items_list)
    }

    // Reads from a path into a buffer-reader
    pub fn read<P: AsRef<Path>>(path: P) -> Result<BufReader<File>, Box<dyn Error>> {
        // Open the file in read-only mode with buffer.
        let file = File::open(path).map_err(|err_msg| ReadError::PathError(Box::from(err_msg)))?;

        let reader = BufReader::new(file);

        Ok(reader)
    }

    // Writes a String to a path
    pub fn write<P: AsRef<Path>>(path: P, object: String) -> Result<(), Box<dyn Error>> {
        let _ = fs::write(path, &object)?;
        Ok(())
    }
}
