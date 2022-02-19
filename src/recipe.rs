use serde::{Deserialize, Serialize};
use std::{error::Error, fmt, ops::Deref, path::Path};

use crate::errors::ReadError;
use crate::helpers::read;

/// let r1 = Recipe(String::from("eggs"));
/// let r2 = Recipe(String::from("sandwiches"));
/// let recipes = vec![r1, r2];
///
/// Using impl Deref as suggested [here](https://stackoverflow.com/a/68278323/15238776)
///    as the easiest approach for producing an iterator from structs such as my Recipes and
///    Groceries structs
///
///  println!("recipes:");
///  for recipe in recipes.iter() {
///     println!("{}", recipe);
///  }
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Recipe(pub String);

impl fmt::Display for Recipe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Recipes {
    pub collection: Vec<Recipe>,
}

// CHANGE THIS TO A TRAIT IMPLEMENTATION THAT CAN BE REPEATED
impl Recipes {
    pub fn new() -> Self {
        Recipes::new_initialized(Default::default())
    }

    pub fn new_initialized(initial_content: Recipe) -> Recipes {
        let collection: Vec<Recipe> = vec![initial_content];

        Recipes { collection }
    }

    // Opens user's recipes library from the path provided
    pub fn from_path<P: AsRef<Path> + Copy>(path: P) -> Result<Recipes, Box<dyn Error>> {
        let reader = read(path)?;

        let recipes = serde_json::from_reader(reader).map_err(ReadError::DeserializingError)?;

        Ok(recipes)
    }
    /*
    pub fn as_slice(&self) -> &[Recipe] {
    &self.collection
    }
    */
}

impl Default for Recipes {
    fn default() -> Self {
        Self::new()
    }
}

impl Deref for Recipes {
    type Target = Vec<Recipe>;

    fn deref(&self) -> &Self::Target {
        &self.collection
    }
}

/*
fn new_recipes() -> Result<(), Box<dyn Error>> {
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
             '{}'",
path, e
)
})?;

let RecipeLib(mut updated) = recipes.library;

let new_recipe = get_new_recipe()?;

updated.push(new_recipe);

let recipes = Recipes {
library: RecipeLib(updated),
};

save_recipes(recipes)?;

eprintln!(
"Add more recipes to our library?\n\
         --y\n\
         --any other key to exit"
);
}
Ok(())
}

// Gets a new recipe from user
// and returns it as a Recipe
fn get_new_recipe() -> Result<Recipe, Box<dyn Error>> {
eprintln!("What's the recipe?");

let name = input()?;

let mut items = Vec::new();

items = list_input(items)?;

Ok(Recipe(String::from(name)))
}

fn save_recipes(recipes: Recipes) -> Result<(), Box<dyn Error>> {
let json = serde_json::to_string(&recipes)?;

write("recipes.json", json)?;

Ok(())
}

fn print_recipes() -> Result<(), Box<dyn Error>> {
let path = "recipes.json";

let recipes = read_recipes(path).map_err(|e| {
format!(
"Failed to read recipes file '{}':\n\
         '{}'",
path, e
)
})?;

eprintln!("Here are our recipes:");

for recipe in recipes {
eprintln!("- {}", recipe);
}
eprintln!();

Ok(())
}
*/
