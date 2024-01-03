pub(crate) mod connection;
mod import;
mod migrations;

use common::{
    export::{YamlSerializable, ITEMS_YAML_PATH, LIST_YAML_PATH},
    item::Name,
    items::Items,
    list::List,
    recipes::{Ingredients, Recipe},
};
use diesel::{prelude::*, r2d2::ConnectionManager, SqliteConnection};
use r2d2::PooledConnection;

use crate::{
    import_store::ImportStore,
    models::{
        self, Item, ItemInfo, NewChecklistItem, NewItem, NewItemRecipe, NewItemSection,
        NewListItem, NewListRecipe, NewRecipe, NewSection, RecipeModel, Section,
    },
    schema,
    store::{Storage, StoreError, StoreResponse},
};

use self::{
    connection::{Connection, ConnectionPool, DatabaseConnector, DbUri},
    import::{import_items, import_sections},
    migrations::run_migrations,
};

#[derive(Clone)]
pub struct SqliteStore {
    pool: ConnectionPool,
}

impl SqliteStore {
    pub async fn new(db_uri: DbUri) -> Result<Self, StoreError> {
        let pool = DatabaseConnector::new(db_uri).try_connect().await?;
        let store = Self { pool };
        store.run_migrations()?;
        Ok(store)
    }

    pub(crate) fn run_migrations(&self) -> Result<(), StoreError> {
        let mut connection = self.connection()?;
        connection.immediate_transaction(run_migrations)
    }

    pub(crate) fn connection(
        &self,
    ) -> Result<PooledConnection<ConnectionManager<SqliteConnection>>, r2d2::Error> {
        self.pool.get()
    }

    fn get_or_insert_item(
        connection: &mut SqliteConnection,
        name: &str,
    ) -> Result<i32, StoreError> {
        diesel::insert_into(schema::items::table)
            .values(NewItem { name })
            .on_conflict_do_nothing()
            .execute(connection)?;

        let item_query = schema::items::table.filter(schema::items::dsl::name.eq(name));

        Ok(item_query
            .select(schema::items::dsl::id)
            .first(connection)?)
    }

    fn get_recipe_id(
        connection: &mut SqliteConnection,
        recipe: &str,
    ) -> Result<Option<i32>, StoreError> {
        Ok(schema::recipes::table
            .filter(schema::recipes::dsl::name.eq(recipe))
            .select(schema::recipes::dsl::id)
            .first(connection)
            .optional()?)
    }

    fn get_or_insert_recipe(
        connection: &mut SqliteConnection,
        name: &str,
    ) -> Result<i32, StoreError> {
        match Self::get_recipe_id(connection, name)? {
            Some(id) => Ok(id),
            None => {
                diesel::insert_into(schema::recipes::table)
                    .values(NewRecipe { name })
                    .on_conflict_do_nothing()
                    .execute(connection)?;

                Ok(schema::recipes::table
                    .filter(schema::recipes::dsl::name.eq(name))
                    .select(schema::recipes::dsl::id)
                    .first(connection)?)
            }
        }
    }

    fn insert_item_recipe(
        connection: &mut SqliteConnection,
        item_id: i32,
        recipe_id: i32,
    ) -> Result<(), StoreError> {
        diesel::insert_into(schema::items_recipes::table)
            .values(NewItemRecipe { item_id, recipe_id })
            .on_conflict_do_nothing()
            .execute(connection)?;
        Ok(())
    }

    fn get_section_id(
        connection: &mut SqliteConnection,
        section: &str,
    ) -> Result<Option<i32>, StoreError> {
        Ok(schema::sections::table
            .filter(schema::sections::dsl::name.eq(section))
            .select(schema::sections::dsl::id)
            .first(connection)
            .optional()?)
    }

    fn get_or_insert_section(
        connection: &mut SqliteConnection,
        section: &str,
    ) -> Result<i32, StoreError> {
        match Self::get_section_id(connection, section)? {
            Some(id) => Ok(id),
            None => {
                diesel::insert_into(schema::sections::table)
                    .values(NewSection { name: section })
                    .on_conflict_do_nothing()
                    .execute(connection)?;

                Ok(schema::sections::table
                    .filter(schema::sections::dsl::name.eq(section))
                    .select(schema::sections::dsl::id)
                    .first(connection)?)
            }
        }
    }

