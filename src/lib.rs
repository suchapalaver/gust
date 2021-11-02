use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fs::{self, write, File},
    io::{stdin, stdout, BufReader, Write},
    path::Path,
};

#[derive(Serialize, Deserialize, Debug)]
struct Groceries {
    sections: Vec<GroceriesSection>,
}

impl Groceries {
    fn write_to_file<P: AsRef<Path>>(groceries: Groceries, path: P) -> Result<(), &'static str> {
        let json: String = serde_json::to_string(&groceries).unwrap();
        write(path, &json).expect("Unable to write file");
        Ok(())
    }

    fn read_file<P: AsRef<Path>>(path: P) -> Result<Groceries, &'static str> {
        // Open the file in read-only mode with buffer.
        let reader = BufReader::new(File::open(path).expect("file issue"));
        let groceries: Groceries = serde_json::from_reader(reader).expect("reader issue");
        Ok(groceries)
    }

    fn update(updated_items: Vec<GroceriesSection>) -> Result<(), &'static str> {
        let groceries = Groceries {
            sections: updated_items,
        };
        Groceries::write_to_file(groceries, "groceries_dict.json").unwrap();
        Ok(())
    }

    fn get_sections() -> Result<Vec<GroceriesSection>, &'static str> {
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
    fn write_to_file<P: AsRef<Path>>(recipes: Recipes, path: P) -> Result<(), &'static str> {
        let json: String = serde_json::to_string(&recipes).unwrap();
        fs::write(path, &json).expect("Unable to write file");
        Ok(())
    }

    fn from_file<P: AsRef<Path>>(path: P) -> Result<Recipes, &'static str> {
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
struct ShoppingList {
    recipes_msg: String,
    recipes: Vec<String>,
    checklist_msg: String,
    checklist: Vec<String>,
    list_msg: String,
    list: Vec<String>,
}

impl ShoppingList {
    fn new() -> Result<ShoppingList, &'static str> {
        Ok(ShoppingList {
            recipes_msg: "We're making ...".to_string(),
            recipes: Vec::new(),
            checklist_msg: "Check ...".to_string(),
            checklist: Vec::new(),
            list_msg: "We need ...".to_string(),
            list: Vec::new(),
        })
    }

    fn from_file<P: AsRef<Path>>(path: P) -> Result<ShoppingList, &'static str> {
        // Open the file in read-only mode with buffer.
        let reader = BufReader::new(File::open(path).expect("file issue"));
        let shopping: ShoppingList = serde_json::from_reader(reader).expect("reader issue");
        Ok(shopping)
    }

    fn write_to_file<P: AsRef<Path>>(shopping: &ShoppingList, path: P) -> Result<(), &'static str> {
        let json: String = serde_json::to_string(&shopping).unwrap();
        fs::write(path, &json).expect("Unable to write file");
        Ok(())
    }

    fn get() -> Result<ShoppingList, &'static str> {
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

