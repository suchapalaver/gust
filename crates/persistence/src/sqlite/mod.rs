use common::{
    item::Name,
    items::Items,
    list::List,
    recipes::{Ingredients, Recipe},
};
use core::fmt;
use diesel::{prelude::*, sqlite::Sqlite, SqliteConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenv::dotenv;
use std::env;

use crate::{
    models::{
        self, Item, ItemInfo, NewChecklistItem, NewItem, NewItemRecipe, NewListItem, NewListRecipe,
        NewRecipe, RecipeModel, Section,
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

impl fmt::Debug for SqliteStore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SQLite")
    }
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

    fn get_recipe(
        &mut self,
        recipe: &str,
    ) -> Result<Option<Vec<RecipeModel>>, diesel::result::Error> {
        schema::recipes::table
            .filter(schema::recipes::dsl::name.eq(recipe))
            .load::<models::RecipeModel>(self.connection())
            .optional()
    }
}

impl Storage for SqliteStore {
    fn add_checklist_item(&mut self, item: &Name) -> Result<(), StoreError> {
        let id = self.get_or_insert_item(item.as_str())?;
        let query = {
            diesel::insert_into(schema::checklist::table)
                .values(NewChecklistItem { id })
                .on_conflict_do_nothing()
        };
        query.execute(self.connection())?;
        Ok(())
    }

    fn add_item(&mut self, item: &Name) -> Result<(), StoreError> {
        let item_name = item.to_string();
        let _ = self.get_or_insert_item(&item_name);
        Ok(())
    }

    fn add_list_item(&mut self, item: &Name) -> Result<(), StoreError> {
        let id = self.get_or_insert_item(item.as_str())?;
        let query = diesel::insert_into(schema::list::table)
            .values(NewListItem { id })
            .on_conflict_do_nothing();
        query.execute(self.connection())?;
        Ok(())
    }

    fn add_list_recipe(&mut self, recipe: &Recipe) -> Result<(), StoreError> {
        let Some(ingredients) = self.recipe_ingredients(recipe)? else {
            return Err(StoreError::RecipeIngredients(recipe.to_string()));
        };

        let id = self.get_or_insert_recipe(recipe.as_str());

        diesel::insert_into(schema::list_recipes::table)
            .values(NewListRecipe { id })
            .on_conflict_do_nothing()
            .execute(self.connection())?;

        for item in ingredients.iter() {
            let item_id = self.get_or_insert_item(item.as_str())?;
            let query = diesel::insert_into(schema::list::table)
                .values(NewListItem { id: item_id })
                .on_conflict_do_nothing();
            query.execute(self.connection())?;

            let new_item_recipe = NewItemRecipe {
                item_id,
                recipe_id: id,
            };
            diesel::insert_into(schema::items_recipes::table)
                .values(&new_item_recipe)
                .on_conflict_do_nothing()
                .execute(self.connection())?;
        }

        Ok(())
    }

    fn add_recipe(&mut self, recipe: &Recipe, ingredients: &Ingredients) -> Result<(), StoreError> {
        let recipe_id = self.get_or_insert_recipe(recipe.as_str());
        let item_ids = ingredients
            .iter()
            .map(|ingredient| self.get_or_insert_item(ingredient.as_str()))
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
            .map(Into::into)
            .collect())
    }

    fn list(&mut self) -> Result<List, StoreError> {
        let mut list = schema::items::table
            .filter(
                schema::items::dsl::id.eq_any(schema::list::table.select(schema::list::dsl::id)),
            )
            .load::<Item>(self.connection())?
            .into_iter()
            .map(Into::into)
            .collect::<List>();

        list.recipes = self.list_recipes()?;

        list.checklist = self.checklist()?;

        Ok(list)
    }

    fn list_recipes(&mut self) -> Result<Vec<Recipe>, StoreError> {
        Ok(schema::recipes::table
            .filter(
                schema::recipes::dsl::id
                    .eq_any(schema::list_recipes::table.select(schema::list_recipes::dsl::id)),
            )
            .load::<RecipeModel>(self.connection())?
            .into_iter()
            .map(Into::into)
            .collect())
    }

    fn delete_checklist_item(&mut self, item: &Name) -> Result<(), StoreError> {
        diesel::delete(
            schema::checklist::table.filter(
                schema::checklist::dsl::id.eq_any(
                    schema::items::table
                        .select(schema::items::dsl::id)
                        .filter(schema::items::dsl::name.eq(item.as_str())),
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
        use schema::items::dsl::items;

        Ok(items
            .load::<Item>(self.connection())?
            .into_iter()
            .map(Into::into)
            .collect())
    }

    fn refresh_list(&mut self) -> Result<(), StoreError> {
        let _ = diesel::delete(schema::list::table).execute(self.connection())?;
        Ok(())
    }

    fn recipe_ingredients(&mut self, recipe: &Recipe) -> Result<Option<Ingredients>, StoreError> {
        let Some(results) = self.get_recipe(recipe.as_str())? else {
            return Ok(None);
        };
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
                .map(|item| Name::from(item.name.as_str()))
                .collect::<Ingredients>();

            v.push(ingredients);
        }

        Ok(v.into_iter().take(1).next())
    }

    fn sections(&mut self) -> Result<Vec<common::item::Section>, StoreError> {
        use schema::sections::dsl::sections;

        Ok(sections
            .load::<Section>(self.connection())?
            .into_iter()
            .map(|sec| sec.name().into())
            .collect())
    }

    fn recipes(&mut self) -> Result<Vec<Recipe>, StoreError> {
        use schema::recipes::dsl::recipes;

        Ok(recipes
            .load::<models::RecipeModel>(self.connection())?
            .into_iter()
            .map(Into::into)
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{item::Name, recipes::Ingredients};

    fn inmem_sqlite_store() -> SqliteStore {
        // Set up a connection to an in-memory SQLite database for testing
        let connection = SqliteConnection::establish(":memory:").unwrap();
        let mut store = SqliteStore::new(connection);
        crate::sqlite::run_migrations(store.connection()).unwrap();
        store
    }

    fn test_item() -> Name {
        Name::from("test item")
    }

    #[test]
    fn test_add_checklist_item() {
        let mut store = inmem_sqlite_store();

        let item_name = test_item();
        store.add_checklist_item(&item_name).unwrap();

        assert!(store
            .checklist()
            .unwrap()
            .iter()
            .any(|item| item.name() == &item_name));
    }

    #[test]
    fn test_add_item() {
        let mut store = inmem_sqlite_store();

        let item_name = test_item();
        store.add_item(&item_name).unwrap();

        let items = store.items().unwrap();

        assert!(items
            .collection
            .iter()
            .any(|item| item.name() == &item_name));
    }

    #[test]
    fn test_add_list_item() {
        let mut store = inmem_sqlite_store();

        let item_name = test_item();
        store.add_list_item(&item_name).unwrap();

        let list = store.list().unwrap();
        let item_in_list = list.items.iter().any(|item| item.name() == &item_name);

        assert!(item_in_list);
    }

    #[test]
    fn test_add_list_recipe() {
        let mut store = inmem_sqlite_store();

        let ingredients =
            Ingredients::from_iter(vec![Name::from("ingredient 1"), Name::from("ingredient 2")]);

        let recipe = Recipe::new("test recipe").unwrap();
        store.add_recipe(&recipe, &ingredients).unwrap();

        store.add_list_recipe(&recipe).unwrap();

        let list = store.list().unwrap();
        insta::assert_debug_snapshot!(list, @r###"
        List {
            checklist: [],
            recipes: [
                Recipe(
                    "test recipe",
                ),
            ],
            items: [
                Item {
                    name: Name(
                        "ingredient 1",
                    ),
                    section: None,
                    recipes: None,
                },
                Item {
                    name: Name(
                        "ingredient 2",
                    ),
                    section: None,
                    recipes: None,
                },
            ],
        }
        "###);
    }

    #[test]
    fn test_add_recipe() {
        let mut store = inmem_sqlite_store();

        let ingredients =
            Ingredients::from_iter(vec![Name::from("ingredient 1"), Name::from("ingredient 2")]);

        let recipe = Recipe::new("test recipe").unwrap();
        store.add_recipe(&recipe, &ingredients).unwrap();

        let recipes = store.recipes().unwrap();
        assert_eq!(recipes.len(), 1);

        let added_recipe = &recipes[0];
        assert_eq!(added_recipe.as_str(), "test recipe");

        let recipe_ingredients = store.recipe_ingredients(&recipe).unwrap().unwrap();
        assert_eq!(recipe_ingredients, ingredients);
    }

    #[test]
    fn test_delete_checklist_item() {
        let mut store = inmem_sqlite_store();

        let item_name = test_item();
        store.add_checklist_item(&item_name).unwrap();

        assert!(store
            .checklist()
            .unwrap()
            .iter()
            .any(|item| item.name() == &item_name));

        store.delete_checklist_item(&item_name).unwrap();

        assert!(store
            .checklist()
            .unwrap()
            .iter()
            .all(|item| item.name() != &item_name));
    }

    #[test]
    fn test_delete_recipe() {
        let mut store = inmem_sqlite_store();

        let ingredients =
            Ingredients::from_iter(vec![Name::from("ingredient 1"), Name::from("ingredient 2")]);

        let recipe = Recipe::new("test recipe").unwrap();
        store.add_recipe(&recipe, &ingredients).unwrap();

        let recipes = store.recipes().unwrap();
        assert_eq!(recipes.len(), 1);

        let added_recipe = &recipes[0];
        assert_eq!(added_recipe.as_str(), "test recipe");

        let recipe_ingredients = store.recipe_ingredients(&recipe).unwrap().unwrap();
        assert_eq!(recipe_ingredients, ingredients);

        store.delete_recipe(&recipe).unwrap();

        let recipes = store.recipes().unwrap();
        assert_eq!(recipes.len(), 0);

        let recipe_ingredients = store.recipe_ingredients(&recipe).unwrap();
        assert_eq!(recipe_ingredients, None);
    }

    #[test]
    fn test_refresh_list() {
        let mut store = inmem_sqlite_store();

        store.refresh_list().unwrap();

        let list = store.list().unwrap();
        assert_eq!(list.items.len(), 0);

        let item1 = Name::from("item 1");
        let item2 = Name::from("item 2");
        store.add_list_item(&item1).unwrap();
        store.add_list_item(&item2).unwrap();

        let list = store.list().unwrap();
        assert_eq!(list.items.len(), 2);
        assert!(list.items.iter().any(|item| item.name() == &item1));
        assert!(list.items.iter().any(|item| item.name() == &item2));

        store.refresh_list().unwrap();

        let list = store.list().unwrap();
        assert_eq!(list.items.len(), 0);
    }
}
