use url::Url;

use crate::{
    item::{ItemName, Section},
    recipes::{Ingredients, Recipe},
};

#[derive(Debug)]
pub enum ApiCommand {
    Add(Add),
    Delete(Delete),
    FetchRecipe(Url),
    MigrateJsonDbToSqlite,
    Read(Read),
    Update(Update),
}

#[derive(Debug)]
pub enum Add {
    ChecklistItem(ItemName),
    Item {
        name: ItemName,
        section: Option<Section>,
    },
    ListItem(ItemName),
    ListRecipe(Recipe),
    NewList,
    Recipe {
        recipe: Recipe,
        ingredients: Ingredients,
    },
}

impl Add {
    pub fn checklist_item_from_name(name: ItemName) -> Self {
        Self::ChecklistItem(name)
    }

    pub fn item_from_name_and_section(name: ItemName, section: Option<Section>) -> Self {
        Self::Item { name, section }
    }

    pub fn list_item_from_name(name: ItemName) -> Self {
        Self::ListItem(name)
    }

    pub fn list_recipe_from_name(name: Recipe) -> Self {
        Self::ListRecipe(name)
    }

    pub fn new_list() -> Self {
        Self::NewList
    }

    pub fn recipe_from_name_and_ingredients(recipe: Recipe, ingredients: Ingredients) -> Self {
        Self::Recipe {
            recipe,
            ingredients,
        }
    }
}

#[derive(Debug)]
pub enum Delete {
    ChecklistItem(ItemName),
    ClearChecklist,
    ClearList,
    Item(ItemName),
    ListItem(ItemName),
    Recipe(Recipe),
}

impl Delete {
    pub fn item_from_name(name: ItemName) -> Self {
        Self::Item(name)
    }

    pub fn recipe_from_name(name: Recipe) -> Self {
        Self::Recipe(name)
    }
}

#[derive(Debug)]
pub enum Read {
    All,
    Checklist,
    Item(ItemName),
    List,
    ListRecipes,
    Recipe(Recipe),
    Recipes,
    Sections,
}

impl Read {
    pub fn item_from_name(name: ItemName) -> Self {
        Self::Item(name)
    }

    pub fn recipe_from_name(name: Recipe) -> Self {
        Self::Recipe(name)
    }
}

#[derive(Debug)]
pub enum Update {
    Item(ItemName),
    Recipe(Recipe),
}

impl Update {
    pub fn recipe_from_name(name: Recipe) -> Self {
        Self::Recipe(name)
    }
}
