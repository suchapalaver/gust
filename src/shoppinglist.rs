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
/*
// You need a checklist on every shopping list
// to check if we need certain items we're not sure about
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Checklist(pub Vec<GroceriesItem>);

impl Checklist {
fn new() -> Self {
        Checklist::new_initialized()
    }

    pub fn new_initialized() -> Checklist {
        Checklist(Vec::new())
    }

    pub fn as_slice(&self) -> &[GroceriesItem] {
&self.0
    }
}
 */

use std::{error::Error, path::Path};

use serde::{Deserialize, Serialize};

use crate::{input, prompt_for_y, Groceries, GroceriesItem, ReadError, Recipe};

// used to serialize and deserialize the
// most recently saved list or to create a
// new grocery list that can be saved as JSON
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShoppingList {
    pub checklist: Vec<GroceriesItem>,
    pub recipes: Vec<Recipe>,
    pub groceries: Vec<GroceriesItem>,
}

impl ShoppingList {
    pub fn new() -> Self {
        // this fn def is duplicate
        Self::new_initialized()
    }

    pub fn new_initialized() -> Self {
        let checklist: Vec<GroceriesItem> = Vec::new();
        let recipes: Vec<Recipe> = Vec::new();
        let groceries: Vec<GroceriesItem> = Vec::new();
        ShoppingList {
            // this fn def is unique to this struct
            checklist, // or default
            recipes,   // or default()
            groceries,
        }
    }
}

impl Default for ShoppingList {
    fn default() -> Self {
        Self::new()
    }
}

// Prompt user whether to use a saved or new list and return their choice
pub fn get_saved_or_new_list() -> Result<ShoppingList, Box<dyn Error>> {
    let mut shopping_list = ShoppingList::new();

    eprintln!(
        "\n\
	 Use most recently saved list?\n\
	 *y*\n\
	 *any other key* for fresh list"
    );

    if crate::prompt_for_y()? {
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

// Prints list
pub fn print_list() -> Result<(), Box<dyn Error>> {
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
        // check if there are checklist items
        if !shopping_list.checklist.is_empty()
	    // print them if so
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
pub fn add_recipes_to_list(
    mut shopping_list: ShoppingList,
) -> Result<ShoppingList, Box<dyn Error>> {
    eprintln!(
        "Add recipe ingredients to our list?\n\
	 *y*\n\
	 *any other key* to continue"
    );

    while prompt_for_y()? {
        let path = "groceries.json";

        let groceries = Groceries::from_path(path).map_err(|e| {
            format!(
                "Failed to read recipes file '{}':\n\
		 '{}'",
                path, e
            )
        })?;

        for recipe in groceries.recipes.into_iter() {
            eprintln!(
                "Shall we add ...\n\
		 {}?\n\
		 *y*\n\
		 *s* to skip to end of recipes\n\
		 *any other key* for next recipe",
                recipe
            );

            match input()?.as_str() {
                "y" => shopping_list.recipes.push(recipe),
                //"y" => shopping_list = add_recipe_to_list(shopping_list, recipe)?,
                "s" => break,
                &_ => continue,
            }
        }
        eprintln!(
            "Add any more recipe ingredients to our list?\n\
	     *y*\n\
	     *any other key* to continue"
        );
    }
    Ok(shopping_list)
}
/*
// Adds ingredients of an individual recipe to a shopping list
// AGAIN, TOTAL RESTRUCTURING NEEDED
pub fn add_recipe_to_list(
    mut shopping_list: ShoppingList,
    recipe: Recipe,
) -> Result<ShoppingList, Box<dyn Error>> {
    shopping_list.recipes.push(recipe);
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
*/
// Adds groceries to list
// NEEDS TO ADD *ALL* RECIPE INGREDIENTS FIRST
pub fn add_groceries_to_list(
    mut shopping_list: ShoppingList,
) -> Result<ShoppingList, Box<dyn Error>> {
    // BEST TO COMPILE THE LIST AT THE END
    let path = "groceries.json";

    let groceries = Groceries::from_path(path)?;

    for groceriesitem in groceries.collection.iter() {
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
        /*
            let groceries = Groceries::from_path(path).map_err(|e| {
                format!(
                    "Failed to read groceries file '{}':\n\
             '{}'",
                    path, e
                )
            })?;
        */
        shopping_list = add_grocery_sections_to_list(shopping_list, &groceries)?;

        eprintln!(
            "Add more groceries to shopping list?\n\
	     --y\n\
	     --any other key to skip"
        );
    }
    Ok(shopping_list)
}

// Open and deserialize a shopping list JSON file from given path
pub fn read_list<P: AsRef<Path> + Copy>(path: P) -> Result<ShoppingList, Box<dyn Error>> {
    let reader = crate::helpers::read(path)?;

    let shopping_list = serde_json::from_reader(reader).map_err(ReadError::DeserializingError)?;

    Ok(shopping_list)
}

// Saves shopping list
pub fn save_list(shopping_list: ShoppingList) -> Result<(), Box<dyn Error>> {
    eprintln!(
        "Save current list?\n\
	 --y\n\
	 --any other key to continue"
    );

    if crate::prompt_for_y()? {
        let json = serde_json::to_string(&shopping_list)?;
        // Put trace here
        crate::helpers::write("list.json", json)?;
    }
    Ok(())
}

pub fn add_grocery_sections_to_list(
    shopping_list: ShoppingList,
    groceries: &Groceries,
) -> Result<ShoppingList, Box<dyn Error>> {
    let sections = vec!["fresh", "pantry", "dairy", "protein", "freezer"];

    let groceries_by_section: Vec<Vec<GroceriesItem>> = {
        sections
            .into_iter()
            .map(|x| fum(groceries, &shopping_list, x))
            .collect()
    };

    let shopping_list = foo(groceries_by_section, shopping_list)?;

    Ok(shopping_list)
}

fn fum(groceries: &Groceries, shopping_list: &ShoppingList, section: &str) -> Vec<GroceriesItem> {
    groceries
        .collection
        .iter()
        .cloned() // ...
        .filter(|x| !shopping_list.groceries.contains(x) && x.section.0 == section)
        .collect()
}

fn foo(
    groceries_sections: Vec<Vec<GroceriesItem>>,
    mut shopping_list: ShoppingList,
) -> Result<ShoppingList, Box<dyn Error>> {
    for groceries_section in groceries_sections {
        if !groceries_section.is_empty() {
            shopping_list = far(&groceries_section, shopping_list)?;
        }
    }
    Ok(shopping_list)
}

fn far(
    groceries_section: &[GroceriesItem],
    mut shopping_list: ShoppingList,
) -> Result<ShoppingList, Box<dyn Error>> {
    for groceriesitem in groceries_section {
        eprintln!(
            "Do we need {}?\n\
	     *y*\n\
	     *n* for next section\n\
	     *any other key* to continue",
            groceriesitem.name.0.to_lowercase()
        );

        match input()?.as_str() {
            "y" => {
                //shopping_list = add_grocery_section_to_list(shopping_list, groceries_section)?
                shopping_list.groceries.push(groceriesitem.clone());
            }
            "n" => break,
            &_ => continue,
        }
    }
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
