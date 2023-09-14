// pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

use common::{
    item::ItemName,
    items::Groceries,
    list::ShoppingList,
    recipes::{Ingredients, RecipeName},
};
use diesel::prelude::*;
// use diesel_migrations::{embed_migrations, EmbeddedMigrations};
use diesel::SqliteConnection;
use dotenv::dotenv;
use std::env;

use crate::{
    models::{
        self, Item, NewChecklistItem, NewItem, NewItemRecipe, NewListItem, NewRecipe, Section,
    },
    schema,
    store::{Storage, StoreError},
};

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub struct SqliteStore {
    connection: SqliteConnection,
}

impl SqliteStore {
    pub fn new(connection: SqliteConnection) -> Self {
        Self { connection }
    }

    pub fn connection(&mut self) -> &mut SqliteConnection {
        &mut self.connection
    }

    fn get_or_insert_item(&mut self, name: &str) -> i32 {
        let item_query = diesel::insert_into(crate::schema::items::table)
            .values(NewItem { name })
            .on_conflict_do_nothing();
        item_query
            .execute(self.connection())
            .expect("Error inserting item");

        let item_query = schema::items::table.filter(schema::items::dsl::name.eq(name));

        item_query
            .select(schema::items::dsl::id)
            .first(self.connection())
            .expect("Error loading item")
    }

    fn get_or_insert_recipe(&mut self, name: &str) -> i32 {
        diesel::insert_into(schema::recipes::table)
            .values(NewRecipe { name })
            .on_conflict_do_nothing()
            .execute(self.connection())
            .expect("Error inserting recipe");

        let recipe_query = schema::recipes::table.filter(schema::recipes::dsl::name.eq(name));

        recipe_query
            .select(schema::recipes::dsl::id)
            .first(self.connection())
            .expect("Error loading recipe")
    }

    fn insert_item_recipe(&mut self, item_id: i32, recipe_id: i32) {
        let item_recipe_query = diesel::insert_into(crate::schema::items_recipes::table)
            .values(NewItemRecipe { item_id, recipe_id })
            .on_conflict(schema::items_recipes::dsl::item_id)
            .do_update()
            .set(schema::items_recipes::dsl::recipe_id.eq(recipe_id));
        item_recipe_query
            .execute(self.connection())
            .expect("Error inserting new item-recipe");
    }

    pub fn load_item(&mut self, item_id: i32) -> Vec<Item> {
        schema::items::table
            .filter(schema::items::dsl::id.eq(&item_id))
            .load::<Item>(self.connection())
            .expect("Error loading item")
    }

    pub fn load_recipe(&mut self, recipe_name: &str) -> Vec<models::Recipe> {
        schema::recipes::table
            .filter(schema::recipes::dsl::name.eq(recipe_name))
            .load::<models::Recipe>(self.connection())
            .expect("Error loading recipe")
    }
}

impl Storage for SqliteStore {
    fn add_item(&mut self, item: &ItemName) -> Result<(), StoreError> {
        let item_name = item.to_string();
        let _ = self.get_or_insert_item(&item_name);
        Ok(())
    }

    fn add_checklist_item(&mut self, item: &ItemName) {
        let item_name = item.to_string();
        let item_id = self.get_or_insert_item(&item_name);
        let item_query = {
            diesel::insert_into(schema::checklist::table)
                .values(NewChecklistItem { item_id })
                .on_conflict_do_nothing()
        };
        item_query
            .execute(self.connection())
            .expect("Error adding item to checklist");
    }

    fn add_list_item(&mut self, item: &ItemName) {
        let item_name = item.to_string();
        let item_id = self.get_or_insert_item(&item_name);
        let item_query = diesel::insert_into(crate::schema::list::table)
            .values(NewListItem { item_id })
            .on_conflict_do_nothing();
        item_query
            .execute(self.connection())
            .expect("Error adding item to list");
    }

