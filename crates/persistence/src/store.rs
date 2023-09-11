use common::{
    errors::StoreError,
    groceriesitem::ItemName,
    recipes::{Ingredients, RecipeName},
};
use diesel::prelude::*;
// use diesel_migrations::{embed_migrations, EmbeddedMigrations};
use dotenv::dotenv;
use std::{env, str::FromStr};

use crate::models::{
    self, Item, NewChecklistItem, NewItem, NewItemRecipe, NewListItem, NewRecipe, Recipe, Section,
};

// pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub enum Store {
    Json,
    Sqlite(SqliteStore),
}

pub struct SqliteStore {
    connection: SqliteConnection,
}

impl Store {
    pub fn new_sqlite(connection: SqliteConnection) -> Self {
        Self::Sqlite(SqliteStore { connection })
    }

    pub fn sqlite_connection(&mut self) -> &mut SqliteConnection {
        match self {
            Self::Sqlite(store) => &mut store.connection,
            _ => unreachable!(),
        }
    }

    pub fn add_checklist_item(&mut self, item: &ItemName) {
        match self {
            Self::Sqlite(_) => {
                let item_name = item.to_string();
                let item_id = self.get_or_insert_item(&item_name);
                let item_query = {
                    diesel::insert_into(crate::schema::checklist::table)
                        .values(NewChecklistItem { item_id })
                        .on_conflict_do_nothing()
                };
                item_query
                    .execute(self.sqlite_connection())
                    .expect("Error adding item to checklist");
            }
            Self::Json => unimplemented!(),
        }
    }

    pub fn add_item(&mut self, item: &ItemName) {
        let item_name = item.to_string();
        let _ = self.get_or_insert_item(&item_name);
    }

    pub fn add_list_item(&mut self, item: &ItemName) {
        match self {
            Self::Sqlite(_) => {
                let item_name = item.to_string();
                let item_id = self.get_or_insert_item(&item_name);
                let item_query = diesel::insert_into(crate::schema::list::table)
                    .values(NewListItem { item_id })
                    .on_conflict_do_nothing();
                item_query
                    .execute(self.sqlite_connection())
                    .expect("Error adding item to list");
            }
            Self::Json => unimplemented!(),
        }
    }

    pub fn add_recipe(&mut self, recipe: &RecipeName, ingredients: &Ingredients) {
        let recipe_name = recipe.to_string().to_lowercase();
        let recipe_id = self.get_or_insert_recipe(&recipe_name);
        let item_ids: Vec<i32> = ingredients
            .iter()
            .map(|ingredient| {
                let item_name = ingredient.0.to_string().to_lowercase();
                self.get_or_insert_item(&item_name)
            })
            .collect();

        for item_id in item_ids {
            self.insert_item_recipe(item_id, recipe_id);
        }
    }

    pub fn delete_checklist_item(&mut self, item: &ItemName) {
        match self {
            Self::Sqlite(store) => {
                let name = item.to_string();
                diesel::delete(
                    crate::schema::checklist::table.filter(
                        crate::schema::checklist::dsl::item_id.eq_any(
                            crate::schema::items::table
                                .select(crate::schema::items::dsl::id)
                                .filter(crate::schema::items::dsl::name.eq(&name)),
                        ),
                    ),
                )
                .execute(&mut store.connection)
                .unwrap();
            }
            Self::Json => unimplemented!(),
        }
    }

    pub fn delete_recipe(&mut self, recipe: &RecipeName) -> Result<(), StoreError> {
        match self {
            Self::Sqlite(store) => {
                let name = recipe.to_string();
                diesel::delete(
                    crate::schema::items_recipes::table.filter(
                        crate::schema::items_recipes::dsl::recipe_id.eq_any(
                            crate::schema::recipes::table
                                .select(crate::schema::recipes::dsl::id)
                                .filter(crate::schema::recipes::dsl::name.eq(&name)),
                        ),
                    ),
                )
                .execute(&mut store.connection)
                .unwrap();
                diesel::delete(
                    crate::schema::recipes::table
                        .filter(crate::schema::recipes::dsl::name.eq(name)),
                )
                .execute(&mut store.connection)
                .unwrap();
            }
            Self::Json => unimplemented!(),
        }

        Ok(())
    }

    pub fn checklist(&mut self) -> Vec<Item> {
        match self {
            Self::Sqlite(store) => crate::schema::items::table
                .filter(crate::schema::items::dsl::id.eq_any(
                    crate::schema::checklist::table.select(crate::schema::checklist::dsl::item_id),
                ))
                .load::<Item>(&mut store.connection)
                .expect("Error loading checklist"),
            Self::Json => unimplemented!(),
        }
    }

