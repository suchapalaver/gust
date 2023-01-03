use crate::{persistence::establish_connection, groceries::Groceries, ReadError};
use clap::ArgMatches;
use colored::Colorize;

use crate::models::*;
use diesel::prelude::*;

pub fn run(sync_matches: &ArgMatches) -> Result<(), ReadError> {
    let path = sync_matches.get_one::<String>("path").unwrap();

    match sync_matches.subcommand() {
        Some(("add", s_matches)) => add_recipe(s_matches, path)?,
        Some(("add-to-db", s_matches)) => add_recipe_to_db(s_matches),
        Some(("delete", s_matches)) => recipes_delete(s_matches, path)?,
        Some(("delete-from-db", s_matches)) => delete_recipe_from_db(s_matches),
        Some(("show", _)) => show_recipes(),
        _ => recipes_print(sync_matches, path)?,
    }
    Ok(())
}

fn show_recipes() {
    use crate::schema::recipes::dsl::*;

    let connection = &mut establish_connection();
    let results = recipes
        .load::<Recipe>(connection)
        .expect("Error loading recipes");

    println!(
        "{} {} {}{}",
        "Displaying".blue().bold(),
        results.len().to_string().blue().bold(),
        "recipes".blue().bold(),
        ":".blue().bold()
    );
    for item in results {
        println!(" {} {}", "-".bold().blue(), item.name.blue());
    }
}

fn add_recipe_to_db(s_matches: &ArgMatches) {
    use crate::schema::recipes;

    let connection = &mut establish_connection();

    let name_elems: Vec<_> = s_matches
        .values_of("name")
        .expect("name is required")
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

fn delete_recipe_from_db(s_matches: &ArgMatches) {
    use crate::schema::recipes::dsl::*;

    let connection = &mut establish_connection();

    let name_elems: Vec<_> = s_matches
        .values_of("name")
        .expect("name is required")
        .collect();
    let pattern = &name_elems.join(" ");

    diesel::delete(recipes.filter(name.like(pattern)))
        .execute(connection)
        .expect("Error deleting recipe");
}

fn add_recipe(s_matches: &ArgMatches, path: &str) -> Result<(), ReadError> {
    let name_elems: Vec<_> = s_matches
        .values_of("name")
        .expect("name is required")
        .collect();
    let n = name_elems.join(" ");
    eprintln!("RecipeName: {}", n);
    let ingredient_vec: Vec<_> = s_matches
        .values_of("ingredients")
        .expect("ingredients required")
        .collect();
    let i = ingredient_vec.join(", ");
    eprintln!("Ingredients: {}", i);
    let mut g = Groceries::from_path(path)?;
    eprintln!("before adding: {:?}", g.recipes);
    g.add_recipe(&n, &i)?;
    eprintln!("after adding: {:?}", g.recipes);
    g.save(path)?;
    Ok(())
}

fn recipes_delete(s_matches: &ArgMatches, path: &str) -> Result<(), ReadError> {
    let name_elems: Vec<_> = s_matches
        .values_of("name")
        .expect("name is required")
        .collect();
    let n = name_elems.join(" ");
    eprintln!("RecipeName: {}", n);
    let mut g = Groceries::from_path(path)?;
    eprintln!("before deleting: {:?}", g.recipes);
    g.delete_recipe(&n)?;
    eprintln!("after: {:?}", g.recipes);
    g.save(path)?;
    Ok(())
}

fn recipes_print(sync_matches: &ArgMatches, path: &str) -> Result<(), ReadError> {
    let groceries = Groceries::from_path(path)?;
    if let Ok(Some(name)) = sync_matches.try_get_one::<String>("recipe") {
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
