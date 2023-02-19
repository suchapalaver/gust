use clap::{Arg, Command};

pub fn cli() -> Command<'static> {
    Command::new("grusterylist")
        .about("grusterylist: makes grocery lists, written in rust")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("recipes")
                .about("manages recipes library")
                .subcommand(
                    Command::new("show")
                        .subcommand_required(false)
                        .about("show recipes"),
                )
                .subcommand(
                    Command::new("add-to-db")
                        .subcommand_required(false)
                        .about("adds recipe to db")
                        .arg(
                            Arg::with_name("name")
                                .short('n')
                                .long("name")
                                .required(true)
                                .takes_value(true)
                                .multiple_values(true)
                                .help("provides name of recipe to be added"),
                        ),
                )
                .subcommand(
                    Command::new("add")
                        .subcommand_required(false)
                        .about("adds recipes to library")
                        .arg(
                            Arg::with_name("name")
                                .short('n')
                                .long("name")
                                .required(true)
                                .takes_value(true)
                                .multiple_values(true)
                                .help("provides name of recipe to be added"),
                        )
                        .arg(
                            Arg::with_name("ingredients")
                                .short('i')
                                .long("ingredients")
                                .required(true)
                                .takes_value(true)
                                .multiple_values(true)
                                .help("provides name of recipe to be added"),
                        ),
                )
                .subcommand(
                    Command::new("delete")
                        .about("deletes recipe from library")
                        .arg(
                            Arg::with_name("name")
                                .long("name")
                                .required(true)
                                .takes_value(true)
                                .multiple_values(true)
                                .help("provides name of recipe to be deleted"),
                        ),
                )
                .subcommand(
                    Command::new("delete-from-db")
                        .about("deletes recipe from db")
                        .arg(
                            Arg::with_name("name")
                                .long("name")
                                .required(true)
                                .takes_value(true)
                                .multiple_values(true)
                                .help("provides name of recipe to be deleted"),
                        ),
                )
                .subcommand(
                    Command::new("edit")
                        .about("edits recipes in library")
                        .arg(
                            Arg::with_name("recipe")
                                .long("recipe")
                                .required(true)
                                .takes_value(true)
                                .multiple_values(true)
                                .help("provides name of recipe to be edited"),
                        )
                        .subcommand(
                            Command::new("delete")
                                .about("delete an ingredient from a recipe")
                                .arg(
                                    Arg::with_name("ingredient")
                                        .long("ingredient")
                                        .required(true)
                                        .takes_value(true)
                                        .multiple_values(true)
                                        .help("provides name of ingredient to be deleted"),
                                ),
                        )
                        .subcommand(
                            Command::new("add")
                                .about("adds an ingredient to a recipe")
                                .arg(
                                    Arg::with_name("ingredient")
                                        .long("ingredient")
                                        .required(true)
                                        .takes_value(true)
                                        .multiple_values(true)
                                        .help("provides name of ingredient to be added"),
                                ),
                        )
                        .subcommand(
                            Command::new("edit")
                                .about("edits an ingredient in a recipe")
                                .arg(
                                    Arg::with_name("ingredient")
                                        .long("ingredient")
                                        .required(true)
                                        .takes_value(true)
                                        .multiple_values(true)
                                        .help("provides name of ingredient to be edited"),
                                ),
                        ),
                )
                // --path groceries.json
                .arg(
                    Arg::with_name("path")
                        .long("path")
                        .takes_value(true)
                        .default_value("groceries.json")
                        .help("provides path for groceries library"),
                )
                .arg(
                    Arg::with_name("recipe")
                        .long("recipe")
                        .takes_value(true)
                        .help("provides name of recipe to view"),
                ),
        )
        .subcommand(
            Command::new("groceries")
                .about("manages groceries library")
                .subcommand(Command::new("add").about("Adds grocery items to library"))
                .arg(
                    Arg::with_name("path")
                        .long("path")
                        .takes_value(true)
                        .default_value("groceries.json")
                        .help("provides path for groceries library"),
                ),
        )
        .subcommand(
            Command::new("list")
                .about("makes shopping lists")
                .arg(
                    Arg::with_name("path")
                        .long("path")
                        .takes_value(true)
                        .default_value("list.json")
                        .help("provides path for shopping list"),
                )
                .arg(
                    Arg::with_name("library path")
                        .long("lib-path")
                        .takes_value(true)
                        .default_value("list.json")
                        .help("provides path for groceries library"),
                ),
        )
        .subcommand(Command::new("show-list-sections").about("show shopping sections"))
        .subcommand(Command::new("show-recipes").about("show recipes"))
        .subcommand(
            Command::new("migrate-json-items-to-db")
                .about("transfer old version json file storage to SQLite db"),
        )
        .subcommand(Command::new("show-items-in-db").about("show items in SQLite db"))
}
