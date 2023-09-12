use crate::{
    item::{ItemName, Section},
    recipes::{Ingredients, RecipeName},
};

pub enum ApiCommand {
    Add(Add),
    Delete(Delete),
    Read(Read),
    Update(Update),
}

pub enum Add {
    ChecklistItem(ItemName),
    Item {
        name: ItemName,
        section: Option<Section>,
    },
    ListItem(ItemName),
    NewList,
    ListRecipe(RecipeName),
    Recipe {
        recipe: RecipeName,
        ingredients: Ingredients,
    },
}

pub enum Delete {
    ChecklistItem(ItemName),
    ClearChecklist,
    ClearList,
    Item(ItemName),
    ListItem(ItemName),
    Recipe(RecipeName),
}

pub enum Read {
    All,
    Checklist,
    Item(ItemName),
    Items,
    List,
    ListRecipes,
    Recipe(RecipeName),
    Recipes,
    Sections,
}

pub enum Update {
    Item(ItemName),
    Recipe(RecipeName),
}
