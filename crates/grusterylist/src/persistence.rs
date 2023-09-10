use common::{
    commands::{Add, ApiCommand, Delete, Read, Update},
    errors::StoreError,
    groceriesitem::ItemName,
    recipes::{Ingredients, RecipeName},
};
use diesel::prelude::*;
// use diesel_migrations::{embed_migrations, EmbeddedMigrations};
use dotenv::dotenv;
use std::{env, str::FromStr};

use crate::{
    models::{Item, NewChecklistItem, NewItem, NewItemRecipe, NewListItem, NewRecipe, Section},
    show::{display, display_sections},
};

// pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub(crate) struct Store {
    pub(crate) connection: SqliteConnection,
}

impl Store {
    pub(crate) fn new(connection: SqliteConnection) -> Self {
        Self { connection }
    }

    pub(crate) fn add_checklist_item(&mut self, item: &ItemName) {
        let item_name = item.to_string();
        let item_id = self.get_or_insert_item(&item_name);
        let item_query = diesel::insert_into(crate::schema::checklist::table)
            .values(NewChecklistItem { item_id })
            .on_conflict_do_nothing();
        item_query
            .execute(&mut self.connection)
            .expect("Error adding item to checklist");
    }

    pub(crate) fn add_item(&mut self, item: &ItemName) {
        let item_name = item.to_string();
        let _ = self.get_or_insert_item(&item_name);
    }

    pub(crate) fn add_list_item(&mut self, item: &ItemName) {
        let item_name = item.to_string();
        let item_id = self.get_or_insert_item(&item_name);
        let item_query = diesel::insert_into(crate::schema::list::table)
            .values(NewListItem { item_id })
            .on_conflict_do_nothing();
        item_query
            .execute(&mut self.connection)
            .expect("Error adding item to list");
    }

