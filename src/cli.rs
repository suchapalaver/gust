use clap::{Arg, Command};

pub fn cli() -> Command<'static> {
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
                .subcommand(
                    Command::new("edit")
                        .about("Edits recipes in library")
                        .arg(
                            Arg::with_name("recipe")
                                .long("recipe")
                                .required(true)
                                .takes_value(true)
                                .multiple_values(true)
                                .help("Provides name of recipe to be edited"),
                        )
                        .subcommand(
                            Command::new("delete")
                                .about("Delete an ingredient from a recipe")
                                .arg(
                                    Arg::with_name("ingredient")
                                        .long("ingredient")
                                        .required(true)
                                        .takes_value(true)
                                        .multiple_values(true)
                                        .help("Provides name of ingredient to be deleted"),
                                ),
                        )
                        .subcommand(
                            Command::new("add")
                                .about("Adds an ingredient to a recipe")
                                .arg(
                                    Arg::with_name("ingredient")
                                        .long("ingredient")
                                        .required(true)
                                        .takes_value(true)
                                        .multiple_values(true)
                                        .help("Provides name of ingredient to be added"),
                                ),
                        )
                        .subcommand(
                            Command::new("edit")
                                .about("Edits an ingredient in a recipe")
                                .arg(
                                    Arg::with_name("ingredient")
                                        .long("ingredient")
                                        .required(true)
                                        .takes_value(true)
                                        .multiple_values(true)
                                        .help("Provides name of ingredient to be edited"),
                                ),
                        ),
                )
                // --path groceries.json
                .arg(
                    Arg::with_name("path")
                        .long("path")
                        .takes_value(true)
                        .default_value("groceries.json")
                        .help("Provides path for groceries library"),
                )
                .arg(
                    Arg::with_name("recipe")
                        .long("recipe")
                        .takes_value(true)
                        .help("Provides name of recipe to view"),
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
