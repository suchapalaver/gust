use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fs::{self, write, File},
    io::{stdin, stdout, BufReader, Write},
    path::Path,
};

mod json {
    use super::*;

    pub fn read_json<P: AsRef<Path>>(path: P) -> Result<BufReader<File>, Box<dyn Error>> {
        // Open the file in read-only mode with buffer.
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        Ok(reader)
    }

    pub fn write_groceries<P: AsRef<Path>>(
        groceries: Groceries,
        path: P,
    ) -> Result<(), Box<dyn Error>> {
        let json = serde_json::to_string(&groceries)?; //.expect_err("Unable to serialize groceries as JSON string");
        write(path, &json)?; //.expect_err("Unable to write groceries to file");
        Ok(())
    }

    pub fn write_recipes_to_file<P: AsRef<Path>>(
        recipes: Recipes,
        path: P,
    ) -> Result<(), Box<dyn Error>> {
        let json: String = serde_json::to_string(&recipes)?;
        fs::write(path, &json)?; //.expect_err("Unable to write recipes to file");
        Ok(())
    }

    pub fn write_shopping_list_to_file<P: AsRef<Path>>(
        shopping: &ShoppingList,
        path: P,
    ) -> Result<(), Box<dyn Error>> {
        let json: String = serde_json::to_string(&shopping)?;
        fs::write(path, &json)?; //.expect_err("Unable to write shopping list to file");
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
struct Recipe {
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
    fn new() -> Result<ShoppingList, Box<dyn Error>> {
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

pub fn run() -> Result<(), Box<dyn Error>> {
    let mut done = false;
    while !done {
        let add_groceries_prompt = "Do we want to add any more items to our big list?\n(\
	     y for yes, \
	     any other key for no)";
        eprintln!("{}", add_groceries_prompt);
        if let "y" = input()?.trim() {
            let reader = json::read_json("groceries_dict.json")?;
            let groceries: Groceries = serde_json::from_reader(reader)?;
            let sections: Vec<GroceriesSection> = groceries.sections;
            add_groceries_to_library(sections)?; // ADD GROCERIES TO MASTER LIST
        } else {
            done = true;
        }
    }

    let mut no_need_to_add_to_recipes = false;
    while !no_need_to_add_to_recipes {
        let add_recipes_prompt =
            "Do we want to add more recipes to our recipes library?\n(y for yes, any other key for no)";
        eprintln!("{}", add_recipes_prompt);
        if let "y" = input()?.trim() {
            let reader = json::read_json("recipes.json")?;
            let mut recipes: Recipes = serde_json::from_reader(reader)?; //.expect_err("Problem opening recipes file")
            recipes = add_to_recipes_lib(recipes)?; // ADD RECIPES TO RECIPES LIBRARY
            json::write_recipes_to_file(recipes, "recipes.json")
                .expect_err("Problem writing updated recipes to file");
        } else {
            no_need_to_add_to_recipes = true;
        }
    }

    let mut shopping_list = ShoppingList::new()?; //.expect_err("Problem creating a new shopping list"))
    eprintln!(
        "Use most recent list?\n(\
	 y for yes, \
	 any other key for new list)"
    );
    if let "y" = input()?.trim() {
        let reader = json::read_json("list.json")?;
        shopping_list = serde_json::from_reader(reader)?; // .expect_err("Problem opening most recent shopping list from file")),
    }

    let mut done_adding_recipe_ingredients_to_shopping_list = false;
    while !done_adding_recipe_ingredients_to_shopping_list {
        eprintln!(
            "Add recipe ingredients to our list?\n(\
	     y for yes, \
	     any other key for no)"
        );
        if let "y" = input()?.trim() {
            let reader = json::read_json("recipes.json")?;
            let recipes: Recipes = serde_json::from_reader(reader)?; //.expect_err("Problem reading recipes from file");
            shopping_list = add_recipes_to_list(shopping_list, recipes)?; // ADD RECIPE INGREDIENTS TO LIST
        } else {
            done_adding_recipe_ingredients_to_shopping_list = true;
        }
    }

    let mut done_adding_groceries_to_list = false;
    while !done_adding_groceries_to_list {
        eprintln!("Add groceries to shopping list?\n(y for yes, any other key to skip)");
        if let "y" = input()?.trim() {
            let reader = json::read_json("groceries.json")?;
            let groceries: Groceries = serde_json::from_reader(reader)?; // .expect_err("Problem reading groceries from file");
                                                                         // json::read_groceries_from_file("groceries_dict.json")?;
            shopping_list = add_groceries_to_list(shopping_list, groceries)?; // ADD TO SHOPPING LIST AND CHECKLIST
        } else {
            done_adding_groceries_to_list = true;
        }
    }

    json::write_shopping_list_to_file(&shopping_list, "most_recent_grocery_list.json")?; //.expect_err("Problem saving grocery list"); // SAVE MOST RECENT LIST

    output(shopping_list)?; // OUTPUT

    eprintln!("\nForgotten anything?\n(y for yes, any other key to continue)"); // ADD ANYTHING ELSE TO MASTER GROCERY LISTS, RECIPE LISTS, OR SHOPPINGLIST

    if let "y" = input()?.trim() {
        eprintln!("Oh we have?\n...");
        run()?; //.expect_err("Problem re-running program to add additional items");
    } else {
    }

    eprintln!("Bye! Happy shopping! Bon appetit!");

    Ok(())
}

// Function for getting user input
fn input() -> Result<String, Box<dyn Error>> {
    let _ = Write::flush(&mut stdout())?;
    let mut input = String::new();
    stdin().read_line(&mut input)?; // .expect_err("Problem with getting user input");
    Ok(input)
}

fn add_groceries_to_library(groceries: Vec<GroceriesSection>) -> Result<(), Box<dyn Error>> {
    let mut updated_groceries: Vec<GroceriesSection> = Vec::new();

    for section in groceries {
        eprintln!(
	    "Add to our {} section?\n(y for yes, any other key for no, s to skip remaining sections)",
	    section.section
	);
        match input()?.trim() {
            "s" => break,
            "y" => {
                let mut items: Vec<String> = section.items;

                eprintln!("What shall we add? Enter the items, separated by commas");

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
        crate::json::write_groceries(groceries, "groceries_dict.json")?; // .expect_err("Unable to write updated groceries to file"); //.expect_err("Problem updating groceries")
    }
    Ok(())
}

fn add_to_recipes_lib(recipes: Recipes) -> Result<Recipes, Box<dyn Error>> {
    let mut updated: Vec<Recipe> = recipes.recipes;

    let new_recipe: Recipe = {
        eprintln!("What's the name of the recipe we're adding?");

        let mut recipe = input()?;

        recipe.pop();

        eprintln!("Enter the ingredients, separated by commas");

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
