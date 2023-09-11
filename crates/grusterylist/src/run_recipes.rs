use clap::ArgMatches;

use db::{models::*, persistence::establish_connection};
use diesel::prelude::*;

pub fn add_recipe_to_db(matches: &ArgMatches) {
    use db::schema::recipes;

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

pub fn delete_recipe_from_db(matches: &ArgMatches) {
    use db::schema::recipes::dsl::*;

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
