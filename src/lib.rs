use serde::{Deserialize, Serialize};
use std::{
    fs::{self, write, File},
    io::{stdin, stdout, BufReader, Result, Write},
    path::Path,
};

#[derive(Serialize, Deserialize, Debug)]
struct Groceries {
    sections: Vec<GroceriesSection>,
}

impl Groceries {
    fn write_to_file<P: AsRef<Path>>(groceries: Groceries, path: P) -> Result<()> {
        let json: String = serde_json::to_string(&groceries).unwrap();
        write(path, &json).expect("Unable to write file");
        Ok(())
    }

    fn read_file<P: AsRef<Path>>(path: P) -> Result<Groceries> {
        // Open the file in read-only mode with buffer.
        let reader = BufReader::new(File::open(path).expect("file issue"));
        let groceries: Groceries = serde_json::from_reader(reader).expect("reader issue");
        Ok(groceries)
    }

    fn update(updated_items: Vec<GroceriesSection>) -> Result<()> {
        let groceries = Groceries {
            sections: updated_items,
        };
        Groceries::write_to_file(groceries, "groceries_dict.json").unwrap();
        Ok(())
    }

    fn get_sections() -> Result<Vec<GroceriesSection>> {
        let groceries: Groceries =
            Groceries::read_file("groceries_dict.json").expect("Unable to read file");
        let sections: Vec<GroceriesSection> = groceries.sections;
        Ok(sections)
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct GroceriesSection {
    section: String,
    items: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Recipes {
    recipes: Vec<Recipe>,
}

impl Recipes {
    fn write_to_file<P: AsRef<Path>>(recipes: Recipes, path: P) -> Result<()> {
        let json: String = serde_json::to_string(&recipes).unwrap();
        fs::write(path, &json).expect("Unable to write file");
        Ok(())
    }

    fn from_file<P: AsRef<Path>>(path: P) -> Result<Recipes> {
        // Open the file in read-only mode with buffer.
        let reader = BufReader::new(File::open(path).expect("file issue"));
        let recipes: Recipes = serde_json::from_reader(reader).expect("reader issue");
        Ok(recipes)
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Recipe {
    recipe: String,
    ingredients: Vec<String>,
}

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
    fn new() -> Result<ShoppingList> {
        Ok(ShoppingList {
            recipes_msg: "We're making ...".to_string(),
            recipes: Vec::new(),
            checklist_msg: "Check ...".to_string(),
            checklist: Vec::new(),
            list_msg: "We need ...".to_string(),
            list: Vec::new(),
        })
    }

    fn from_file<P: AsRef<Path>>(path: P) -> Result<ShoppingList> {
        // Open the file in read-only mode with buffer.
        let reader = BufReader::new(File::open(path).expect("file issue"));
        let shopping: ShoppingList = serde_json::from_reader(reader).expect("reader issue");
        Ok(shopping)
    }

    fn write_to_file<P: AsRef<Path>>(shopping: ShoppingList, path: P) -> Result<()> {
        let json: String = serde_json::to_string(&shopping).unwrap();
        fs::write(path, &json).expect("Unable to write file");
        Ok(())
    }

    pub fn get() -> Result<ShoppingList> {
        println!("Use most recent list?\n(y for yes, any other key for new list)");
        let _ = Write::flush(&mut stdout());
        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .expect("problem with user input");
        match input.trim() {
            "y" => Ok(ShoppingList::from_file("most_recent_grocery_list.json").unwrap()),
            &_ => Ok(ShoppingList::new().unwrap()),
        }
    }
}

pub fn add_to_master_groceries() -> Result<()> {
    let add_groceries_prompt =
        "Do we want to add items to our big list?\n(y for yes, any other key for no)";
    println!("{}", add_groceries_prompt);
    match inputty().unwrap().trim() {
        "y" => {
            let groceries: Vec<GroceriesSection> = Groceries::get_sections().unwrap();
            let mut updated: Vec<GroceriesSection> = Vec::new();
            for section in groceries {
                println!(
		    "Add to our {} section?\n(y for yes, any other key for no, s to skip remaining sections)",
		    section.section
		);
                match inputty().unwrap().trim() {
                    "s" => break,
                    "y" => {
                        updated.push(GroceriesSection {
                            section: section.section,
                            items: redo_section(section.items).unwrap(),
                        });
                    }
                    &_ => {
                        updated.push(GroceriesSection {
                            section: section.section,
                            items: section.items,
                        });
                    }
                }
            }
            if !updated.len() == 0 {
                Groceries::update(updated).unwrap();
            }
        }
        &_ => {}
    }
    Ok(())
}

fn redo_section(mut items: Vec<String>) -> Result<Vec<String>> {
    println!("What shall we add? Enter the items, separated by commas");
    let mut input: String = inputty().unwrap();
    input.pop();
    let add_items_to_section: Vec<&str> = input.split(',').collect();
    add_items_to_section.iter().for_each(|i| {
        if !items.contains(&i.to_string()) {
            items.push(i.to_string());
        }
    });
    Ok(items)
}

pub fn add_to_recipes_lib() -> Result<()> {
    let add_recipes_prompt =
        "Do we want to add to our recipes library?\n(y for yes, any other key for no)";
    println!("{}", add_recipes_prompt);
    if let "y" = inputty().unwrap().trim() {
        let recipes = Recipes::from_file("recipes.json").unwrap();
        let mut updated: Vec<Recipe> = recipes.recipes;
        let new_recipe: Recipe = {
            println!("What's the name of the recipe we're adding?");
            let mut recipe = inputty().unwrap();
            recipe.pop();
            println!("Enter the ingredients, separated by commas");
            let mut ingredients = inputty().unwrap();
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
        Recipes::write_to_file(recipes, "recipes.json").unwrap();
        let mut done_with_adding_recipes = false;
        while !done_with_adding_recipes {
            println!("Add another recipe?\n(y for yes, any other key to continue)");
            match inputty().unwrap().trim() {
                "y" => {
                    add_to_recipes_lib().unwrap();
                }
                &_ => {
                    done_with_adding_recipes = true;
                }
            }
        }
    }
    Ok(())
}

pub fn add_recipes(mut shopping: ShoppingList) -> Result<ShoppingList> {
    println!("Add recipe ingredients to our list today?\n(y for yes, any other key for no)");
    match inputty().unwrap().trim() {
        "y" => {
            shopping = get_recipes(shopping).unwrap();
            println!("Add more recipes?\n(y for yes, any other key for no)");
            match inputty().unwrap().trim() {
                "y" => {
                    let shopping: ShoppingList = add_recipes(shopping).unwrap();
                    Ok(shopping)
                }
                &_ => Ok(shopping),
            }
        }
        &_ => Ok(shopping),
    }
}

fn get_recipes(mut shopping: ShoppingList) -> Result<ShoppingList> {
    let recipes = Recipes::from_file("recipes.json").unwrap();
    println!
	("Which recipes shall we add?\n(y to add recipe, s to skip to end of recipes, any other key to skip to next recipe)"
	);
    for recipe in recipes.recipes {
        println!("{}?", recipe.recipe);
        match inputty().unwrap().trim() {
            "y" => {
                shopping.recipes.push(recipe.recipe.to_owned());
                println!(
		    "Do we need ... ?\n(y to add ingredient, c to remind to check, a to add this and all remaining ingredients, any other key to skip to next ingredient)"
		);
                for ingredient in &recipe.ingredients {
                    println!("{}?", ingredient.to_lowercase());
                    match inputty().unwrap().trim() {
                        "y" => {
                            if shopping
                                .list
                                .contains(&ingredient.to_owned().to_lowercase())
                            {
                            } else {
                                shopping.list.push(ingredient.to_owned().to_lowercase());
                            }
                        }
                        "c" => {
                            shopping
                                .checklist
                                .push(ingredient.to_owned().to_lowercase());
                        }
                        "a" => {
                            for ingredient in recipe.ingredients {
                                if !shopping
                                    .list
                                    .contains(&ingredient.to_owned().to_lowercase())
                                {
                                    shopping.list.push(ingredient);
                                } else {
                                }
                            }
                            break;
                        }
                        &_ => {}
                    }
                }
            }
            "s" => {
                break;
            }
            &_ => {}
        }
    }
    Ok(shopping)
}

pub fn add_groceries(mut shopping: ShoppingList) -> Result<()> {
    println!("Okay, add groceries to a shopping list?\n(y for yes, any other key to skip)");
    match inputty().unwrap().trim() {
        "y" => {
            let groceries: Groceries = Groceries::read_file("groceries_dict.json").unwrap();
            for section in &groceries.sections {
                println!(
		    "Do we need {}?\n(y for yes, s to skip remaining sections, any other key to continue)\n",
		    section.section.to_lowercase()
		);
                match inputty().unwrap().trim() {
                    "y" => {
                        println!("Do we need ...?\n(y for yes, c for check, s to skip to next section, any other key to continue)");
                        for item in &section.items {
                            if shopping.list.contains(&item.to_lowercase()) {
                            } else {
                                println!("{}?", item.to_lowercase());
                                match inputty().unwrap().trim() {
                                    "y" => shopping.list.push(item.to_lowercase().to_string()),
                                    "c" => shopping.checklist.push(item.to_lowercase().to_string()),
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
        }
        &_ => {}
    }
    output(shopping).unwrap();
    Ok(())
}

pub fn add_anything_else() -> Result<()> {
    println!("\nForgotten anything?\n(y for yes, any other key to continue)");
    match inputty().unwrap().trim() {
        "y" => {
            println!("Oh we have?\n...");
            add_groceries(ShoppingList::from_file("most_recent_grocery_list.json").unwrap())
                .unwrap();
        }
        &_ => {
            output(ShoppingList::from_file("most_recent_grocery_list.json").unwrap()).unwrap();
        }
    }
    let mut added_groceries_or_recipes = false;
    let mut done_adding_to_master_groceries_recipes = false;
    while !done_adding_to_master_groceries_recipes {
	println!("\nAdd anything else to our master recipe and groceries lists now?\n('r' to add recipes, 'g' to add groceries, any other key to continue)");
	match inputty().unwrap().trim() {
            "r" => {
		added_groceries_or_recipes = true;
		add_to_recipes_lib().unwrap();
            }
            "g" => {
		added_groceries_or_recipes = true;
		add_to_master_groceries().unwrap();
            }
            &_ => done_adding_to_master_groceries_recipes = true,
	}
    }
    if added_groceries_or_recipes {
        // OPTION TO GO AGAIN WITH ADDING TO SHOPPINGLIST
	let mut done_with_shopping_list = false;
	while !done_with_shopping_list {
            println!("Add anything else to our shopping list?\n(r to add recipes, g to add groceries, any other key to continue)");
            match inputty().unwrap().trim() {
		"y" => {
                    add_groceries(add_recipes(ShoppingList::get().unwrap()).unwrap()).unwrap();
		}
		&_ => done_with_shopping_list = true,
            }
	}
    }
    Ok(())
}

fn inputty() -> Result<String> {
    let _ = Write::flush(&mut stdout());
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("problem with user input");
    Ok(input)
}

fn output(shopping: ShoppingList) -> Result<()> {
    if !shopping.checklist.is_empty() && !shopping.recipes.is_empty() && !shopping.list.is_empty() {
        println!("Here's what we have again:\n");
    }
    if !shopping.checklist.is_empty() {
        println!("{}", shopping.checklist_msg);
        shopping.checklist.iter().for_each(|item| {
            println!("\t{}", item.to_lowercase());
        });
    } else {
    }
    if !shopping.recipes.is_empty() {
        println!("{}", shopping.recipes_msg);

        shopping.recipes.iter().for_each(|recipe| {
            println!("\t{}", recipe);
        });
    }
    if !shopping.list.is_empty() {
        println!("{}", shopping.list_msg);
        shopping.list.iter().for_each(|item| {
            println!("\t{}", item);
        });
    }
    ShoppingList::write_to_file(shopping, "most_recent_grocery_list.json").unwrap();
    Ok(())
}
