use clap::{App, Arg};
use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fs::{self, File},
    io::{stdin, stdout, BufReader, Write},
    path::Path,
};

// assumes presence in pwd of the following
// files:
// - groceries.json
// - recipes.json
// - list.json
// working examples can be found in the
// grusterylist repository
pub fn run() -> Result<(), Box<dyn Error>> {
    let matches = App::new("grusterylist")
        .help(
            "\n\
	     grusterylist 0.1.0\n\
	     Makes grocery lists in Rust\n\
	     (C) https://github.com/suchapalaver/\n\n\
	     Usage: cargo run -- <opts>\n\n\
	     Options:\n\
	     -h, --help       Display this message\n\
	     -V, --version    Display version info\n\
	     -g, --groceries  Add groceries to groceries library\n\
	     -r, --recipes    Add recipes to recipes library\n\
	     -l, --list       Make a shopping list\n\n\
	     Examples:\n\
	     $ cargo run -- --groceries\n\
	     $ cargo run -- -r\n\n",
        )
        .arg(Arg::with_name("groceries").long("groceries").short("g"))
        .arg(Arg::with_name("recipes").short("r").long("recipes"))
        .arg(Arg::with_name("list").short("l").long("list"))
        .get_matches();

    if matches.is_present("groceries") || matches.is_present("g") {
        update_groceries()?;
    }
    if matches.is_present("recipes") || matches.is_present("r") {
        new_recipes()?;
    }
    if matches.is_present("list") || matches.is_present("l") {
        let mut shopping_list = get_saved_or_new_list()?;

        shopping_list = add_recipes_to_list(shopping_list)?;

        shopping_list = add_groceries_to_list(shopping_list)?;

        save_list(shopping_list)?;

        print_list()?;
    }

    Ok(())
}

// used to serialize and deserialize a
// database of groceries we buy organized
// by kind of by kitchen section
#[derive(Serialize, Deserialize, Debug)]
pub struct Groceries {
    sections: Vec<GroceriesSection>,
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
    library: Vec<Recipe>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Recipe {
    name: String,
    items: Vec<String>,
}

// used to serialize and deserialize the
// grocery list on record or to create a
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

    pub fn update_groceries() -> Result<(), Box<dyn Error>> {
        eprintln!(
            "Add groceries to our library?\n(\
	     Enter 'y' to add groceries,\n\
	     any other key to exit)"
        );
        while prompt_for_y()? {
            let mut updated_groceries_sections = Vec::new();
            let groceries = read_groceries("groceries.json")?;
            let groceries_sections = groceries.sections;
            for groceries_section in groceries_sections {
                eprintln!(
                    "Add to our {} section?\n(\
		     y for yes,\n\
		     any other key for no)",
                    groceries_section.name
                );
                match input()?.trim() {
                    "y" => {
                        let items = list_input(groceries_section.items)?;

			updated_groceries_sections.push(GroceriesSection {
                            name: groceries_section.name,
                            items,
                        });
                    }
                    &_ => {
                        updated_groceries_sections.push(GroceriesSection {
                            name: groceries_section.name,
                            items: groceries_section.items,
                        });
                    }
                }
            }
            let groceries = Groceries {
                sections: updated_groceries_sections,
            };
            let json = serde_json::to_string(&groceries)?;
            write_json("groceries.json", json)?;

            eprintln!(
                "Add more groceries to our library?\n(\
		 Enter 'y' to add groceries,\n\
		 any other key to exit)"
            );
        }

        Ok(())
    }

    pub fn read_groceries<P: AsRef<Path>>(path: P) -> Result<Groceries, Box<dyn Error>> {
        let reader = read_json(path)?;
        let groceries = serde_json::from_reader(reader)?;
        Ok(groceries)
    }
}

use crate::recipes::*;
mod recipes {
    use super::*;

    pub fn new_recipes() -> Result<(), Box<dyn Error>> {
        eprintln!(
            "Add recipes to our library?\n(\
	     'y' for yes,\n\
	     any other key for no)"
        );
        while prompt_for_y()? {
            let recipes = read_recipes("recipes.json")?;
            let mut updated = recipes.library;
            let new_recipe = get_new_recipe()?;
            updated.push(new_recipe);
            let recipes = Recipes { library: updated };
            save_recipes(recipes)?;
            eprintln!(
                "Add more recipes to our library?\n(\
		 'y' for yes,\n\
		 any other key for no)"
            );
        }
        Ok(())
    }

    pub fn read_recipes<P: AsRef<Path>>(path: P) -> Result<Recipes, Box<dyn Error>> {
        let reader = read_json(path)?;
        let recipes = serde_json::from_reader(reader)?;
        Ok(recipes)
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
        write_json("recipes.json", json)?;
        Ok(())
    }
}

use crate::list::*;
mod list {
    use super::*;

    pub fn get_saved_or_new_list() -> Result<ShoppingList, Box<dyn Error>> {
        let mut shopping_list = ShoppingList::new()?;
        eprintln!(
            "Use most recent list?\n(\
	     'y' for yes, \
	     any other key for new list)"
        );
        if prompt_for_y()? {
            shopping_list = read_list("list.json")?;
        }
        eprintln!("View current list?");
        if prompt_for_y()? {
            print_list()?;
        }
        Ok(shopping_list)
    }

