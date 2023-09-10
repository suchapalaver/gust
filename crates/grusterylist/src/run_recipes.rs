use clap::ArgMatches;
use common::{
    errors::{GrusterylistError, ReadError},
    groceries::Groceries,
    helpers::ReadWrite,
};

use crate::{models::*, persistence::establish_connection};
use diesel::prelude::*;

fn add_recipe_to_db(matches: &ArgMatches) {
    use crate::schema::recipes;

    let connection = &mut establish_connection();

    let name_elems: Vec<_> = matches
        .get_many::<String>("name")
        .expect("name is required")
        .cloned()
        .collect();
    let name = &name_elems.join(" ");
    eprintln!("Recipe: {}", name);

    let new_recipe = NewRecipe { name };

    diesel::insert_into(recipes::table)
        .values(&new_recipe)
        .on_conflict_do_nothing()
        .execute(connection)
        .expect("Error saving new post");
}

fn delete_recipe_from_db(matches: &ArgMatches) {
    use crate::schema::recipes::dsl::*;

    let connection = &mut establish_connection();

    let name_elems: Vec<_> = matches
        .get_many::<String>("name")
        .expect("name is required")
        .cloned()
        .collect();
    let pattern = &name_elems.join(" ");

    diesel::delete(recipes.filter(name.like(pattern)))
        .execute(connection)
        .expect("Error deleting recipe");
}

fn add_recipe(matches: &ArgMatches, path: &str) -> Result<(), GrusterylistError> {
    // connect to db
    // https://docs.diesel.rs/master/diesel/upsert/struct.IncompleteOnConflict.html#method.do_update
    let name_elems: Vec<_> = matches
        .get_many::<String>("name")
        .expect("name is required")
        .cloned()
        .collect();

    let name = name_elems.join(" ");

    println!("Recipe: {name}");

    let ingredient_vec: Vec<_> = matches
        .get_many::<String>("ingredients")
        .expect("ingredients required")
        .cloned()
        .collect();

    let ingredients = ingredient_vec.join(", ");

    println!("Ingredients: {ingredients}");

    let mut groceries = Groceries::from_path(path)?;

    println!("before adding: {:?}", groceries.recipes);

    groceries.add_recipe(&name, &ingredients)?;

    println!("after adding: {:?}", groceries.recipes);

    groceries.save(path)?;

    Ok(())
}

fn recipes_delete(matches: &ArgMatches, path: &str) -> Result<(), ReadError> {
    let name_elems: Vec<_> = matches
        .get_many::<String>("name")
        .expect("name is required")
        .cloned()
        .collect();

    let n = name_elems.join(" ");

    eprintln!("Recipe: {}", n);

    let mut g = Groceries::from_path(path)?;

    eprintln!("before deleting: {:?}", g.recipes);

    g.delete_recipe(&n)?;

    eprintln!("after: {:?}", g.recipes);

    g.save(path)?;

    Ok(())
}

fn recipes_print(matches: &ArgMatches, path: &str) -> Result<(), ReadError> {
    let groceries = Groceries::from_path(path)?;
    if let Ok(Some(name)) = matches.try_get_one::<String>("recipe") {
        eprintln!();
        eprintln!("RecipeName: {name}");
        eprintln!("Ingredients:");
        for ingredient in groceries.recipe_ingredients(name) {
            eprintln!("{}", ingredient);
        }
        eprintln!();
    } else {
        eprintln!();
        eprintln!("Here are our recipes:");
        for recipe in groceries.recipes() {
            eprintln!("{}", recipe);
        }
        eprintln!();
    }
    Ok(())
}
