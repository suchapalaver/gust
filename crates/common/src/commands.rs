use url::Url;

use crate::{
    item::Name,
    recipes::{Ingredients, Recipe},
    section::Section,
};

#[derive(Debug)]
pub enum ApiCommand {
    Add(Add),
    Delete(Delete),
    FetchRecipe(Url),
    ImportFromJson,
    Read(Read),
    Update(Update),
}

#[derive(Debug)]
pub enum Add {
    ChecklistItem(Name),
    Item {
        name: Name,
        section: Option<Section>,
    },
    ListItem(Name),
    ListRecipe(Recipe),
    Recipe {
        recipe: Recipe,
        ingredients: Ingredients,
    },
}

impl Add {
    pub fn checklist_item_from_name(name: Name) -> Self {
        Self::ChecklistItem(name)
    }

    pub fn item_from_name_and_section(name: Name, section: Option<Section>) -> Self {
        Self::Item { name, section }
    }

    pub fn list_item_from_name(name: Name) -> Self {
        Self::ListItem(name)
    }

    pub fn list_recipe_from_name(name: Recipe) -> Self {
        Self::ListRecipe(name)
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
    ChecklistItem(Name),
    ClearChecklist,
    ClearList,
    Item(Name),
    ListItem(Name),
    Recipe(Recipe),
}

impl Delete {
    pub fn item_from_name(name: Name) -> Self {
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
    Item(Name),
    List,
    ListRecipes,
    Recipe(Recipe),
    Recipes,
    Sections,
}

impl Read {
    pub fn item_from_name(name: Name) -> Self {
        Self::Item(name)
    }

    pub fn recipe_from_name(name: Recipe) -> Self {
        Self::Recipe(name)
    }
}

#[derive(Debug)]
pub enum Update {
    Item(Name),
    RefreshList,
    Recipe(Recipe),
}

impl Update {
    pub fn refresh_list() -> Self {
        Self::RefreshList
    }

    pub fn recipe_from_name(name: Recipe) -> Self {
        Self::Recipe(name)
    }
}
