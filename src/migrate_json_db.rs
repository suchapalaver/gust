use std::collections::HashSet;

use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SqliteConnection};

use crate::{
    groceries::Groceries,
    models::{self, NewItem, NewItemRecipe, NewItemSection, NewRecipe, NewSection},
    persistence::establish_connection,
    schema, Item, ReadError, RecipeName, Section, ShoppingList,
};

fn load_groceries_library() -> Result<Groceries, ReadError> {
    Groceries::from_path("groceries.json")
}

fn load_list() -> Result<ShoppingList, ReadError> {
    ShoppingList::from_path("list.json")
}

fn load_groceries_collection() -> Result<Vec<Item>, ReadError> {
    Ok(load_groceries_library()?.collection)
}

fn load_recipes() -> Result<Vec<RecipeName>, ReadError> {
    let mut recipes: HashSet<RecipeName> = HashSet::new();

    {
        let groceries = load_groceries_library()?;

        for item in groceries.collection {
            if let Some(item_recipes) = item.recipes {
                for recipe in item_recipes {
                    recipes.insert(recipe);
                }
            }
        }

        for recipe in groceries.recipes {
            recipes.insert(recipe);
        }
    }

    {
        let list = load_list()?;

        for recipe in list.recipes {
            recipes.insert(recipe);
        }
    }

    Ok(recipes.into_iter().collect())
}

fn load_sections() -> Result<Vec<Section>, ReadError> {
    Ok(load_groceries_library()?.sections)
}

fn migrate_sections(connection: &mut SqliteConnection) -> Result<(), ReadError> {
    let sections = load_sections()?;

    use crate::schema::sections;

    for name in sections {
        let section = NewSection {
            name: &name.to_string(),
        };

        diesel::insert_into(sections::table)
            .values(&section)
            .on_conflict_do_nothing()
            .execute(connection)
            .expect("Error transferring section");
    }

    Ok(())
}

fn migrate_recipes(connection: &mut SqliteConnection) -> Result<(), ReadError> {
    let recipes = load_recipes()?;

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

pub(crate) fn migrate_groceries() -> Result<(), ReadError> {
    let connection = &mut establish_connection();

    migrate_sections(connection)?;
    migrate_recipes(connection)?;

    let groceries = load_groceries_collection()?;

    use crate::schema::{items::dsl::*, recipes::dsl::*, sections::dsl::*};

    for item in groceries {
        // add the item to the item table
        let new_item = NewItem {
            name: &item.name.to_string(),
        };

        diesel::insert_into(schema::items::table)
            .values(&new_item)
            .on_conflict_do_nothing()
            .execute(connection)
            .unwrap_or_else(|_| panic!("Error transferring item {}", item.name));

        // get the item's item_id
        let results = items
            .filter(schema::items::dsl::name.eq(item.name.to_string()))
            .load::<models::Item>(connection)
            .expect("Error loading recipes");

        assert_eq!(results.len(), 1);

        let item_id = results[0].id;

        if let Some(item_recipes) = item.recipes {
            // log the item_id in items_recipes
            for r in item_recipes {
                let new_recipe = NewRecipe {
                    name: &r.to_string(),
                };

            diesel::insert_into(schema::recipes::table)
                .values(&new_recipe)
                .on_conflict_do_nothing()
                .execute(connection)
                .unwrap_or_else(|_| panic!("Error inserting recipe {r}"));

                let results = recipes
                    .filter(schema::recipes::dsl::name.eq(r.to_string()))
                    .load::<models::Recipe>(connection)
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
            let results = sections
                .filter(schema::sections::dsl::name.eq(item_section.to_string()))
                .load::<models::Recipe>(connection)
                .expect("Error loading recipes");

            assert_eq!(results.len(), 1);

            let section_id = results[0].id;

            let new_item_section = NewItemSection {
                item_id,
                section_id,
            };

            diesel::insert_into(schema::items_sections::table)
                .values(&new_item_section)
                .on_conflict_do_nothing()
                .execute(connection)
                .unwrap_or_else(|_| panic!("Error transferring item_section for {}", item.name));
        }
    }

    Ok(())
}