    fn insert_item_section(
        connection: &mut SqliteConnection,
        item_id: i32,
        section_id: i32,
    ) -> Result<(), StoreError> {
        diesel::insert_into(schema::items_sections::table)
            .values(NewItemSection {
                item_id,
                section_id,
            })
            .on_conflict_do_nothing()
            .execute(connection)?;
        Ok(())
    }

    async fn get_list(&self) -> Result<List, StoreError> {
        let store = self.clone();
        tokio::task::spawn_blocking(move || {
            let mut connection = store.connection()?;
            connection.immediate_transaction(|connection| {
                Ok(schema::items::table
                    .filter(
                        schema::items::dsl::id
                            .eq_any(schema::list::table.select(schema::list::dsl::id)),
                    )
                    .load::<Item>(connection)?
                    .into_iter()
                    .map(Into::into)
                    .collect::<List>())
            })
        })
        .await?
    }

    async fn get_list_recipes(&self) -> Result<Vec<Recipe>, StoreError> {
        let store = self.clone();
        tokio::task::spawn_blocking(move || {
            let mut connection = store.connection()?;
            connection.immediate_transaction(|connection| {
                Ok(schema::recipes::table
                    .filter(
                        schema::recipes::dsl::id.eq_any(
                            schema::list_recipes::table.select(schema::list_recipes::dsl::id),
                        ),
                    )
                    .load::<RecipeModel>(connection)?
                    .into_iter()
                    .map(Into::into)
                    .collect())
            })
        })
        .await?
    }

    fn load_item(connection: &mut SqliteConnection, item_id: i32) -> Result<Vec<Item>, StoreError> {
        Ok(schema::items::table
            .filter(schema::items::dsl::id.eq(&item_id))
            .load::<Item>(connection)?)
    }

    fn get_recipe(
        connection: &mut SqliteConnection,
        recipe: &str,
    ) -> Result<Option<Vec<RecipeModel>>, StoreError> {
        Ok(schema::recipes::table
            .filter(schema::recipes::dsl::name.eq(recipe))
            .load::<models::RecipeModel>(connection)
            .optional()?)
    }

    fn get_section_for_item(
        connection: &mut SqliteConnection,
        item_id: i32,
    ) -> Result<Option<String>, StoreError> {
        use crate::schema::{items_sections, sections};

        Ok(items_sections::table
            .filter(items_sections::item_id.eq(item_id))
            .left_join(sections::table.on(sections::id.eq(items_sections::section_id)))
            .select(sections::name.nullable())
            .first::<Option<String>>(connection)
            .optional()?
            .flatten())
    }

    fn get_item_recipes(
        connection: &mut SqliteConnection,
        item_id: i32,
    ) -> Result<Vec<String>, StoreError> {
        use crate::schema::{items_recipes, recipes};

        Ok(items_recipes::table
            .filter(items_recipes::item_id.eq(item_id))
            .left_join(recipes::table.on(recipes::id.eq(items_recipes::recipe_id)))
            .select(recipes::name.nullable())
            .load::<Option<String>>(connection)
            .optional()?
            .into_iter()
            .flatten()
            .flatten()
            .collect::<Vec<String>>())
    }
}

impl Storage for SqliteStore {
    async fn add_checklist_item(&self, item: &Name) -> Result<StoreResponse, StoreError> {
        let store = self.clone();
        let item = item.clone();
        tokio::task::spawn_blocking(move || {
            let mut connection = store.connection()?;
            connection.immediate_transaction(|connection| {
                let id = Self::get_or_insert_item(connection, item.as_str())?;
                let query = {
                    diesel::insert_into(schema::checklist::table)
                        .values(NewChecklistItem { id })
                        .on_conflict_do_nothing()
                };
                query.execute(connection)?;
                Ok(StoreResponse::AddedChecklistItem(item))
            })
        })
        .await?
    }

    async fn add_item(
        &self,
        item: &Name,
        section: &Option<common::section::Section>,
    ) -> Result<StoreResponse, StoreError> {
        let store = self.clone();
        let item = item.clone();
        let section = section.clone();
        tokio::task::spawn_blocking(move || {
            let mut connection = store.connection()?;
            connection.immediate_transaction(|connection| {
                let item_name = item.to_string();
                let item_id = Self::get_or_insert_item(connection, &item_name)?;
                if let Some(section) = section {
                    let section_id = Self::get_or_insert_section(connection, section.as_str())?;
                    Self::insert_item_section(connection, item_id, section_id)?;
                }
                Ok(StoreResponse::AddedItem(item))
            })
        })
        .await?
    }

