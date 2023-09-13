use std::fmt::{self, Display};

use common::{
    commands::{Add, ApiCommand, Delete, Read, Update},
    recipes::{Ingredients, RecipeName},
};
use persistence::store::{Storage, Store, StoreError};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Store error: {0}")]
    StoreError(#[from] StoreError),
}

pub struct Api {
    store: Store,
}

impl Api {
    pub fn new(store: Store) -> Self {
        Self { store }
    }

    pub fn execute(&mut self, command: ApiCommand) -> Result<ApiResponse, ApiError> {
        match command {
            ApiCommand::Add(cmd) => match cmd {
                Add::ChecklistItem(name) => {
                    self.store.add_checklist_item(&name);
                    todo!()
                }
                Add::Recipe {
                    recipe,
                    ingredients,
                } => {
                    self.store.add_recipe(&recipe, &ingredients);
                    todo!()
                }
                Add::Item { name, .. } => {
                    self.store.add_item(&name)?;
                    todo!()
                }
                Add::ListItem(name) => {
                    self.store.add_list_item(&name);
                    todo!()
                }
                Add::ListRecipe(_recipe) => todo!(),
                Add::NewList => todo!(),
            },
            ApiCommand::Delete(cmd) => match cmd {
                Delete::ChecklistItem(name) => {
                    self.store.delete_checklist_item(&name);
                    todo!()
                }
                Delete::ClearChecklist => todo!(),
                Delete::ClearList => todo!(),
                Delete::Item(_name) => todo!(),
                Delete::ListItem(_name) => todo!(),
                Delete::Recipe(recipe) => {
                    self.store.delete_recipe(&recipe).unwrap();
                    todo!()
                }
            },
            ApiCommand::Read(cmd) => match cmd {
                Read::All => {
                    let results = self.store.items();
                    todo!()
                    // display(results, ToDisplay::Items);
                    // Ok(())
                }
                Read::Checklist => {
                    let items = self.store.checklist();
                    todo!()
                    // display(items, ToDisplay::Checklist);
                    // Ok(())
                }
                Read::Item(_name) => todo!(),
                Read::Items => todo!(),
                Read::List => {
                    let cmd = ApiCommand::Read(Read::Checklist);
                    self.execute(cmd)?;
                    let items = self.store.list();
                    todo!()
                }
                Read::ListRecipes => todo!(),
                Read::Recipe(recipe) => match self.store.recipe_ingredients(&recipe) {
                    Ok(Some(ingredients)) => Ok(ApiResponse::RecipeIngredients(ingredients)),
                    Ok(_) => todo!(),
                    Err(e) => todo!(),
                },
                Read::Recipes => Ok(ApiResponse::Recipes(self.store.recipes()?)),
                Read::Sections => {
                    let results = self.store.sections();
                    todo!()
                }
            },
            ApiCommand::Update(cmd) => match cmd {
                Update::Item(_name) => todo!(),
                Update::Recipe(_name) => todo!(),
            },
        }
    }
}

pub enum ApiResponse {
    Checklist,
    Items,
    List,
    Recipes(Vec<RecipeName>),
    RecipeIngredients(Ingredients),
    Sections,
}

impl Display for ApiResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Checklist => write!(f, "checklist"),
            Self::Items => write!(f, "items"),
            Self::List => write!(f, "list"),
            Self::Recipes(recipes) => {
                for recipe in recipes {
                    writeln!(f, "{}", recipe)?;
                }
                Ok(())
            }
            Self::RecipeIngredients(ingredients) => {
                for ingredient in ingredients.iter() {
                    writeln!(f, "{}", ingredient)?;
                }
                Ok(())
            }
            Self::Sections => write!(f, "sections"),
        }
    }
}

#[cfg(test)]
mod tests {
    use common::recipes::RecipeName;

    use crate::ApiResponse;

    #[test]
    fn test_recipes_response_display() {
        let recipes = ApiResponse::Recipes(vec![
            RecipeName::from("peanut butter and jelly sandwich"),
            RecipeName::from("cheese and apple snack"),
        ]);
        insta::assert_display_snapshot!(recipes, @r###"
        peanut butter and jelly sandwich
        cheese and apple snack
        "###);
    }
}
