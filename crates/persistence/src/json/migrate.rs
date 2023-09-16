use common::sections::SECTIONS;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SqliteConnection};

use crate::{
    models::{self, NewItem, NewItemRecipe, NewItemSection, NewRecipe, NewSection},
    schema,
    store::{Storage, StoreError},
};

use super::JsonStore;

fn migrate_sections(connection: &mut SqliteConnection) -> Result<(), StoreError> {
    let sections = SECTIONS;

    use crate::schema::sections;

    for name in sections {
        let section = NewSection { name };

        diesel::insert_into(sections::table)
            .values(&section)
            .on_conflict_do_nothing()
            .execute(connection)
            .expect("Error transferring section");
    }

    Ok(())
}

fn migrate_recipes(
    json: &mut JsonStore,
    connection: &mut SqliteConnection,
) -> Result<(), StoreError> {
    let recipes = json.recipes()?;

    use crate::schema::recipes;

    for recipe in recipes {
        let recipe = NewRecipe {
            name: &recipe.to_string().to_lowercase(),
        };

        diesel::insert_into(recipes::table)
            .values(&recipe)
            .on_conflict_do_nothing()
            .execute(connection)
            .expect("Error transferring recipe");
    }

    Ok(())
}

pub fn migrate_groceries(
    json: &mut JsonStore,
    connection: &mut SqliteConnection,
) -> Result<(), StoreError> {
    migrate_sections(connection)?;
    migrate_recipes(json, connection)?;

    let groceries = json.items()?;
    let items_table = schema::items::table;
    let recipes_table = schema::recipes::table;
    let sections_table = schema::sections::table;

    for item in groceries.collection {
        // add the item to the item table
        let new_item = NewItem {
            name: &item.name.to_string(),
        };

        diesel::insert_into(items_table)
            .values(&new_item)
            .on_conflict_do_nothing()
            .execute(connection)
            .unwrap_or_else(|_| panic!("Error transferring item {}", item.name));

        // get the item's item_id
        let results = items_table
            .filter(schema::items::dsl::name.eq(item.name.to_string()))
            .load::<models::Item>(connection)
            .expect("Error loading recipes");

        assert_eq!(results.len(), 1);

        let item_id = results[0].id;

        if let Some(item_recipes) = item.recipes {
            // log the item_id in items_recipes
            for recipe in item_recipes {
                let new_recipe = NewRecipe {
                    name: &recipe.to_string(),
                };

                diesel::insert_into(schema::recipes::table)
                    .values(&new_recipe)
                    .on_conflict_do_nothing()
                    .execute(connection)
                    .unwrap_or_else(|_| panic!("Error inserting recipe {recipe}"));

                let results = recipes_table
                    .filter(schema::recipes::dsl::name.eq(recipe.to_string()))
                    .load::<models::RecipeModel>(connection)
                    .expect("Error loading recipes");

                assert_eq!(results.len(), 1);

                let recipe_id = results[0].id;

                let new_item_recipe = NewItemRecipe { item_id, recipe_id };

                diesel::insert_into(schema::items_recipes::table)
                    .values(&new_item_recipe)
                    .on_conflict_do_nothing()
                    .execute(connection)
                    .unwrap_or_else(|_| panic!("Error transferring item_recipe for {}", item.name));
            }
        }

        if let Some(item_section) = item.section {
            // log the item_id in items_sections
            let results = sections_table
                .filter(schema::sections::dsl::name.eq(item_section.to_string()))
                .load::<models::Section>(connection)
                .expect("Error loading recipes");

            assert_eq!(results.len(), 1);

            for result in results {
                let section_id = result.id;

                let new_item_section = NewItemSection {
                    item_id,
                    section_id,
                };

                diesel::insert_into(schema::items_sections::table)
                    .values(&new_item_section)
                    .on_conflict_do_nothing()
                    .execute(connection)
                    .unwrap_or_else(|_| {
                        panic!("Error transferring item_section for {}", item.name)
                    });
            }
        }
    }

    Ok(())
}
