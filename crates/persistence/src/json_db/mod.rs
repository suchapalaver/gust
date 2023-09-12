use std::{collections::HashSet, path::Path};

use common::{
    helpers::ReadWrite,
    input::{item_from_user, item_matches, section_from_user},
    item::{Item, Section},
    items::Groceries,
    list::ShoppingList,
    recipes::RecipeName,
    ReadError,
};

pub const ITEMS_JSON_PATH: &str = "groceries.json";

pub const LIST_JSON_PATH: &str = "list.json";

pub struct JsonStore {
    items_path: String,
    list_path: String,
}

impl Default for JsonStore {
    fn default() -> Self {
        Self {
            items_path: ITEMS_JSON_PATH.to_string(),
            list_path: LIST_JSON_PATH.to_string(),
        }
    }
}

pub fn load_groceries_library<P: AsRef<Path> + std::marker::Copy>(
    path: P,
) -> Result<Groceries, ReadError> {
    Groceries::from_path(path)
}

pub fn load_list() -> Result<ShoppingList, ReadError> {
    ShoppingList::from_path(LIST_JSON_PATH)
}

pub fn load_groceries_collection<P: AsRef<Path> + std::marker::Copy>(
    path: P,
) -> Result<Vec<Item>, ReadError> {
    Ok(load_groceries_library(path)?.collection)
}

pub fn load_recipes<P: AsRef<Path> + std::marker::Copy>(
    path: P,
) -> Result<Vec<RecipeName>, ReadError> {
    let mut recipes: HashSet<RecipeName> = HashSet::new();

    {
        let groceries = load_groceries_library(path)?;

        for item in groceries.collection {
            if let Some(item_recipes) = item.recipes {
                for recipe in item_recipes {
                    recipes.insert(recipe);
                }
            }
        }

        for recipe in groceries.recipes {
            recipes.insert(recipe);
        }
    }

    {
        let list = load_list()?;

        for recipe in list.recipes {
            recipes.insert(recipe);
        }
    }

    Ok(recipes.into_iter().collect())
}

pub fn load_sections<P: AsRef<Path> + std::marker::Copy>(
    path: P,
) -> Result<Vec<Section>, ReadError> {
    Ok(load_groceries_library(path)?.sections)
}

pub fn view_groceries() -> Result<(), ReadError> {
    for item in Groceries::from_path(ITEMS_JSON_PATH)?.items() {
        eprintln!();
        eprintln!("{}", item);
        eprintln!();
    }
    Ok(())
}

pub fn add_grocery_item() -> Result<(), ReadError> {
    let item = item_from_user();

    let section = section_from_user();

    let mut groceries = Groceries::from_path(ITEMS_JSON_PATH).unwrap_or_default();

    let mut present = false;

    for item in groceries.get_item_matches(&item) {
        if item_matches(item) {
            present = true;
            break;
        }
    }

    if present {
        eprintln!("Item already in library");
    } else {
        let new_item = Item::new(&item).with_section(&section);
        groceries.add_item(new_item);
        todo!();
    }
    Ok(())
}