    fn add_recipe(&mut self, recipe: &RecipeName, ingredients: &Ingredients) {
        let recipe_name = recipe.to_string().to_lowercase();
        let recipe_id = self.get_or_insert_recipe(&recipe_name);
        let item_ids: Vec<i32> = ingredients
            .iter()
            .map(|ingredient| {
                let item_name = ingredient.as_str().to_lowercase();
                self.get_or_insert_item(&item_name)
            })
            .collect();

        for item_id in item_ids {
            self.insert_item_recipe(item_id, recipe_id);
        }
    }

    fn checklist(&mut self) -> Vec<Item> {
        schema::items::table
            .filter(
                schema::items::dsl::id
                    .eq_any(schema::checklist::table.select(schema::checklist::dsl::item_id)),
            )
            .load::<Item>(self.connection())
            .expect("Error loading checklist")
    }

    fn list(&mut self) -> Result<ShoppingList, StoreError> {
        Ok(schema::items::table
            .filter(
                schema::items::dsl::id
                    .eq_any(schema::list::table.select(schema::list::dsl::item_id)),
            )
            .load::<Item>(self.connection())
            .expect("Error loading list")
            .into_iter()
            .map(|item| item.into())
            .collect())
    }

    fn delete_checklist_item(&mut self, item: &ItemName) {
        let name = item.to_string();
        diesel::delete(
            schema::checklist::table.filter(
                schema::checklist::dsl::item_id.eq_any(
                    schema::items::table
                        .select(schema::items::dsl::id)
                        .filter(schema::items::dsl::name.eq(&name)),
                ),
            ),
        )
        .execute(self.connection())
        .unwrap();
    }

    fn delete_recipe(&mut self, recipe: &RecipeName) -> Result<(), StoreError> {
        let name = recipe.to_string();
        diesel::delete(
            schema::items_recipes::table.filter(
                schema::items_recipes::dsl::recipe_id.eq_any(
                    schema::recipes::table
                        .select(schema::recipes::dsl::id)
                        .filter(schema::recipes::dsl::name.eq(&name)),
                ),
            ),
        )
        .execute(self.connection())
        .expect("Error deleting recipe");
        diesel::delete(schema::recipes::table.filter(schema::recipes::dsl::name.eq(name)))
            .execute(self.connection())?;

        Ok(())
    }

    fn items(&mut self) -> Result<Groceries, StoreError> {
        use schema::items::dsl::*;

        Ok(items
            .load::<Item>(self.connection())
            .expect("Error loading items")
            .into_iter()
            .map(|i| i.into())
            .collect())
    }

    fn recipe_ingredients(
        &mut self,
        recipe: &RecipeName,
    ) -> Result<Option<Ingredients>, StoreError> {
        let results = self.load_recipe(recipe.as_str());

        let mut v = Vec::<Ingredients>::with_capacity(results.len());

        for recipe in results {
            let recipe_id = recipe.id;

            let results = schema::items_recipes::table
                .filter(schema::items_recipes::dsl::recipe_id.eq(&recipe_id))
                .load::<models::ItemRecipe>(self.connection())
                .expect("Error loading recipe");

            let ingredients = results
                .iter()
                .flat_map(|item_recipe| self.load_item(item_recipe.item_id))
                .map(|item| ItemName::from(item.name.as_str()))
                .collect::<Ingredients>();

            v.push(ingredients);
        }

        Ok(v.into_iter().take(1).next())
    }

    fn sections(&mut self) -> Vec<Section> {
        use schema::sections::dsl::*;

        sections
            .load::<Section>(self.connection())
            .expect("Error loading sections")
    }

    fn recipes(&mut self) -> Result<Vec<RecipeName>, StoreError> {
        use schema::recipes::dsl::*;

        Ok(recipes
            .load::<models::Recipe>(self.connection())
            .expect("Error loading recipes")
            .into_iter()
            .map(|r| r.into())
            .collect())
    }
}
