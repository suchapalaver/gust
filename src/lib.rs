use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fs::{self, File},
    io::{stdin, stdout, BufReader, Write},
    path::Path,
};

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
        fs::write(path, &json)?; //.expect_err("Unable to write groceries to file");
        Ok(())
    }
}

// Groceries struct used to serialize and deserialize a database of groceries we buy
// organized by section of our kitchen storage, dairy, freezer, etc ....
#[derive(Serialize, Deserialize, Debug)]
pub struct Groceries {
    sections: Vec<GroceriesSection>,
}

// GroceriesSection struct works with structure of Groceries struct
#[derive(Serialize, Deserialize, Debug)]
pub struct GroceriesSection {
    section: String,
    items: Vec<String>,
}

// Recipes is used to serialize and deserialize a database of recipes
#[derive(Serialize, Deserialize, Debug)]
pub struct Recipes {
    recipes: Vec<Recipe>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Recipe {
    recipe: String,
    ingredients: Vec<String>,
}

// ShoppingList is used to serialize and deserialize the grocery list on record
// or to create a new grocery list that can be saved as a JSON structure
#[derive(Serialize, Deserialize, Debug)]
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

use crate::helpers::*;
mod helpers {
    use super::*;
    // Function for getting user input
    pub fn input() -> Result<String, Box<dyn Error>> {
        let _ = Write::flush(&mut stdout())?;
        let mut input = String::new();
        stdin().read_line(&mut input)?; // .expect_err("Problem with getting user input");
        Ok(input)
    }

    pub fn prompt_for_y_or_any() -> Result<bool, Box<dyn Error>> {
        let add_recipes_prompt =
            "Do we want to add more recipes to our recipes library?\n(y for yes, any other key for no)";
        eprintln!("{}", add_recipes_prompt);
        Ok("y" == input()?.trim())
    }
}

pub fn run() -> Result<(), Box<dyn Error>> {
    while prompt_for_y_or_any()? {
        eprintln!(
            "Add groceries to our library?\n(\
	     y for yes, \
	     any other key for no)"
        );
        if prompt_for_y_or_any()? {
            let reader = read_json("groceries_dict.json")?;
            let groceries: Groceries = serde_json::from_reader(reader)?;
            let sections: Vec<GroceriesSection> = groceries.sections;
            add_groceries_to_library(sections)?; // ADD GROCERIES TO MASTER LIST
        }
    }

    while prompt_for_y_or_any()? {
        eprintln!(
            "Add recipes to our library?\n(\
	     y for yes, \
	     any other key for no)"
        );
        if prompt_for_y_or_any()? {
            let reader = read_json("recipes.json")?;
            let mut recipes: Recipes = serde_json::from_reader(reader)?; //.expect_err("Problem opening recipes file")
            recipes = add_to_recipes_lib(recipes)?; // ADD RECIPES TO RECIPES LIBRARY
            let json = serde_json::to_string(&recipes)?;
            write_json("recipes.json", json)?; // .expect_err("Problem writing updated recipes to file");
        }
    }

    let mut shopping_list = ShoppingList::new()?; //.expect_err("Problem creating a new shopping list"))
    eprintln!(
        "Use most recent list?\n(\
	 y for yes, \
	 any other key for new list)"
    );
    if let "y" = input()?.trim() {
        let reader = read_json("list.json")?;
        shopping_list = serde_json::from_reader(reader)?; // .expect_err("Problem opening most recent shopping list from file")),
    }

    while prompt_for_y_or_any()? {
        eprintln!(
            "Add recipe ingredients to our list?\n(\
	     y for yes, \
	     any other key for no)"
        );
        if prompt_for_y_or_any()? {
            let reader = read_json("recipes.json")?;
            let recipes: Recipes = serde_json::from_reader(reader)?; //.expect_err("Problem reading recipes from file");
            shopping_list = add_recipes_to_list(shopping_list, recipes)?; // ADD RECIPE INGREDIENTS TO LIST
        }
    }

    while prompt_for_y_or_any()? {
        eprintln!("Add groceries to shopping list?\n(y for yes, any other key to skip)");
        if prompt_for_y_or_any()? {
            let reader = read_json("groceries.json")?;
            let groceries: Groceries = serde_json::from_reader(reader)?; // .expect_err("Problem reading groceries from file");
            shopping_list = add_groceries_to_list(shopping_list, groceries)?; // ADD TO SHOPPING LIST AND CHECKLIST
        }
    }

    let json = serde_json::to_string(&shopping_list)?;
    write_json("list.json", json)?; //.expect_err("Problem saving grocery list"); // SAVE MOST RECENT LIST

    output(shopping_list)?; // OUTPUT

    eprintln!(
        "\nForgotten anything?\n(\
	 y for yes, \
	 any other key to continue)"
    ); // ADD ANYTHING ELSE TO MASTER GROCERY LISTS, RECIPE LISTS, OR SHOPPINGLIST
    if prompt_for_y_or_any()? {
        eprintln!("Oh we have?\n...");
        run()?; //.expect_err("Problem re-running program to add additional items");
    }

    eprintln!(
        "Bye! \
	 Happy shopping! \
	 Bon appetit!"
    );
    Ok(())
}

fn add_groceries_to_library(groceries: Vec<GroceriesSection>) -> Result<(), Box<dyn Error>> {
    let mut updated_groceries: Vec<GroceriesSection> = Vec::new();

    for section in groceries {
        eprintln!(
            "Add to our {} section?\n(\
	     y for yes, \
	     any other key for no, \
	     s to skip remaining sections)",
            section.section
        );
        match input()?.trim() {
            "s" => break,
            "y" => {
                let mut items: Vec<String> = section.items;
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
                updated_groceries.push(GroceriesSection {
                    section: section.section,
                    items,
                });
            }
            &_ => {
                updated_groceries.push(GroceriesSection {
                    section: section.section,
                    items: section.items,
                });
            }
        }
    }
    if !updated_groceries.len() == 0 {
        let groceries = Groceries {
            sections: updated_groceries,
        };
        let json = serde_json::to_string(&groceries)?;
        write_json("groceries.json", json)?; // .expect_err("Unable to write updated groceries to file"); // .expect_err("Problem updating groceries")
    }
    Ok(())
}

fn add_to_recipes_lib(recipes: Recipes) -> Result<Recipes, Box<dyn Error>> {
    let mut updated: Vec<Recipe> = recipes.recipes;
    let new_recipe: Recipe = {
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
        Recipe {
            recipe,
            ingredients,
        }
    };
    updated.push(new_recipe);
    let recipes = Recipes { recipes: updated };

    Ok(recipes)
}

fn add_recipes_to_list(
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

fn output(shopping_list: ShoppingList) -> Result<(), Box<dyn Error>> {
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
    Ok(())
}
