use std::{ops::Deref, io::BufReader, fs::File, path::Path, error::Error};

use serde::{Serialize, Deserialize};

use crate::{ReadError::DeserializingError, read, GroceriesItem};

///  let apples = GroceriesItem {
///     name: GroceriesItemName("apples".to_string()),
///     section: GroceriesItemSection("fresh".to_string()),
///     is_recipe_ingredient: true,
///     recipes: vec![Recipe("cheese and apple snack".to_string())],
///  };
///
///  let cheddar = GroceriesItem {
///     name: GroceriesItemName("cheddar cheese".to_string()),
///     section: GroceriesItemSection("dairy".to_string()),
///     is_recipe_ingredient: true,
///     recipes: vec![Recipe("cheese and apple snack".to_string()), Recipe("tomato pasta".to_string())],
///     };
///
/// let olive_oil = GroceriesItem {
///     name: GroceriesItemName("olive oil".to_string()),
///     section: GroceriesItemSection("pantry".to_string()),
///     is_recipe_ingredient: true,
///     recipes: vec![Recipe("TOO MANY TO NAME!".to_string())],
/// };
///
/// let groceries = Groceries {
///     collection: vec![apples, cheddar, olive_oil],
/// };
///
/// deref is implicitly called so groceries.iter()
/// represents groceries.collection.iter()
///
/// println!("groceries:");
///
/// for item in groceries.iter() {
///     println!("{}", item);
/// }
///
/// for item in groceries.iter() {
///    for r in item.iter() {
///         println!("{}", r);
///     }
/// }
///
/// let apple_cheese_ingredients: Vec<_> = groceries
///     .iter()
///     .filter(|x| {
///         x.recipes
///             .contains(&Recipe("cheese and apple snack".to_string()))
///     })
///     .collect();
///
/// println!();
/// println!("experiment:");
/// for ingredient in apple_cheese_ingredients {
///     let recipes = ingredient.iter().map(|x| x.0.clone()).collect::<Vec<_>>();
///     let recipes_str = recipes.join(", ");
///     println!("ingredient: {}\trecipes: {}", ingredient, recipes_str);
/// }
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Groceries {
    pub collection: Vec<GroceriesItem>,
}

impl Deref for Groceries {
    type Target = Vec<GroceriesItem>;

    fn deref(&self) -> &Self::Target {
        &self.collection
    }
}

impl Groceries {
    pub fn new() -> Self {
        Self::new_initialized()
    }

    pub fn new_initialized() -> Self {
        let collection: Vec<GroceriesItem> = Vec::new();

        Self { collection } // using Self, can I make this a generic trait?
    }

    pub fn from_path<P: AsRef<Path> + Copy>(path: P) -> Result<Self, Box<dyn Error>> {
        let reader: BufReader<File> = read(path)?;

        let data: Self = serde_json::from_reader(reader).map_err(DeserializingError)?;

        Ok(data)
    }
}