    async fn add_list_item(&self, item: &Name) -> Result<StoreResponse, StoreError> {
        let store = self.clone();
        let item = item.clone();
        tokio::task::spawn_blocking(move || {
            let mut connection = store.connection()?;
            connection.immediate_transaction(|connection| {
                let id = Self::get_or_insert_item(connection, item.as_str())?;
                let query = diesel::insert_into(schema::list::table)
                    .values(NewListItem { id })
                    .on_conflict_do_nothing();
                query.execute(connection)?;
                Ok(StoreResponse::AddedListItem(item))
            })
        })
        .await?
    }

    async fn add_list_recipe(&self, recipe: &Recipe) -> Result<StoreResponse, StoreError> {
        let StoreResponse::RecipeIngredients(Some(ingredients)) =
            self.recipe_ingredients(recipe).await?
        else {
            // TODO:
            return Err(StoreError::RecipeIngredients(recipe.to_string()));
        };

        let store = self.clone();
        let recipe = recipe.clone();
        tokio::task::spawn_blocking(move || {
            let mut connection = store.connection()?;
            connection.immediate_transaction(|connection| {
                let id = Self::get_or_insert_recipe(connection, recipe.as_str())?;
                diesel::insert_into(schema::list_recipes::table)
                    .values(NewListRecipe { id })
                    .on_conflict_do_nothing()
                    .execute(connection)?;
                for item in ingredients.iter() {
                    let item_id = Self::get_or_insert_item(connection, item.as_str())?;
                    let query = diesel::insert_into(schema::list::table)
                        .values(NewListItem { id: item_id })
                        .on_conflict_do_nothing();
                    query.execute(connection)?;

                    let new_item_recipe = NewItemRecipe {
                        item_id,
                        recipe_id: id,
                    };
                    diesel::insert_into(schema::items_recipes::table)
                        .values(&new_item_recipe)
                        .on_conflict_do_nothing()
                        .execute(connection)?;
                }
                Ok(StoreResponse::AddedListRecipe(recipe))
            })
        })
        .await?
    }

    async fn add_recipe(
        &self,
        recipe: &Recipe,
        ingredients: &Ingredients,
    ) -> Result<StoreResponse, StoreError> {
        let store = self.clone();
        let recipe = recipe.clone();
        let ingredients = ingredients.clone();
        tokio::task::spawn_blocking(move || {
            let mut connection: PooledConnection<ConnectionManager<SqliteConnection>> =
                store.connection()?;
            connection.immediate_transaction(|connection| {
                let recipe_id = Self::get_or_insert_recipe(connection, recipe.as_str())?;
                let item_ids = ingredients
                    .iter()
                    .map(|ingredient| Self::get_or_insert_item(connection, ingredient.as_str()))
                    .collect::<Result<Vec<i32>, _>>()?;

                for item_id in item_ids {
                    Self::insert_item_recipe(connection, item_id, recipe_id)?;
                }
                Ok(StoreResponse::AddedRecipe(recipe))
            })
        })
        .await?
    }

    async fn checklist(&self) -> Result<StoreResponse, StoreError> {
        let store = self.clone();
        tokio::task::spawn_blocking(move || {
            let mut connection = store.connection()?;
            connection.immediate_transaction(|connection| {
                Ok(StoreResponse::Checklist(
                    schema::items::table
                        .filter(
                            schema::items::dsl::id.eq_any(
                                schema::checklist::table.select(schema::checklist::dsl::id),
                            ),
                        )
                        .load::<Item>(connection)?
                        .into_iter()
                        .map(Into::into)
                        .collect(),
                ))
            })
        })
        .await?
    }

    async fn list(&self) -> Result<StoreResponse, StoreError> {
        let mut list = self.get_list().await?;
        list = list.with_recipes(self.get_list_recipes().await?);
        let StoreResponse::Checklist(checklist) = self.checklist().await? else {
            todo!()
        };
        list = list.with_checklist(checklist);
        Ok(StoreResponse::List(list))
    }

    async fn delete_checklist_item(&self, item: &Name) -> Result<StoreResponse, StoreError> {
        let store = self.clone();
        let item = item.clone();
        tokio::task::spawn_blocking(move || {
            let mut connection = store.connection()?;
            connection.immediate_transaction(|connection| {
                diesel::delete(
                    schema::checklist::table.filter(
                        schema::checklist::dsl::id.eq_any(
                            schema::items::table
                                .select(schema::items::dsl::id)
                                .filter(schema::items::dsl::name.eq(item.as_str())),
                        ),
                    ),
                )
                .execute(connection)?;
                Ok(StoreResponse::DeletedChecklistItem(item))
            })
        })
        .await?
    }

