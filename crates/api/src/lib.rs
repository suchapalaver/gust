use std::fmt::{self, Display};

use common::{
    commands::{Add, ApiCommand, Delete, Read, Update},
    fetcher::{FetchError, Fetcher},
    item::{Item, ItemName, Section},
    items::Items,
    list::List,
    recipes::{Ingredients, Recipe},
};
use persistence::store::{Storage, Store, StoreError};

use thiserror::Error;
use url::Url;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Fetch error: {0}")]
    FetchError(#[from] FetchError),

    #[error("Store error: {0}")]
    StoreError(#[from] StoreError),
}

pub struct Api {
    store: Store,
}

impl Api {
    pub fn new(store: &str) -> Result<Self, ApiError> {
        let store = Store::new(store)?;
        Ok(Self { store })
    }

    pub async fn execute(&mut self, command: ApiCommand) -> Result<ApiResponse, ApiError> {
        match command {
            ApiCommand::Add(cmd) => self.add(cmd),
            ApiCommand::Delete(cmd) => self.delete(cmd),
            ApiCommand::FetchRecipe(url) => self.fetch_recipe(url).await,
            ApiCommand::MigrateJsonDbToSqlite => self.migrate_json_store_to_sqlite(),
            ApiCommand::Read(cmd) => self.read(cmd).await,
            ApiCommand::Update(cmd) => self.update(cmd),
        }
    }

    fn add(&mut self, cmd: Add) -> Result<ApiResponse, ApiError> {
        match cmd {
            Add::ChecklistItem(name) => {
                self.store.add_checklist_item(&name)?;
                Ok(ApiResponse::ItemAdded(name))
            }
            Add::Recipe {
                recipe,
                ingredients,
            } => {
                self.store.add_recipe(&recipe, &ingredients)?;
                Ok(ApiResponse::RecipeAdded(recipe))
            }
            Add::Item { name, .. } => {
                self.store.add_item(&name)?;
                Ok(ApiResponse::ItemAdded(name))
            }
            Add::ListItem(name) => {
                self.store.add_list_item(&name)?;
                Ok(ApiResponse::ListItemAdded(name))
            }
            Add::ListRecipe(_recipe) => todo!(),
            Add::NewList => todo!(),
        }
    }

    async fn read(&mut self, cmd: Read) -> Result<ApiResponse, ApiError> {
        match cmd {
            Read::All => {
                let results = self.store.items()?;
                Ok(ApiResponse::Items(results))
            }
            Read::Checklist => {
                let items = self.store.checklist()?;
                Ok(ApiResponse::Checklist(items))
            }
            Read::Item(_name) => todo!(),
            Read::List => {
                let list = self.store.list()?;
                Ok(ApiResponse::List(list))
            }
            Read::ListRecipes => todo!(),
            Read::Recipe(recipe) => match self.store.recipe_ingredients(&recipe) {
                Ok(Some(ingredients)) => Ok(ApiResponse::RecipeIngredients(ingredients)),
                Ok(None) => Ok(ApiResponse::NothingReturned(ApiCommand::Read(
                    Read::Recipe(recipe),
                ))),
                Err(e) => Err(e.into()),
            },
            Read::Recipes => Ok(ApiResponse::Recipes(self.store.recipes()?)),
            Read::Sections => {
                let results = self.store.sections()?;
                Ok(ApiResponse::Sections(results))
            }
        }
    }

    fn update(&mut self, cmd: Update) -> Result<ApiResponse, ApiError> {
        match cmd {
            Update::Item(_name) => todo!(),
            Update::Recipe(_name) => todo!(),
        }
    }

    fn delete(&mut self, cmd: Delete) -> Result<ApiResponse, ApiError> {
        match cmd {
            Delete::ChecklistItem(name) => {
                self.store.delete_checklist_item(&name)?;
                Ok(ApiResponse::ChecklistItemDeleted(name))
            }
            Delete::ClearChecklist => todo!(),
            Delete::ClearList => todo!(),
            Delete::Item(_name) => todo!(),
            Delete::ListItem(_name) => todo!(),
            Delete::Recipe(recipe) => {
                self.store.delete_recipe(&recipe)?;
                todo!()
            }
        }
    }

    async fn fetch_recipe(&mut self, url: Url) -> Result<ApiResponse, ApiError> {
        let fetcher = Fetcher::from(url);
        let (recipe, ingredients) = fetcher.fetch_recipe().await?;

        self.store.add_recipe(&recipe, &ingredients)?;
        Ok(ApiResponse::FetchedRecipe((recipe, ingredients)))
    }

    fn migrate_json_store_to_sqlite(&mut self) -> Result<ApiResponse, ApiError> {
        self.store.migrate_json_store_to_sqlite()?;
        Ok(ApiResponse::JsonToSqlite)
    }
}

pub enum ApiResponse {
    Checklist(Vec<Item>),
    ChecklistItemDeleted(ItemName),
    JsonToSqlite,
    Items(Items),
    ItemAdded(ItemName),
    List(List),
    ListItemAdded(ItemName),
    NothingReturned(ApiCommand),
    Recipes(Vec<Recipe>),
    RecipeAdded(Recipe),
    FetchedRecipe((Recipe, Ingredients)),
    RecipeIngredients(Ingredients),
    Sections(Vec<Section>),
}

impl Display for ApiResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Checklist(items) => {
                for item in items {
                    writeln!(f, "{}", item)?;
                }
                Ok(())
            }
            Self::ChecklistItemDeleted(name) => write!(f, "Checklist item deleted: {name}"),
            Self::FetchedRecipe((recipe, ingredients)) => {
                writeln!(f, "\n{}:", recipe)?;
                for ingredient in ingredients.iter() {
                    writeln!(f, "{}", ingredient)?;
                }
                Ok(())
            }
            Self::Items(items) => {
                for item in &items.collection {
                    writeln!(f, "{}", item)?;
                }
                Ok(())
            }
            Self::ItemAdded(name) => write!(f, "Item added: {name}"),
            Self::JsonToSqlite => write!(f, "JSON to SQLite data store migration successful"),
            Self::List(list) => {
                for item in &list.items {
                    writeln!(f, "{}", item)?;
                }
                Ok(())
            }
            Self::ListItemAdded(name) => write!(f, "Item added to list: {name}"),
            Self::NothingReturned(cmd) => write!(f, "Nothing returned for command: {:?}.", cmd),
            Self::Recipes(recipes) => {
                for recipe in recipes {
                    writeln!(f, "{}", recipe)?;
                }
                Ok(())
            }
            Self::RecipeAdded(name) => write!(f, "Recipe added: {name}"),
            Self::RecipeIngredients(ingredients) => {
                for ingredient in ingredients.iter() {
                    writeln!(f, "{}", ingredient)?;
                }
                Ok(())
            }
            Self::Sections(sections) => {
                for section in sections {
                    writeln!(f, "{}", section)?;
                }
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use common::recipes::Recipe;

    use crate::ApiResponse;

    #[test]
    fn test_recipes_response_display() {
        let recipes = ApiResponse::Recipes(vec![
            Recipe::from("peanut butter and jelly sandwich"),
            Recipe::from("cheese and apple snack"),
        ]);
        insta::assert_display_snapshot!(recipes, @r###"
        peanut butter and jelly sandwich
        cheese and apple snack
        "###);
    }
}
