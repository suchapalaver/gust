use crate::ReadError;
use clap::{Arg, Command};

pub fn run() -> Result<(), ReadError> {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("recipes", sync_matches)) => Ok(crate::run_recipes::add_delete(sync_matches)?),
        Some(("groceries", _sync_matches)) => Ok(crate::run_groceries::run()?),
        Some(("list", _sync_matches)) => Ok(crate::run_shopping_list::run()?),
        _ => unreachable!(),
    }
}

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
                )
                // --path groceries.json
                .arg(
                    Arg::with_name("path")
                        .long("path")
                        .takes_value(true)
                        .default_value("groceries.json")
                        .help("Provides path for groceries library"),
                ),
        )
        .subcommand(
            Command::new("groceries")
                .about("Manages groceries library")
                .subcommand(Command::new("add").about("Adds grocery items to library"))
                .arg(
                    Arg::with_name("path")
                        .long("path")
                        .takes_value(true)
                        .default_value("groceries.json")
                        .help("Provides path for groceries library"),
                ),
        )
        .subcommand(
            Command::new("list")
                .about("Makes shopping lists")
                .arg(
                    Arg::with_name("path")
                        .long("path")
                        .takes_value(true)
                        .default_value("list.json")
                        .help("Provides path for shopping list"),
                )
                .arg(
                    Arg::with_name("library path")
                        .long("lib-path")
                        .takes_value(true)
                        .default_value("groceries.json")
                        .help("Provides path for groceries library"),
                ),
        )
}