    async fn delete_recipe(&self, recipe: &Recipe) -> Result<StoreResponse, StoreError> {
        let store = self.clone();
        let recipe = recipe.clone();
        let StoreResponse::RecipeIngredients(ingredients) =
            self.recipe_ingredients(&recipe).await?
        else {
            todo!()
        };
        tokio::task::spawn_blocking(move || {
            let mut connection = store.connection()?;
            connection.immediate_transaction(|connection| {
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
                .execute(connection)?;
                diesel::delete(schema::recipes::table.filter(schema::recipes::dsl::name.eq(name)))
                    .execute(connection)?;
                if let Some(ingredients) = ingredients {
                    for item in ingredients.iter() {
                        diesel::delete(
                            schema::items::table.filter(schema::items::dsl::name.eq(item.as_str())),
                        )
                        .execute(connection)?;
                    }
                }
                Ok(StoreResponse::DeletedRecipe(recipe))
            })
        })
        .await?
    }

    async fn export(&self) -> Result<StoreResponse, StoreError> {
        let items = self.items().await?;
        let StoreResponse::List(list) = self.list().await? else {
            todo!()
        };

        let items = items.collection().to_vec();

        items.serialize_to_yaml_and_write(ITEMS_YAML_PATH)?;
        list.serialize_to_yaml_and_write(LIST_YAML_PATH)?;

        Ok(StoreResponse::Exported(items, list))
    }

    async fn import_from_json(&self) -> Result<StoreResponse, StoreError> {
        let import_store = ImportStore::default();
        let mut connection = self.connection()?;
        let items = import_store.items()?;
        tokio::task::spawn_blocking(move || {
            connection.immediate_transaction(|connection| {
                import_sections(connection)?;
                import_items(connection, items)?;
                Ok(StoreResponse::ImportToSqlite)
            })
        })
        .await?
    }

    async fn items(&self) -> Result<Items, StoreError> {
        use crate::schema::items;

        let store = self.clone();
        tokio::task::spawn_blocking(move || {
            let mut connection = store.connection()?;
            connection.immediate_transaction(|connection| {
                let all_items: Vec<Item> = items::dsl::items.load::<Item>(connection)?;

                all_items
                    .into_iter()
                    .map(|item| {
                        let section = Self::get_section_for_item(connection, item.id)?;
                        let item_recipes = Self::get_item_recipes(connection, item.id)?;

                        let mut item: common::item::Item = item.into();

                        if let Some(section) = section {
                            item = item.with_section(&section);
                        }

                        if !item_recipes.is_empty() {
                            item = item.with_recipes(
                                item_recipes
                                    .into_iter()
                                    .map(Into::into)
                                    .collect::<Vec<Recipe>>()
                                    .as_slice(),
                            );
                        }

                        Ok(item)
                    })
                    .collect::<Result<_, _>>()
            })
        })
        .await?
    }

    async fn refresh_list(&self) -> Result<StoreResponse, StoreError> {
        let store = self.clone();
        tokio::task::spawn_blocking(move || {
            let mut connection = store.connection()?;
            connection.immediate_transaction(|connection| {
                diesel::delete(schema::list::table).execute(connection)?;
                Ok(StoreResponse::RefreshList)
            })
        })
        .await?
    }

    async fn recipe_ingredients(&self, recipe: &Recipe) -> Result<StoreResponse, StoreError> {
        let store = self.clone();
        let recipe = recipe.clone();
        tokio::task::spawn_blocking(move || {
            let mut connection = store.connection()?;
            connection.immediate_transaction(|connection| {
                let Some(results) = Self::get_recipe(connection, recipe.as_str())? else {
                    return Ok(StoreResponse::RecipeIngredients(None));
                };

                let mut v = Vec::<Ingredients>::with_capacity(results.len());

                for recipe in results {
                    let recipe_id = recipe.id;

                    let results = schema::items_recipes::table
                        .filter(schema::items_recipes::dsl::recipe_id.eq(&recipe_id))
                        .load::<models::ItemRecipe>(connection)?;

                    let ingredients = results
                        .iter()
                        .map(|item_recipe| Self::load_item(connection, item_recipe.item_id))
                        .collect::<Result<Vec<Vec<Item>>, _>>()?
                        .into_iter()
                        .flatten()
                        .map(|item| Name::from(item.name.as_str()))
                        .collect::<Ingredients>();

                    v.push(ingredients);
                }

                Ok(StoreResponse::RecipeIngredients(
                    v.into_iter().take(1).next(),
                ))
            })
        })
        .await?
    }