    fn read_list<P: AsRef<Path>>(path: P) -> Result<ShoppingList, Box<dyn Error>> {
        let reader = read_json(path)?;
        let shopping_list = serde_json::from_reader(reader)?;
        Ok(shopping_list)
    }

    pub fn add_recipes_to_list(
        mut shopping_list: ShoppingList,
    ) -> Result<ShoppingList, Box<dyn Error>> {
        eprintln!(
            "Add recipe ingredients to our list?\n(\
	     'y' for yes,\n\
	     any other key for no)"
        );
        while prompt_for_y()? {
            let recipes = read_recipes("recipes.json")?;
            eprintln!(
                "Shall we add ...\n(\
		 y to add recipe,\n\
		 s to skip to end of recipes,\n\
		 any other key for next recipe)"
            );
            for recipe in recipes.library {
                eprintln!("{}?", recipe.name);
                match input()?.trim() {
                    "s" => {
                        break;
                    }
                    "y" => {
                        shopping_list.recipes.push(recipe.name.to_owned());
                        eprintln!(
                            "Do we need ... ?\n(\
			     y to add ingredient,\n\
			     c to remind to check,\n\
			     a to add this and all remaining ingredients,\n\
			     any other key for next ingredient)"
                        );
                        for ingredient in &recipe.items {
                            eprintln!("{}?", ingredient.to_lowercase());
                            match input()?.trim() {
                                "y" => {
                                    if !shopping_list
                                        .items
                                        .contains(&ingredient.to_owned().to_lowercase())
                                    {
                                        shopping_list
                                            .items
                                            .push(ingredient.to_owned().to_lowercase());
                                    }
                                }
                                "c" => {
                                    shopping_list
                                        .checklist
                                        .push(ingredient.to_owned().to_lowercase());
                                }
                                "a" => {
                                    for ingredient in recipe.items {
                                        if !shopping_list
                                            .items
                                            .contains(&ingredient.to_owned().to_lowercase())
                                        {
                                            shopping_list.items.push(ingredient);
                                        }
                                    }
                                    break;
                                }
                                &_ => {}
                            }
                        }
                    }
                    &_ => {}
                }
            }
	    eprintln!(
            "Add any more recipe ingredients to our list?\n(\
	     'y' for yes,\n\
	     any other key for no)"
        );
       
        }
        Ok(shopping_list)
    }

    pub fn add_groceries_to_list(
        mut shopping_list: ShoppingList,
    ) -> Result<ShoppingList, Box<dyn Error>> {
        eprintln!(
            "Add groceries to shopping list?\n(\
	     'y' for yes,\n\
	     any other key to skip)"
        );
        while prompt_for_y()? {
            let groceries = read_groceries("groceries.json")?;
            for groceries_section in &groceries.sections {
                eprintln!(
                    "Do we need {}?\n(\
		     y for yes,\n\
		     s to skip remaining sections, \
		     any other key to continue)\n",
                    groceries_section.name.to_lowercase()
                );
                match input()?.trim() {
                    "y" => {
                        eprintln!(
                            "Do we need ...?\n(\
			     y for yes,\n\
			     c for check,\n\
			     s to skip to next section,\n\
			     any other key to continue)"
                        );
                        for item in &groceries_section.items {
                            if !shopping_list.items.contains(&item.to_lowercase()) {
                                eprintln!("{}?", item.to_lowercase());

                                match input()?.trim() {
                                    "y" => {
                                        shopping_list.items.push(item.to_lowercase().to_string())
                                    }
                                    "c" => shopping_list
                                        .checklist
                                        .push(item.to_lowercase().to_string()),
                                    "s" => break,
                                    &_ => {}
                                }
                            }
                        }
                    }
                    "s" => break,
                    &_ => {}
                }
            }
	    eprintln!(
		"Add more groceries to shopping list?\n(\
		 'y' for yes,\n\
		 any other key to skip)"
            );
      
        }
        Ok(shopping_list)
    }

    pub fn print_list() -> Result<(), Box<dyn Error>> {
        eprintln!(
            "Print shopping list?\n\
	     ('y' for yes,\n\
	     any other key to continue)"
        );
        if prompt_for_y()? {
            let shopping_list = read_list("list.json")?;
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

    pub fn save_list(shopping_list: ShoppingList) -> Result<(), Box<dyn Error>> {
        eprintln!(
            "Save current list?\n\
	     ('y' for yes,\n\
	     any other key to continue)"
        );
        if prompt_for_y()? {
            let json = serde_json::to_string(&shopping_list)?;
            write_json("list.json", json)?;
        }
        Ok(())
    }
}

use crate::json_rw::*;
mod json_rw {
    use super::*;

    pub fn read_json<P: AsRef<Path>>(path: P) -> Result<BufReader<File>, Box<dyn Error>> {
        // Open the file in read-only mode with buffer.
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        Ok(reader)
    }

    pub fn write_json<P: AsRef<Path>>(path: P, json: String) -> Result<(), Box<dyn Error>> {
        fs::write(path, &json)?;
        Ok(())
    }
}

use crate::helpers::*;
mod helpers {
    use super::*;

    // get user input when it's 'y' or anything else
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

    pub fn list_input(mut items_list: Vec<String>) -> Result<Vec<String>, Box<dyn Error>> {
        eprintln!(
            "Enter the items,\n\
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
}
