use common::{
    item::ItemName,
    items::Items,
    list::List,
    recipes::{Ingredients, Recipe},
};
use diesel::{prelude::*, sqlite::Sqlite, SqliteConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenv::dotenv;
use std::env;

use crate::{
    models::{
        self, Item, ItemInfo, NewChecklistItem, NewItem, NewItemRecipe, NewListItem, NewRecipe,
        Section,
    },
    schema,
    store::{Storage, StoreError},
};

pub fn establish_connection() -> Result<SqliteConnection, StoreError> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    Ok(SqliteConnection::establish(&database_url)?)
}

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

pub fn run_migrations(connection: &mut impl MigrationHarness<Sqlite>) -> Result<(), StoreError> {
    // This will run the necessary migrations.
    //
    // See the documentation for `MigrationHarness` for
    // all available methods.
    connection.run_pending_migrations(MIGRATIONS)?;

    Ok(())
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

    fn get_or_insert_item(&mut self, name: &str) -> Result<i32, StoreError> {
        diesel::insert_into(schema::items::table)
            .values(NewItem { name })
            .on_conflict_do_nothing()
            .execute(self.connection())?;

        let item_query = schema::items::table.filter(schema::items::dsl::name.eq(name));

        Ok(item_query
            .select(schema::items::dsl::id)
            .first(self.connection())?)
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

    fn insert_item_recipe(
        &mut self,
        item_id: i32,
        recipe_id: i32,
    ) -> Result<(), diesel::result::Error> {
        diesel::insert_into(crate::schema::items_recipes::table)
            .values(NewItemRecipe { item_id, recipe_id })
            .execute(self.connection())?;
        Ok(())
    }

    pub fn load_item(&mut self, item_id: i32) -> Vec<Item> {
        schema::items::table
            .filter(schema::items::dsl::id.eq(&item_id))
            .load::<Item>(self.connection())
            .expect("Error loading item")
    }

    pub fn load_recipe(
        &mut self,
        recipe_name: &str,
    ) -> Result<Vec<models::RecipeModel>, diesel::result::Error> {
        schema::recipes::table
            .filter(schema::recipes::dsl::name.eq(recipe_name))
            .load::<models::RecipeModel>(self.connection())
    }
}

impl Storage for SqliteStore {
    fn add_item(&mut self, item: &ItemName) -> Result<(), StoreError> {
        let item_name = item.to_string();
        let _ = self.get_or_insert_item(&item_name);
        Ok(())
    }

    fn add_checklist_item(&mut self, item: &ItemName) -> Result<(), StoreError> {
        let id = self.get_or_insert_item(item.as_str())?;
        let query = {
            diesel::insert_into(schema::checklist::table)
                .values(NewChecklistItem { id })
                .on_conflict_do_nothing()
        };
        query.execute(self.connection())?;
        Ok(())
    }

    fn add_list_item(&mut self, item: &ItemName) -> Result<(), StoreError> {
        let item_name = item.to_string();
        let id = self.get_or_insert_item(&item_name)?;
        let item_query = diesel::insert_into(crate::schema::list::table)
            .values(NewListItem { id })
            .on_conflict_do_nothing();
        item_query.execute(self.connection())?;
        Ok(())
    }

    fn add_recipe(&mut self, recipe: &Recipe, ingredients: &Ingredients) -> Result<(), StoreError> {
        let recipe_name = recipe.to_string().to_lowercase();
        let recipe_id = self.get_or_insert_recipe(&recipe_name);
        let item_ids = ingredients
            .iter()
            .map(|ingredient| {
                let item_name = ingredient.as_str().to_lowercase();
                self.get_or_insert_item(&item_name)
            })
            .collect::<Result<Vec<i32>, _>>()?;

        for item_id in item_ids {
            self.insert_item_recipe(item_id, recipe_id)?;
        }
        Ok(())
    }

    fn checklist(&mut self) -> Result<Vec<common::item::Item>, StoreError> {
        Ok(schema::items::table
            .filter(
                schema::items::dsl::id
                    .eq_any(schema::checklist::table.select(schema::checklist::dsl::id)),
            )
            .load::<Item>(self.connection())?
            .into_iter()
            .map(|item| item.into())
            .collect())
    }

    fn list(&mut self) -> Result<List, StoreError> {
        Ok(schema::items::table
            .filter(
                schema::items::dsl::id.eq_any(schema::list::table.select(schema::list::dsl::id)),
            )
            .load::<Item>(self.connection())?
            .into_iter()
            .map(|item| item.into())
            .collect())
    }

    fn delete_checklist_item(&mut self, item: &ItemName) -> Result<(), StoreError> {
        let name = item.to_string();
        diesel::delete(
            schema::checklist::table.filter(
                schema::checklist::dsl::id.eq_any(
                    schema::items::table
                        .select(schema::items::dsl::id)
                        .filter(schema::items::dsl::name.eq(&name)),
                ),
            ),
        )
        .execute(self.connection())?;
        Ok(())
    }

    fn delete_recipe(&mut self, recipe: &Recipe) -> Result<(), StoreError> {
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

    fn items(&mut self) -> Result<Items, StoreError> {
        use schema::items::dsl::*;

        Ok(items
            .load::<Item>(self.connection())
            .expect("Error loading items")
            .into_iter()
            .map(|i| i.into())
            .collect())
    }

    fn recipe_ingredients(&mut self, recipe: &Recipe) -> Result<Option<Ingredients>, StoreError> {
        let results = self.load_recipe(recipe.as_str())?;

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

    fn sections(&mut self) -> Result<Vec<common::item::Section>, StoreError> {
        use schema::sections::dsl::*;

        Ok(sections
            .load::<Section>(self.connection())?
            .into_iter()
            .map(|sec| sec.name().into())
            .collect())
    }

    fn recipes(&mut self) -> Result<Vec<Recipe>, StoreError> {
        use schema::recipes::dsl::*;

        Ok(recipes
            .load::<models::RecipeModel>(self.connection())?
            .into_iter()
            .map(|r| r.into())
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::item::ItemName;

    #[test]
    fn test_add_checklist_item() {
        // Set up a connection to an in-memory SQLite database for testing
        let connection = SqliteConnection::establish(":memory:").unwrap();
        let mut store = SqliteStore::new(connection);
        crate::sqlite::run_migrations(store.connection()).unwrap();

        // Add an item to the checklist
        let item_name = ItemName::from("test item");
        store.add_checklist_item(&item_name).unwrap();

        // Check if the item is in the checklist
        let checklist = store.checklist().unwrap();
        let item_in_checklist = checklist.iter().any(|item| item.name() == &item_name);

        // Assert that the item is indeed in the checklist
        assert!(item_in_checklist);
    }
}