    pub(crate) fn add_recipe(&mut self, recipe: &RecipeName, ingredients: &Ingredients) {
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

    pub(crate) fn delete_checklist_item(&mut self, item: &ItemName) {
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
        .execute(&mut self.connection)
        .unwrap();
    }

    pub(crate) fn delete_recipe(&mut self, recipe: &RecipeName) -> Result<(), StoreError> {
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
        .execute(&mut self.connection)
        .unwrap();
        diesel::delete(
            crate::schema::recipes::table.filter(crate::schema::recipes::dsl::name.eq(name)),
        )
        .execute(&mut self.connection)
        .unwrap();
        Ok(())
    }

    pub(crate) fn checklist(&mut self) -> Vec<Item> {
        crate::schema::items::table
            .filter(crate::schema::items::dsl::id.eq_any(
                crate::schema::checklist::table.select(crate::schema::checklist::dsl::item_id),
            ))
            .load::<Item>(&mut self.connection)
            .expect("Error loading checklist")
    }

    fn get_or_insert_recipe(&mut self, name: &str) -> i32 {
        let recipe_query = diesel::insert_into(crate::schema::recipes::table)
            .values(NewRecipe { name })
            .on_conflict_do_nothing();

        recipe_query
            .execute(&mut self.connection)
            .expect("Error inserting recipe");

        let recipe_query =
            crate::schema::recipes::table.filter(crate::schema::recipes::dsl::name.eq(name));

        recipe_query
            .select(crate::schema::recipes::dsl::id)
            .first(&mut self.connection)
            .expect("Error loading recipe")
    }

    fn get_or_insert_item(&mut self, name: &str) -> i32 {
        let item_query = diesel::insert_into(crate::schema::items::table)
            .values(NewItem { name })
            .on_conflict_do_nothing();
        item_query
            .execute(&mut self.connection)
            .expect("Error inserting item");

        let item_query =
            crate::schema::items::table.filter(crate::schema::items::dsl::name.eq(name));

        item_query
            .select(crate::schema::items::dsl::id)
            .first(&mut self.connection)
            .expect("Error loading item")
    }

    fn insert_item_recipe(&mut self, item_id: i32, recipe_id: i32) {
        let item_recipe_query = diesel::insert_into(crate::schema::items_recipes::table)
            .values(NewItemRecipe { item_id, recipe_id })
            .on_conflict(crate::schema::items_recipes::dsl::item_id)
            .do_update()
            .set(crate::schema::items_recipes::dsl::recipe_id.eq(recipe_id));
        item_recipe_query
            .execute(&mut self.connection)
            .expect("Error inserting new item-recipe");
    }

    pub(crate) fn list(&mut self) -> Vec<Item> {
        crate::schema::items::table
            .filter(
                crate::schema::items::dsl::id
                    .eq_any(crate::schema::list::table.select(crate::schema::list::dsl::item_id)),
            )
            .load::<Item>(&mut self.connection)
            .expect("Error loading list")
    }

    pub(crate) fn load_item(&mut self, item_id: i32) -> Vec<Item> {
        crate::schema::items::table
            .filter(crate::schema::items::dsl::id.eq(&item_id))
            .load::<Item>(&mut self.connection)
            .expect("Error loading item")
    }

    pub(crate) fn load_recipe(&mut self, recipe_name: &str) -> Vec<crate::models::Recipe> {
        crate::schema::recipes::table
            .filter(crate::schema::recipes::dsl::name.eq(recipe_name))
            .load::<crate::models::Recipe>(&mut self.connection)
            .expect("Error loading recipe")
    }

    pub(crate) fn recipe_ingredients(
        &mut self,
        recipe: &RecipeName,
    ) -> Vec<(RecipeName, Ingredients)> {
        let results = self.load_recipe(&recipe.0.to_string());

        let mut v = Vec::<(RecipeName, Ingredients)>::with_capacity(results.len());

        for recipe in results {
            let recipe_id = recipe.id;
            let recipe = RecipeName::from_str(recipe.name.as_str()).unwrap();

            let results = crate::schema::items_recipes::table
                .filter(crate::schema::items_recipes::dsl::recipe_id.eq(&recipe_id))
                .load::<crate::models::ItemRecipe>(&mut self.connection)
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

    pub(crate) fn show_items(&mut self) {
        use crate::schema::items::dsl::*;

        let results = items
            .load::<Item>(&mut self.connection)
            .expect("Error loading items");

        display(results, "items");
    }

    pub(crate) fn show_sections(&mut self) {
        use crate::schema::sections::dsl::*;

        let results = sections
            .load::<Section>(&mut self.connection)
            .expect("Error loading sections");

        display_sections(results, "sections");
    }

    pub(crate) fn show_recipes(&mut self) {
        use crate::schema::recipes::dsl::*;

        let results = recipes
            .load::<crate::models::Recipe>(&mut self.connection)
            .expect("Error loading recipes");

        display(results, "recipes");
    }
}

pub(crate) fn execute(command: &ApiCommand, store: &mut Store) {
    match command {
        ApiCommand::Add(Add::ChecklistItem(name)) => store.add_checklist_item(name),
        ApiCommand::Add(Add::Recipe {
            recipe,
            ingredients,
        }) => store.add_recipe(recipe, ingredients),
        ApiCommand::Add(Add::Item { name, .. }) => store.add_item(name),
        ApiCommand::Add(Add::ListItem(name)) => store.add_list_item(name),
        ApiCommand::Add(Add::ListRecipe(recipe)) => todo!(),
        ApiCommand::Add(Add::NewList) => todo!(),
        ApiCommand::Delete(Delete::ChecklistItem(name)) => store.delete_checklist_item(name),
        ApiCommand::Delete(Delete::ClearChecklist) => todo!(),
        ApiCommand::Delete(Delete::ClearList) => todo!(),
        ApiCommand::Delete(Delete::Item(name)) => todo!(),
        ApiCommand::Delete(Delete::ListItem(name)) => todo!(),
        ApiCommand::Delete(Delete::Recipe(recipe)) => store.delete_recipe(recipe).unwrap(),
        ApiCommand::Read(Read::All) => store.show_items(),
        ApiCommand::Read(Read::Checklist) => {
            let items = store.checklist();
            display(items, "checklist")
        }
        ApiCommand::Read(Read::Item(name)) => todo!(),
        ApiCommand::Read(Read::Items) => todo!(),
        ApiCommand::Read(Read::List) => {
            let cmd = ApiCommand::Read(Read::Checklist);
            execute(&cmd, store);
            let items = store.list();
            display(items, "list")
        }
        ApiCommand::Read(Read::ListRecipes) => todo!(),
        ApiCommand::Read(Read::Recipe(recipe)) => {
            let _ = store.recipe_ingredients(recipe);
        }
        ApiCommand::Read(Read::Recipes) => store.show_recipes(),
        ApiCommand::Read(Read::Sections) => store.show_sections(),
        ApiCommand::Update(Update::Item(name)) => todo!(),
        ApiCommand::Update(Update::Recipe(name)) => todo!(),
    }
}
