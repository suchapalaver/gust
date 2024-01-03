use common::{items::Items, section::SECTIONS};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SqliteConnection};

use crate::{
    models::{self, NewItem, NewItemRecipe, NewItemSection, NewRecipe, NewSection},
    schema,
    store::StoreError,
};

pub fn migrate_sections(connection: &mut SqliteConnection) -> Result<(), StoreError> {
    use crate::schema::sections;

    let sections = SECTIONS;

    for name in sections {
        let section = NewSection { name };

        diesel::insert_into(sections::table)
            .values(&section)
            .on_conflict_do_nothing()
            .execute(connection)?;
    }

    Ok(())
}

pub fn migrate_items(connection: &mut SqliteConnection, items: Items) -> Result<(), StoreError> {
    let items_table = schema::items::table;
    let recipes_table = schema::recipes::table;
    let sections_table = schema::sections::table;

    for item in items.collection_iter() {
        // add the item to the item table
        let new_item = NewItem {
            name: item.name().as_str(),
        };

        diesel::insert_into(items_table)
            .values(&new_item)
            .on_conflict_do_nothing()
            .execute(connection)?;

        // get the item's item_id
        let results = items_table
            .filter(schema::items::dsl::name.eq(item.name().to_string()))
            .load::<models::Item>(connection)?;

        assert_eq!(results.len(), 1);

        let item_id = results[0].id;

        if let Some(item_recipes) = item.recipes() {
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
                    .load::<models::RecipeModel>(connection)?;

                assert_eq!(results.len(), 1);

                let recipe_id = results[0].id;

                let new_item_recipe = NewItemRecipe { item_id, recipe_id };

                diesel::insert_into(schema::items_recipes::table)
                    .values(&new_item_recipe)
                    .on_conflict_do_nothing()
                    .execute(connection)
                    .unwrap_or_else(|_| {
                        panic!("Error transferring item_recipe for {}", item.name())
                    });
            }
        }

        if let Some(item_section) = item.section() {
            // log the item_id in items_sections
            let results = sections_table
                .filter(schema::sections::dsl::name.eq(item_section.to_string()))
                .load::<models::Section>(connection)?;

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
                        panic!("Error transferring item_section for {}", item.name())
                    });
            }
        }
    }

    Ok(())
}
