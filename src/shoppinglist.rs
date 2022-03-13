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

// Like run() for the shopping-list-making function in grusterylist
pub fn make_list() -> Result<(), Box<dyn Error>> {
    // Open a saved or new list
    let mut shopping_list = get_saved_or_new_list()?;

    // view list if using saved list
    if !shopping_list.groceries.is_empty() {
        print_list()?;
    }

    // add recipes to shoppinglist.recipes
    shopping_list = add_recipes_to_list(shopping_list)?;

    // move everything off list to temp list
    let list_items: Vec<GroceriesItem> = shopping_list.groceries.drain(..).collect();
    assert!(shopping_list.groceries.is_empty());

    // add individual groceries
    shopping_list.groceries = add_groceries_to_list(list_items, &shopping_list.recipes)?;

    // overwrite saved list with current list
    save_list(shopping_list)?;

    // view list
    print_list()?;

    Ok(())
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

// Open and deserialize a shopping list JSON file from given path
pub fn read_list<P: AsRef<Path> + Copy>(path: P) -> Result<ShoppingList, Box<dyn Error>> {
    let reader = crate::helpers::read(path)?;

    let shopping_list = serde_json::from_reader(reader).map_err(ReadError::DeserializingError)?;

    Ok(shopping_list)
}

// Prints list
pub fn print_list() -> Result<(), Box<dyn Error>> {
    eprintln!(
        "\n\
	 Print shopping list?\n\
	 *y*\n\
	 *any other key* to continue"
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
        if !shopping_list.checklist.is_empty() {
            println!("Check if we need:");

            shopping_list.checklist.iter().for_each(|item| {
                println!("\t{}", item.name.0.to_lowercase());
            });
        }
        if !shopping_list.recipes.is_empty() {
            println!("recipes:");

            shopping_list.recipes.iter().for_each(|recipe| {
                println!("\t{}", recipe);
            });
        }
        if !shopping_list.groceries.is_empty() {
            println!("groceries:");

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
                "Failed to read recipes from '{}':\n\
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
                "y" => {
                    if !shopping_list.recipes.contains(&recipe) {
                        shopping_list.recipes.push(recipe)
                    }
                }
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

// go through all items in groceries library
// check for items that are recipe ingredients and not already on list
// check if item is ingredient of any recipes on list
// add relevant items to list
fn add_groceries_to_list(
    mut items_list: Vec<GroceriesItem>,
    //mut shopping_list: ShoppingList,
    recipes: &[Recipe],
) -> Result<Vec<GroceriesItem>, Box<dyn Error>> {
    let path = "groceries.json";

    let groceries = Groceries::from_path(path)?;
    /*
    for groceriesitem in groceries.collection.iter() {

    if groceriesitem.is_recipe_ingredient && !items_list.contains(groceriesitem) {
    //if groceriesitem.is_recipe_ingredient && !shopping_list.groceries.contains(groceriesitem) {

        for recipe in &groceriesitem.recipes {

        if recipes.contains(recipe) && !items_list.contains(groceriesitem) {

            items_list.push(groceriesitem.clone());
                }
            }
        }
    }
     */

    eprintln!(
        "Add groceries to shopping list?\n\
	 *y*\n\
	 *any other key* to skip"
    );

    while prompt_for_y()? {
        items_list = add_grocery_sections_to_list(items_list, recipes, &groceries)?;

        eprintln!(
            "Add more groceries to shopping list?\n\
	     *y*\n\
	     *any other key* to skip"
        );
    }

    Ok(items_list)
}

// Saves shopping list
fn save_list(shopping_list: ShoppingList) -> Result<(), Box<dyn Error>> {
    eprintln!(
        "Save current list?\n\
	 *y*\n\
	 *any other key* to continue"
    );

    if crate::prompt_for_y()? {
        let json = serde_json::to_string(&shopping_list)?;
        // Put trace here
        crate::helpers::write("list.json", json)?;
    }
    Ok(())
}

//
fn add_grocery_sections_to_list(
    shopping_list: Vec<GroceriesItem>, // items_list not in any order with all ingredients from recipes
    recipes: &[Recipe],
    groceries: &Groceries,
) -> Result<Vec<GroceriesItem>, Box<dyn Error>> {
    let sections = vec!["fresh", "pantry", "dairy", "protein", "freezer"];

    let groceries_by_section: Vec<Vec<GroceriesItem>> = {
        sections
            .into_iter()
            .map(|section| get_section_items_not_on_list(groceries, &shopping_list, section))
            .collect()
    };
    //eprintln!("groceries_by_section: {:?}", groceries_by_section);
    let shopping_list = add_sections(groceries_by_section, recipes, shopping_list)?;

    Ok(shopping_list)
}

// returns vector of groceries items belonging to a given section that are not already on list
fn get_section_items_not_on_list(
    groceries: &Groceries,
    shopping_list: &[GroceriesItem],
    section: &str,
) -> Vec<GroceriesItem> {
    let mut a: Vec<GroceriesItem> = shopping_list
        .iter()
        .filter(|groceriesitem| groceriesitem.section.0 == section)
        .cloned()
        .collect();
    //eprintln!("get_section_items_not_on_list variable a: {:?}", a);

    let b: Vec<GroceriesItem> = groceries
        .collection
        .iter()
        .filter(|groceriesitem| groceriesitem.section.0 == section && !a.contains(groceriesitem))
        .cloned()
        .collect();

    //eprintln!("get_section_items_not_on_list variable b: {:?}", a);

    a.extend(b);

    //eprintln!("get_section_items_not_on_list variable a extend(b): {:?}", a);
    a
}

// calls fn add_section_items on non-empty sections
fn add_sections(
    groceries_by_section: Vec<Vec<GroceriesItem>>,
    recipes: &[Recipe],
    mut shopping_list: Vec<GroceriesItem>,
) -> Result<Vec<GroceriesItem>, Box<dyn Error>> {
    for section in groceries_by_section {
        if !section.is_empty() {
            shopping_list = add_section_items(&section, recipes, shopping_list)?;
        }
    }
    Ok(shopping_list)
}

// takes groceries section as a slice of groceries items
// iterates through items
// clones and adds to list items based on user input
fn add_section_items(
    groceries_section: &[GroceriesItem],
    recipes: &[Recipe],
    mut shopping_list: Vec<GroceriesItem>,
) -> Result<Vec<GroceriesItem>, Box<dyn Error>> {
    for groceriesitem in groceries_section {
        //eprintln!("groceriesitem: {}", groceriesitem);

        if !shopping_list.contains(groceriesitem)
            && groceriesitem
                .recipes
                .iter()
                .any(|recipe| recipes.contains(&*recipe))
        {
            //eprintln!("automatically adding recipe ingredient {}", groceriesitem.name);

            shopping_list.push(groceriesitem.clone());
        }
    }

    for groceriesitem in groceries_section {
        if !shopping_list.contains(groceriesitem) {
            eprintln!(
                "Do we need {}?\n\
		 *y*\n\
		 *any other key* for next item\n\
		 *s* for next section",
                groceriesitem.name.0.to_lowercase()
            );

            match input()?.as_str() {
                "y" => {
                    if !shopping_list.contains(groceriesitem) {
                        shopping_list.push(groceriesitem.clone());
                    }
                }
                "s" => break,
                &_ => continue,
            }
        }
    }

    Ok(shopping_list)
}
////////////////////////////////////////////////////////////////////
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