    fn get_or_insert_recipe(&mut self, name: &str) -> i32 {
        match self {
            Self::Sqlite(store) => {
                let recipe_query = diesel::insert_into(crate::schema::recipes::table)
                    .values(NewRecipe { name })
                    .on_conflict_do_nothing();

                recipe_query
                    .execute(&mut store.connection)
                    .expect("Error inserting recipe");

                let recipe_query = crate::schema::recipes::table
                    .filter(crate::schema::recipes::dsl::name.eq(name));

                recipe_query
                    .select(crate::schema::recipes::dsl::id)
                    .first(&mut store.connection)
                    .expect("Error loading recipe")
            }
            Self::Json => unimplemented!(),
        }
    }

    fn get_or_insert_item(&mut self, name: &str) -> i32 {
        match self {
            Self::Sqlite(store) => {
                let item_query = diesel::insert_into(crate::schema::items::table)
                    .values(NewItem { name })
                    .on_conflict_do_nothing();
                item_query
                    .execute(&mut store.connection)
                    .expect("Error inserting item");

                let item_query =
                    crate::schema::items::table.filter(crate::schema::items::dsl::name.eq(name));

                item_query
                    .select(crate::schema::items::dsl::id)
                    .first(&mut store.connection)
                    .expect("Error loading item")
            }
            Self::Json => unimplemented!(),
        }
    }

    fn insert_item_recipe(&mut self, item_id: i32, recipe_id: i32) {
        match self {
            Self::Sqlite(store) => {
                let item_recipe_query = diesel::insert_into(crate::schema::items_recipes::table)
                    .values(NewItemRecipe { item_id, recipe_id })
                    .on_conflict(crate::schema::items_recipes::dsl::item_id)
                    .do_update()
                    .set(crate::schema::items_recipes::dsl::recipe_id.eq(recipe_id));
                item_recipe_query
                    .execute(&mut store.connection)
                    .expect("Error inserting new item-recipe");
            }
            Self::Json => unimplemented!(),
        }
    }

    pub fn list(&mut self) -> Vec<Item> {
        match self {
            Self::Sqlite(store) => {
                crate::schema::items::table
                    .filter(crate::schema::items::dsl::id.eq_any(
                        crate::schema::list::table.select(crate::schema::list::dsl::item_id),
                    ))
                    .load::<Item>(&mut store.connection)
                    .expect("Error loading list")
            }
            Self::Json => unimplemented!(),
        }
    }

    pub fn load_item(&mut self, item_id: i32) -> Vec<Item> {
        match self {
            Self::Sqlite(store) => crate::schema::items::table
                .filter(crate::schema::items::dsl::id.eq(&item_id))
                .load::<Item>(&mut store.connection)
                .expect("Error loading item"),
            Self::Json => unimplemented!(),
        }
    }

    pub fn load_recipe(&mut self, recipe_name: &str) -> Vec<crate::models::Recipe> {
        match self {
            Self::Sqlite(store) => crate::schema::recipes::table
                .filter(crate::schema::recipes::dsl::name.eq(recipe_name))
                .load::<crate::models::Recipe>(&mut store.connection)
                .expect("Error loading recipe"),
            Self::Json => unimplemented!(),
        }
    }

    pub fn recipe_ingredients(&mut self, recipe: &RecipeName) -> Vec<(RecipeName, Ingredients)> {
        match self {
            Self::Sqlite(_) => {
                let results = self.load_recipe(&recipe.0.to_string());

                let mut v = Vec::<(RecipeName, Ingredients)>::with_capacity(results.len());

                for recipe in results {
                    let recipe_id = recipe.id;
                    let recipe = RecipeName::from_str(recipe.name.as_str()).unwrap();

                    let results = crate::schema::items_recipes::table
                        .filter(crate::schema::items_recipes::dsl::recipe_id.eq(&recipe_id))
                        .load::<crate::models::ItemRecipe>(self.sqlite_connection())
                        .expect("Error loading recipe");

                    let ingredients = results
                        .iter()
                        .flat_map(|item_recipe| self.load_item(item_recipe.item_id))
                        .map(|item| ItemName::from(item.name.as_str()))
                        .collect::<Ingredients>();

                    println!("Recipe: {recipe}");
                    println!("Ingredients: {ingredients:#?}");
                    v.push((recipe, ingredients));
                }
                v
            }
            Self::Json => unimplemented!(),
        }
    }

    pub fn items(&mut self) -> Vec<Item> {
        match self {
            Self::Sqlite(store) => {
                use crate::schema::items::dsl::*;

                items
                    .load::<Item>(&mut store.connection)
                    .expect("Error loading items")
            }
            Self::Json => unimplemented!(),
        }
    }

    pub fn sections(&mut self) -> Vec<Section> {
        match self {
            Self::Sqlite(store) => {
                use crate::schema::sections::dsl::*;

                sections
                    .load::<Section>(&mut store.connection)
                    .expect("Error loading sections")
            }
            Self::Json => unimplemented!(),
        }
    }

    pub fn recipes(&mut self) -> Vec<Recipe> {
        match self {
            Self::Sqlite(store) => {
                use crate::schema::recipes::dsl::*;

                recipes
                    .load::<models::Recipe>(&mut store.connection)
                    .expect("Error loading recipes")
            }
            Self::Json => unimplemented!(),
        }
    }
}