    async fn sections(&self) -> Result<StoreResponse, StoreError> {
        use schema::sections::dsl::sections;
        let store = self.clone();
        tokio::task::spawn_blocking(move || {
            let mut connection = store.connection()?;
            connection.immediate_transaction(|connection| {
                Ok(StoreResponse::Sections(
                    sections
                        .load::<Section>(connection)?
                        .into_iter()
                        .map(|sec| sec.name().into())
                        .collect::<Vec<common::section::Section>>(),
                ))
            })
        })
        .await?
    }

    async fn recipes(&self) -> Result<StoreResponse, StoreError> {
        use schema::recipes::dsl::recipes;
        let store = self.clone();
        tokio::task::spawn_blocking(move || {
            let mut connection = store.connection()?;
            connection.immediate_transaction(|connection| {
                Ok(StoreResponse::Recipes(
                    recipes
                        .load::<models::RecipeModel>(connection)?
                        .into_iter()
                        .map(Into::into)
                        .collect(),
                ))
            })
        })
        .await?
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{item::Name, recipes::Ingredients};

    async fn inmem_sqlite_store() -> SqliteStore {
        // Set up a connection to an in-memory SQLite database for testing
        let store = SqliteStore::new(DbUri::inmem()).await.unwrap();
        let migrations_store = store.clone();
        tokio::task::spawn_blocking(move || {
            let mut connection = migrations_store.connection().unwrap();
            connection.immediate_transaction(run_migrations).unwrap();
        })
        .await
        .unwrap();
        store
    }

    fn test_item_name() -> Name {
        Name::from("test item")
    }

    #[tokio::test]
    async fn test_add_checklist_item() {
        let store = inmem_sqlite_store().await;

        let item_name = test_item_name();
        store.add_checklist_item(&item_name).await.unwrap();

        let StoreResponse::Checklist(list) = store.checklist().await.unwrap() else {
            todo!()
        };

        assert!(list.iter().any(|item| item.name() == &item_name));
    }

    #[tokio::test]
    async fn test_add_item() {
        let store = inmem_sqlite_store().await;

        let item_name = test_item_name();
        store.add_item(&item_name, &None).await.unwrap();

        let items = store.items().await.unwrap();

        assert!(items
            .collection_iter()
            .any(|item| item.name() == &item_name));
    }

    #[tokio::test]
    async fn test_add_list_item() {
        let store = inmem_sqlite_store().await;

        let item_name = test_item_name();
        store.add_list_item(&item_name).await.unwrap();

        let StoreResponse::List(list) = store.list().await.unwrap() else {
            todo!()
        };

        let item_in_list = list.items().iter().any(|item| item.name() == &item_name);

        assert!(item_in_list);
    }

    #[tokio::test]
    async fn test_add_list_recipe() {
        let store = inmem_sqlite_store().await;

        let ingredients =
            Ingredients::from_iter(vec![Name::from("ingredient 1"), Name::from("ingredient 2")]);

        let recipe = Recipe::new("test recipe");
        store.add_recipe(&recipe, &ingredients).await.unwrap();

        store.add_list_recipe(&recipe).await.unwrap();

        let StoreResponse::List(list) = store.list().await.unwrap() else {
            todo!()
        };
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

    #[tokio::test]
    async fn test_add_recipe() {
        let store = inmem_sqlite_store().await;

        let ingredients =
            Ingredients::from_iter(vec![Name::from("ingredient 1"), Name::from("ingredient 2")]);

        let recipe = Recipe::new("test recipe");
        store.add_recipe(&recipe, &ingredients).await.unwrap();

        let StoreResponse::Recipes(recipes) = store.recipes().await.unwrap() else {
            todo!()
        };
        assert_eq!(recipes.len(), 1);

        let added_recipe = &recipes[0];
        assert_eq!(added_recipe.as_str(), "test recipe");

        let StoreResponse::RecipeIngredients(Some(recipe_ingredients)) =
            store.recipe_ingredients(&recipe).await.unwrap()
        else {
            todo!()
        };
        assert_eq!(recipe_ingredients, ingredients);
    }

    #[tokio::test]
    async fn test_delete_checklist_item() {
        let store = inmem_sqlite_store().await;

        let item_name = test_item_name();
        store.add_checklist_item(&item_name).await.unwrap();

        let StoreResponse::Checklist(checklist) = store.checklist().await.unwrap() else {
            todo!()
        };

        assert!(checklist.iter().any(|item| item.name() == &item_name));

        store.delete_checklist_item(&item_name).await.unwrap();

        let StoreResponse::Checklist(checklist) = store.checklist().await.unwrap() else {
            todo!()
        };

        assert!(checklist.iter().all(|item| item.name() != &item_name));
    }

    #[tokio::test]
    async fn test_delete_recipe() {
        let store = inmem_sqlite_store().await;

        let ingredients =
            Ingredients::from_iter(vec![Name::from("ingredient 1"), Name::from("ingredient 2")]);

        let recipe = Recipe::new("test recipe");
        store.add_recipe(&recipe, &ingredients).await.unwrap();

        let StoreResponse::Recipes(recipes) = store.recipes().await.unwrap() else {
            todo!()
        };
        assert_eq!(recipes.len(), 1);

        let added_recipe = &recipes[0];
        assert_eq!(added_recipe.as_str(), "test recipe");

        let StoreResponse::RecipeIngredients(Some(recipe_ingredients)) =
            store.recipe_ingredients(&recipe).await.unwrap()
        else {
            todo!()
        };
        assert_eq!(recipe_ingredients, ingredients);

        store.delete_recipe(&recipe).await.unwrap();

        let StoreResponse::Recipes(recipes) = store.recipes().await.unwrap() else {
            todo!()
        };
        assert_eq!(recipes.len(), 0);

        let StoreResponse::RecipeIngredients(recipe_ingredients) =
            store.recipe_ingredients(&recipe).await.unwrap()
        else {
            todo!()
        };
        assert_eq!(recipe_ingredients, None);
    }

    #[tokio::test]
    async fn test_refresh_list() {
        let store = inmem_sqlite_store().await;

        store.refresh_list().await.unwrap();

        let StoreResponse::List(list) = store.list().await.unwrap() else {
            todo!()
        };
        assert_eq!(list.items().len(), 0);

        let item1 = Name::from("item 1");
        let item2 = Name::from("item 2");
        store.add_list_item(&item1).await.unwrap();
        store.add_list_item(&item2).await.unwrap();

        let StoreResponse::List(list) = store.list().await.unwrap() else {
            todo!()
        };
        assert_eq!(list.items().len(), 2);
        assert!(list.items().iter().any(|item| item.name() == &item1));
        assert!(list.items().iter().any(|item| item.name() == &item2));

        store.refresh_list().await.unwrap();

        let StoreResponse::List(list) = store.list().await.unwrap() else {
            todo!()
        };
        assert_eq!(list.items().len(), 0);
    }

    #[tokio::test]
    async fn test_items() {
        let store = inmem_sqlite_store().await;

        let item1 = Name::from("item 1");
        let item2 = Name::from("item 2");
        let section1 = common::section::Section::from("section 1");
        let section2 = common::section::Section::from("section 2");
        store.add_item(&item1, &Some(section1)).await.unwrap();
        store.add_item(&item2, &Some(section2)).await.unwrap();

        let ingredients = Ingredients::from_iter(vec![Name::from("item 1"), Name::from("item 2")]);
        let recipe = Recipe::new("test recipe");

        store.add_recipe(&recipe, &ingredients).await.unwrap();

        let items = store.items().await.unwrap();

        assert_eq!(items.collection().len(), 2);
        assert!(items.collection_iter().any(|item| item.name() == &item1));
        assert!(items.collection_iter().any(|item| item.name() == &item2));

        insta::assert_debug_snapshot!(items, @r###"
        Items {
            sections: [],
            collection: [
                Item {
                    name: Name(
                        "item 1",
                    ),
                    section: Some(
                        Section(
                            "section 1",
                        ),
                    ),
                    recipes: Some(
                        [
                            Recipe(
                                "test recipe",
                            ),
                        ],
                    ),
                },
                Item {
                    name: Name(
                        "item 2",
                    ),
                    section: Some(
                        Section(
                            "section 2",
                        ),
                    ),
                    recipes: Some(
                        [
                            Recipe(
                                "test recipe",
                            ),
                        ],
                    ),
                },
            ],
            recipes: [],
        }
        "###);
    }
}