pub fn run() -> Result<(), Box<dyn Error>> {
    // ADD GROCERIES TO MASTER LIST
    let mut no_need_to_add_to_master_groceries = false;

    while !no_need_to_add_to_master_groceries {
        let add_groceries_prompt =
            "Do we want to add any more items to our big list?\n(y for yes, any other key for no)";
        println!("{}", add_groceries_prompt);

        match input().trim() {
            "y" => {
                let groceries: Vec<GroceriesSection> = Groceries::get_sections().unwrap();
                let mut updated: Vec<GroceriesSection> = Vec::new();
                for section in groceries {
                    println!(
			"Add to our {} section?\n(y for yes, any other key for no, s to skip remaining sections)",
			section.section
		    );
                    match input().trim() {
                        "s" => break,
                        "y" => {
                            updated.push(GroceriesSection {
                                section: section.section,
                                items: redo_groceries_section(section.items),
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
            &_ => no_need_to_add_to_master_groceries = true,
        }
    }

    // ADD RECIPES TO RECIPES LIBRARY
    let mut no_need_to_add_to_recipes = false;

    while !no_need_to_add_to_recipes {
        let add_recipes_prompt =
            "Do we want to add more recipes to our recipes library?\n(y for yes, any other key for no)";
        println!("{}", add_recipes_prompt);

        if let "y" = input().trim() {
            let recipes = Recipes::from_file("recipes.json").unwrap();

            let mut updated: Vec<Recipe> = recipes.recipes;

            let new_recipe: Recipe = {
                println!("What's the name of the recipe we're adding?");

                let mut recipe = input();

                recipe.pop();

                println!("Enter the ingredients, separated by commas");

                let mut ingredients = input();

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
        } else {
            no_need_to_add_to_recipes = true;
        }
    }

    // GET SHOPPING LIST
    let mut shopping_list = ShoppingList::get().unwrap();

    // ADD RECIPE INGREDIENTS TO LIST
    let mut done_adding_recipe_ingredients_to_shopping_list = false;

    while !done_adding_recipe_ingredients_to_shopping_list {
        println!("Add recipe ingredients to our list?\n(y for yes, any other key for no)");

        match input().trim() {
            "y" => {
                shopping_list = add_recipes(shopping_list);
            }
            &_ => done_adding_recipe_ingredients_to_shopping_list = true,
        }
    }

    // ADD TO SHOPPING LIST AND CHECKLIST
    let mut done_adding_groceries_to_list = false;

    while !done_adding_groceries_to_list {
        println!("Add groceries to shopping list?\n(y for yes, any other key to skip)");

        match input().trim() {
            "y" => {
                let groceries: Groceries = Groceries::read_file("groceries_dict.json").unwrap();

                for section in &groceries.sections {
                    println!(
			"Do we need {}?\n(y for yes, s to skip remaining sections, any other key to continue)\n",
			section.section.to_lowercase()
		    );
                    match input().trim() {
                        "y" => {
                            println!("Do we need ...?\n(y for yes, c for check, s to skip to next section, any other key to continue)");

                            for item in &section.items {
                                if shopping_list.list.contains(&item.to_lowercase()) {
                                } else {
                                    println!("{}?", item.to_lowercase());

                                    match input().trim() {
                                        "y" => {
                                            shopping_list.list.push(item.to_lowercase().to_string())
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
            }
            &_ => done_adding_groceries_to_list = true,
        }
    }
    // SAVE MOST RECENT LIST
    ShoppingList::write_to_file(&shopping_list, "most_recent_grocery_list.json").unwrap();

    // OUTPUT
    output(shopping_list);

    // ADD ANYTHING ELSE TO MASTER GROCERY LISTS, RECIPE LISTS,OR SHOPPINGLIST
    println!("\nForgotten anything?\n(y for yes, any other key to continue)");
    match input().trim() {
        "y" => {
            println!("Oh we have?\n...");
            run().unwrap();
        }
        &_ => {}
    }

    println!("Bye! Happy shopping! Bon appetit!");
    Ok(())
}

fn add_recipes(mut shopping: ShoppingList) -> ShoppingList {
    let recipes = Recipes::from_file("recipes.json").unwrap();
    println!
	("Which recipes shall we add?\n(y to add recipe, s to skip to end of recipes, any other key for next recipe)"
	);
    for recipe in recipes.recipes {
        println!("{}?", recipe.recipe);
        match input().trim() {
            "y" => {
                shopping.recipes.push(recipe.recipe.to_owned());
                println!(
		    "Do we need ... ?\n(y to add ingredient, c to remind to check, a to add this and all remaining ingredients, any other key for next ingredient)"
		);
                for ingredient in &recipe.ingredients {
                    println!("{}?", ingredient.to_lowercase());
                    match input().trim() {
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
    shopping
}

fn input() -> String {
    let _ = Write::flush(&mut stdout());
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("problem with user input");
    input
}

fn output(shopping_list: ShoppingList) {
    if !shopping_list.checklist.is_empty()
        && !shopping_list.recipes.is_empty()
        && !shopping_list.list.is_empty()
    {
        println!("Here's what we have:\n");
    }
    if !shopping_list.checklist.is_empty() {
        println!("{}", shopping_list.checklist_msg);
        shopping_list.checklist.iter().for_each(|item| {
            println!("\t{}", item.to_lowercase());
        });
    } else {
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

fn redo_groceries_section(mut items: Vec<String>) -> Vec<String> {
    println!("What shall we add? Enter the items, separated by commas");
    let mut input: String = input();
    input.pop();
    let add_items_to_section: Vec<&str> = input.split(',').collect();
    add_items_to_section.iter().for_each(|i| {
        if !items.contains(&i.to_string()) {
            items.push(i.to_string());
        }
    });
    items
}
