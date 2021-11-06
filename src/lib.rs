use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fs::{self, File},
    io::{stdin, stdout, BufReader, Write},
    path::Path,
};

// used to serialize and deserialize a database of groceries we buy
// organized by section of our kitchen storage
#[derive(Serialize, Deserialize, Debug)]
pub struct Groceries {
    sections: Vec<GroceriesSection>,
}

// works with structure of Groceries struct
#[derive(Serialize, Deserialize, Debug)]
pub struct GroceriesSection {
    section: String,
    items: Vec<String>,
}

// to serialize and deserialize a database of recipes
#[derive(Serialize, Deserialize, Debug)]
pub struct Recipes {
    recipes: Vec<Recipe>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Recipe {
    recipe: String,
    ingredients: Vec<String>,
}

// used to serialize and deserialize the grocery list on record
// or to create a new grocery list that can be saved as JSON
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShoppingList {
    recipes_msg: String,
    recipes: Vec<String>,
    checklist_msg: String,
    checklist: Vec<String>,
    list_msg: String,
    list: Vec<String>,
}

impl ShoppingList {
    pub fn new() -> Result<ShoppingList, Box<dyn Error>> {
        Ok(ShoppingList {
            recipes_msg: "We're making ...".to_string(),
            recipes: Vec::new(),
            checklist_msg: "Check ...".to_string(),
            checklist: Vec::new(),
            list_msg: "We need ...".to_string(),
            list: Vec::new(),
        })
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

use crate::groceries::*;
mod groceries {
    use super::*;

    pub fn read_groceries<P: AsRef<Path>>(path: P) -> Result<Groceries, Box<dyn Error>> {
        let reader = read_json(path)?;
        let groceries = serde_json::from_reader(reader)?;
        Ok(groceries)
    }

    pub fn update_groceries(groceries: Groceries) -> Result<(), Box<dyn Error>> {
        let sections: Vec<GroceriesSection> = groceries.sections;
        let mut updated_groceries_sections: Vec<GroceriesSection> = Vec::new();

        for groceries_section in sections {
            eprintln!(
                "Add to our {} section?\n(\
		 y for yes, \
		 any other key for no, \
		 s to skip remaining sections)",
                groceries_section.section
            );
            match input()?.trim() {
                "y" => {
                    let items = add_groceries_to_section(groceries_section.items)?;

                    updated_groceries_sections.push(GroceriesSection {
                        section: groceries_section.section,
                        items,
                    });
                }
                "s" => break,
                &_ => {
                    updated_groceries_sections.push(GroceriesSection {
                        section: groceries_section.section,
                        items: groceries_section.items,
                    });
                }
            }
        }

        if !updated_groceries_sections.len() == 0 {
            let groceries = Groceries {
                sections: updated_groceries_sections,
            };
            let json = serde_json::to_string(&groceries)?;
            write_json("groceries.json", json)?;
        }
        Ok(())
    }

    pub fn add_groceries_to_library_prompt() -> Result<(), Box<dyn Error>> {
        eprintln!(
            "Add groceries to our library?\n(\
	     'y' for yes, \
	     any other key for no)"
        );
        while prompt_for_y()? {
            let groceries = read_groceries("groceries.json")?;
            update_groceries(groceries)?;
        }
        Ok(())
    }

    pub fn add_groceries_list(
        mut shopping_list: ShoppingList,
    ) -> Result<ShoppingList, Box<dyn Error>> {
        eprintln!(
            "Add groceries to shopping list?\n(\
	     'y' for yes, \
	     any other key to skip)"
        );
        while prompt_for_y()? {
            let groceries = read_groceries("groceries.json")?;
            shopping_list = add_groceries_to_list(shopping_list, groceries)?;
        }
        Ok(shopping_list)
    }

    fn add_groceries_to_list(
        mut shopping_list: ShoppingList,
        groceries: Groceries,
    ) -> Result<ShoppingList, Box<dyn Error>> {
        for section in &groceries.sections {
            eprintln!(
                "Do we need {}?\n(\
		 y for yes, \
		 s to skip remaining sections, \
		 any other key to continue)\n",
                section.section.to_lowercase()
            );
            match input()?.trim() {
                "y" => {
                    eprintln!(
                        "Do we need ...?\n(\
			 y for yes, \
			 c for check, \
			 s to skip to next section, \
			 any other key to continue)"
                    );
                    for item in &section.items {
                        if !shopping_list.list.contains(&item.to_lowercase()) {
                            eprintln!("{}?", item.to_lowercase());

                            match input()?.trim() {
                                "y" => shopping_list.list.push(item.to_lowercase().to_string()),
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
        Ok(shopping_list)
    }

    // takes groceries_section_items and adds user input groceries to section and returns  the section items
    pub fn add_groceries_to_section(mut items: Vec<String>) -> Result<Vec<String>, Box<dyn Error>> {
        eprintln!(
            "What shall we add? \
	     Enter the items, \
	     separated by commas"
        );
        let mut input: String = input()?;
        input.pop();
        let add_items_to_section: Vec<&str> = input.split(',').collect();

        add_items_to_section.iter().for_each(|i| {
            if !items.contains(&i.to_string()) {
                items.push(i.to_string());
            }
        });
        Ok(items)
    }
}

use crate::recipes::*;
mod recipes {
    use super::*;

    pub fn new_recipes() -> Result<(), Box<dyn Error>> {
        eprintln!(
            "Add recipes to our library?\n(\
	     'y' for yes, \
	     any other key for no)"
        );
        while prompt_for_y()? {
            let mut recipes = read_recipes("recipes.json")?;
            recipes = add_recipe_to_lib(recipes)?;
            write_recipes(recipes)?;
        }
        Ok(())
    }

    // Gets a new recipe from user and returns it as a Recipe
    fn new_recipe() -> Result<Recipe, Box<dyn Error>> {
        eprintln!("What's the recipe?");
        let mut recipe = input()?;
        recipe.pop();

        eprintln!(
            "Enter the ingredients, \
	     separated by commas"
        );
        let mut ingredients = input()?;
        ingredients.pop();
        let add_ingredients: Vec<&str> = ingredients.split(',').collect();
        let mut ingredients: Vec<String> = Vec::new();
        for i in &add_ingredients {
            if !ingredients.contains(&i.to_string()) {
                ingredients.push(i.to_string());
            }
        }
        Ok(Recipe {
            recipe,
            ingredients,
        })
    }

    // get user input when it's 'y' or anything else
    pub fn prompt_for_y() -> Result<bool, Box<dyn Error>> {
        Ok("y" == input()?.trim())
    }

    pub fn read_recipes<P: AsRef<Path>>(path: P) -> Result<Recipes, Box<dyn Error>> {
        let reader = read_json(path)?;
        let recipes = serde_json::from_reader(reader)?;
        Ok(recipes)
    }
    
    pub fn write_recipes(recipes: Recipes) -> Result<(), Box<dyn Error>> {
        let json = serde_json::to_string(&recipes)?;
        write_json("recipes.json", json)?;
        Ok(())
    }

     // adds a new recipes to library of recipes and returns Recipes
    pub fn add_recipe_to_lib(recipes: Recipes) -> Result<Recipes, Box<dyn Error>> {
        let mut updated: Vec<Recipe> = recipes.recipes;
        let new_recipe = new_recipe()?;
        updated.push(new_recipe);
        let recipes = Recipes { recipes: updated };
        Ok(recipes)
    }

    pub fn add_recipes(mut shopping_list: ShoppingList) -> Result<ShoppingList, Box<dyn Error>> {
        eprintln!(
            "Add recipe ingredients to our list?\n(\
	     'y' for yes, \
	     any other key for no)"
        );
        while prompt_for_y()? {
            let recipes = read_recipes("recipes.json")?;
            shopping_list = add_recipes_to_list(shopping_list, recipes)?;
        }
        Ok(shopping_list)
    }

    // takes shopping list and recipes library
    // and updates shopping list with recipe ingredients
    pub fn add_recipes_to_list(
        mut shopping_list: ShoppingList,
        recipes: Recipes,
    ) -> Result<ShoppingList, Box<dyn Error>> {
        eprintln!(
            "Which recipes shall we add?\n(\
	     y to add recipe, \
	     s to skip to end of recipes, \
	     any other key for next recipe)"
        );
        for r in recipes.recipes {
            eprintln!("{}?", r.recipe);
            match input()?.trim() {
                "s" => {
                    break;
                }
                "y" => {
                    shopping_list.recipes.push(r.recipe.to_owned());
                    eprintln!(
                        "Do we need ... ?\n(\
			 y to add ingredient, \
			 c to remind to check, \
			 a to add this and all remaining ingredients, \
			 any other key for next ingredient)"
                    );
                    for ingredient in &r.ingredients {
                        eprintln!("{}?", ingredient.to_lowercase());
                        match input()?.trim() {
                            "y" => {
                                if !shopping_list
                                    .list
                                    .contains(&ingredient.to_owned().to_lowercase())
                                {
                                    shopping_list
                                        .list
                                        .push(ingredient.to_owned().to_lowercase());
                                }
                            }
                            "c" => {
                                shopping_list
                                    .checklist
                                    .push(ingredient.to_owned().to_lowercase());
                            }
                            "a" => {
                                for ingredient in r.ingredients {
                                    if !shopping_list
                                        .list
                                        .contains(&ingredient.to_owned().to_lowercase())
                                    {
                                        shopping_list.list.push(ingredient);
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
        Ok(shopping_list)
    }
}

use crate::list::*;
mod list {
    use super::*;

    pub fn get_list() -> Result<ShoppingList, Box<dyn Error>> {
        let mut shopping_list = ShoppingList::new()?;
        eprintln!(
            "Use most recent list?\n(\
	     'y' for yes, \
	     any other key for new list)"
        );
        if prompt_for_y()? {
            shopping_list = read_list("list.json")?;
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
	    let shopping_list = get_list()?;
            if !shopping_list.checklist.is_empty()
		&& !shopping_list.recipes.is_empty()
		&& !shopping_list.list.is_empty()
            {
		eprintln!("Here's what we have:\n");
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
            if !shopping_list.list.is_empty() {
		println!("{}", shopping_list.list_msg);
		shopping_list.list.iter().for_each(|item| {
                    println!("\t{}", item);
		});
            }
        }
        Ok(())
    }

    pub fn read_list<P: AsRef<Path>>(path: P) -> Result<ShoppingList, Box<dyn Error>> {
        let reader = read_json(path)?;
        let shopping_list = serde_json::from_reader(reader)?;
        Ok(shopping_list)
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

use crate::helpers::*;
mod helpers {
    use super::*;

    // Function for getting user input
    pub fn input() -> Result<String, Box<dyn Error>> {
        let _ = Write::flush(&mut stdout())?;
        let mut input = String::new();
        stdin().read_line(&mut input)?;
        Ok(input)
    }

    pub fn forgotten_anything() -> Result<(), Box<dyn Error>> {
        eprintln!(
            "\nForgotten anything?\n(\
	     'y' for yes, \
	     any other key to continue)"
        );
        if prompt_for_y()? {
            run()?; // again
        }
        Ok(())
    }
}

// updating and using groceries and recipes
// assumes presence in pwd of:
// - groceries.json
// - recipes.json
// list.json will be created if not found
pub fn run() -> Result<(), Box<dyn Error>> {
    add_groceries_to_library_prompt()?;

    new_recipes()?;

    let mut shopping_list = get_list()?;

    shopping_list = add_recipes(shopping_list)?;

    shopping_list = add_groceries_list(shopping_list)?;

    save_list(shopping_list)?;

    print_list()?;

    forgotten_anything()?;

    eprintln!(
        "Bye! \
	 Happy shopping! \
	 Bon appetit!"
    );
    Ok(())
}
