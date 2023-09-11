use std::{collections::HashSet, path::Path};

use common::{
    errors::ReadError,
    groceries::Groceries,
    groceriesitem::{Item, Section},
    helpers::ReadWrite,
    recipes::RecipeName,
    shoppinglist::ShoppingList,
};

pub fn load_groceries_library<P: AsRef<Path> + std::marker::Copy>(
    path: P,
) -> Result<Groceries, ReadError> {
    Groceries::from_path(path)
}

pub fn load_list() -> Result<ShoppingList, ReadError> {
    ShoppingList::from_path("list.json")
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

pub fn save() -> Result<(), ReadError> {
    let path = "groceries.json";
    let groceries = Groceries::from_path(path).unwrap_or_default();
    groceries.save(path)?;
    Ok(())
}

pub fn view_groceries() -> Result<(), ReadError> {
    let path = "groceries.json";

    for item in Groceries::from_path(path)?.items() {
        eprintln!();
        eprintln!("{}", item);
        eprintln!();
    }
    Ok(())
}
