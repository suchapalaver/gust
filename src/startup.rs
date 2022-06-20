use crate::{Groceries, ReadError};
use clap::Arg;
use clap::Command;

fn cli() -> Command<'static> {
    Command::new("grusterylist")
        .about("Makes grocery lists")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("recipes")
                .about("Manages recipes library")
                .subcommand(
                    Command::new("add")
                        .subcommand_required(false)
                        .about("Adds recipes to library")
                        .arg(
                            Arg::with_name("name")
                                .short('n')
                                .long("name")
                                .required(true)
                                .takes_value(true)
                                .multiple_values(true)
                                .help("Provides name of recipe to be added"),
                        )
                        .arg(
                            Arg::with_name("ingredients")
                                .short('i')
                                .long("ingredients")
                                .required(true)
                                .takes_value(true)
                                .multiple_values(true)
                                .help("Provides name of recipe to be added"),
                        ),
                )
                .subcommand(
                    Command::new("delete")
                        .about("Deletes recipe from library")
                        .arg(
                            Arg::with_name("name")
                                .long("name")
                                .required(true)
                                .takes_value(true)
                                .multiple_values(true)
                                .help("Provides name of recipe to be deleted"),
                        ),
                ),
        )
        .subcommand(
            Command::new("groceries")
                .about("Manages groceries library")
                .subcommand(Command::new("add").about("Adds grocery items to library")),
        )
        .subcommand(
            Command::new("list")
                .about("Makes shopping lists")
                .arg(
                    Arg::with_name("path")
                        .long("path")
                        .required(false)
                        .takes_value(true)
                        .default_value("list.json")
                        .help("Provides path for shopping list"),
                )
                .arg(
                    Arg::with_name("library")
                        .long("library")
                        .required(false)
                        .takes_value(true)
                        .default_value("groceries.json")
                        .help("Provides path for groceries library"),
                ),
        )
}

pub fn run() -> Result<(), ReadError> {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("recipes", sync_matches)) => match sync_matches.subcommand() {
            Some(("add", s_matches)) => {
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
                // let i = Ingredients::from_input_string(i)?;
                let mut g = Groceries::from_path("groceries.json")?;
                eprintln!("before adding: {:?}", g.recipes);
                g.add_recipe(&n, &i)?;
                eprintln!("after adding: {:?}", g.recipes);
                g.save("groceries.json")?;
                Ok(())
            }
            Some(("delete", s_matches)) => {
                let name_elems: Vec<_> = s_matches
                    .values_of("name")
                    .expect("name is required")
                    .collect();
                let n = name_elems.join(" ");
                eprintln!("Recipe: {}", n);
                let mut g = Groceries::from_path("groceries.json")?;
                eprintln!("before deleting: {:?}", g.recipes);
                g.delete_recipe(&n)?;
                eprintln!("after: {:?}", g.recipes);
                g.save("groceries.json")?;
                Ok(())
            }
            _ => {
                crate::run_recipes::run()?;
                Ok(())
            }
        },
        Some(("groceries", _sync_matches)) => Ok(crate::run_groceries::run()?),
        Some(("list", _sync_matches)) => Ok(crate::run_shopping_list::run()?),
        _ => unreachable!(),
    }
}
