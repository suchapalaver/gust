use crate::Groceries;
use clap::ArgMatches;

pub fn run(sync_matches: &ArgMatches) -> Result<(), crate::ReadError> {
    let path = sync_matches.get_one::<String>("path").unwrap();

    match sync_matches.subcommand() {
        Some(("add", s_matches)) => recipes_add(s_matches, path)?,
        Some(("delete", s_matches)) => recipes_delete(s_matches, path)?,
        _ => recipes_print(sync_matches, path)?,
    }
    Ok(())
}

fn recipes_add(s_matches: &ArgMatches, path: &str) -> Result<(), crate::ReadError> {
    let name_elems: Vec<_> = s_matches
        .values_of("name")
        .expect("name is required")
        .collect();
    let n = name_elems.join(" ");
    eprintln!("Recipe: {}", n);
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

fn recipes_delete(s_matches: &ArgMatches, path: &str) -> Result<(), crate::ReadError> {
    let name_elems: Vec<_> = s_matches
        .values_of("name")
        .expect("name is required")
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

fn recipes_print(sync_matches: &ArgMatches, path: &str) -> Result<(), crate::ReadError> {
    let groceries = Groceries::from_path(path)?;
    if let Ok(Some(name)) = sync_matches.try_get_one::<String>("recipe") {
        eprintln!();
        groceries.print_recipe(name)?;
        eprintln!();
    } else {
        eprintln!();
        eprintln!("Here are our recipes:");
        groceries.print_recipes();
        eprintln!();
    }
    Ok(())
}
