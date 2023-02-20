use clap::{Arg, Command};

pub fn cli() -> Command<'static> {
    Command::new("grusterylist")
        .about("grusterylist: makes grocery lists, written in rust")
        .subcommand_required(true)
        .arg_required_else_help(true)
        // Add
        .subcommand(
            Command::new("add")
                .subcommand_required(false)
                .about("add stuff")
                .subcommand(
                    Command::new("checklist-item")
                        .subcommand_required(false)
                        .about("add item to checklist")
                        .arg(
                            Arg::with_name("name")
                                .long("name")
                                .required(true)
                                .takes_value(true)
                                .multiple_values(true)
                                .help("provides name of item"),
                        ),
                )
                .subcommand(
                    Command::new("item")
                        .subcommand_required(false)
                        .about("add item to library")
                        .arg(
                            Arg::with_name("name")
                                .long("name")
                                .required(true)
                                .takes_value(true)
                                .multiple_values(true)
                                .help("provides name of item"),
                        )
                        .arg(
                            Arg::with_name("section")
                                .long("section")
                                .required(true)
                                .takes_value(true)
                                .multiple_values(true)
                                .help("provides item's section"),
                        ),
                )
                .subcommand(
                    Command::new("list-item")
                        .subcommand_required(false)
                        .about("add item to list")
                        .arg(
                            Arg::with_name("name")
                                .long("name")
                                .required(true)
                                .takes_value(true)
                                .multiple_values(true)
                                .help("provides name of item"),
                        ),
                )
                .subcommand(
                    Command::new("recipe")
                        .subcommand_required(false)
                        .about("add recipe to library")
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
                ),
        )
        // Delete
        .subcommand(
            Command::new("delete")
                .subcommand_required(false)
                .about("add stuff")
                .subcommand(
                    Command::new("checklist-item")
                        .subcommand_required(false)
                        .about("delete item from checklist")
                        .arg(
                            Arg::with_name("name")
                                .long("name")
                                .required(true)
                                .takes_value(true)
                                .multiple_values(true)
                                .help("provides name of item"),
                        ),
                )
                .subcommand(
                    Command::new("checklist")
                        .subcommand_required(false)
                        .about("clear checklist"),
                )
                .subcommand(
                    Command::new("list")
                        .subcommand_required(false)
                        .about("clear list"),
                )
                .subcommand(
                    Command::new("item")
                        .subcommand_required(false)
                        .about("delete item from library")
                        .arg(
                            Arg::with_name("name")
                                .long("name")
                                .required(true)
                                .takes_value(true)
                                .multiple_values(true)
                                .help("provides name of item"),
                        ),
                )
                .subcommand(
                    Command::new("list-item")
                        .subcommand_required(false)
                        .about("delete item from list")
                        .arg(
                            Arg::with_name("name")
                                .long("name")
                                .required(true)
                                .takes_value(true)
                                .multiple_values(true)
                                .help("provides name of item"),
                        ),
                )
                .subcommand(
                    Command::new("recipe")
                        .subcommand_required(false)
                        .about("delete recipe from library")
                        .arg(
                            Arg::with_name("name")
                                .long("name")
                                .required(true)
                                .takes_value(true)
                                .multiple_values(true)
                                .help("provides name of recipe to be deleted"),
                        ),
                ),
        )
        // Read
        .subcommand(
            Command::new("read")
                .subcommand_required(false)
                .about("read stuff")
                .subcommand(
                    Command::new("read-checklist")
                        .subcommand_required(false)
                        .about("read checklist"),
                )
                .subcommand(
                    Command::new("read-checklist-item")
                        .subcommand_required(false)
                        .about("read item from checklist")
                        .arg(
                            Arg::with_name("name")
                                .long("name")
                                .required(true)
                                .takes_value(true)
                                .multiple_values(true)
                                .help("provides name of item"),
                        ),
                )
                .subcommand(
                    Command::new("read-item")
                        .subcommand_required(false)
                        .about("read item from library")
                        .arg(
                            Arg::with_name("name")
                                .long("name")
                                .required(true)
                                .takes_value(true)
                                .multiple_values(true)
                                .help("provides name of item"),
                        ),
                )
                .subcommand(
                    Command::new("read-items")
                        .subcommand_required(false)
                        .about("read all items from library"),
                )
                .subcommand(
                    Command::new("read-item-recipes")
                        .subcommand_required(false)
                        .about("read recipes associated with an item")
                        .arg(
                            Arg::with_name("name")
                                .long("name")
                                .required(true)
                                .takes_value(true)
                                .multiple_values(true)
                                .help("provides name of item"),
                        ),
                )
                .subcommand(
                    Command::new("read-list")
                        .subcommand_required(false)
                        .about("read list"),
                )
                .subcommand(
                    Command::new("read-list-item")
                        .subcommand_required(false)
                        .about("read an item from the list")
                        .arg(
                            Arg::with_name("name")
                                .long("name")
                                .required(true)
                                .takes_value(true)
                                .multiple_values(true)
                                .help("provides name of item"),
                        ),
                )
                .subcommand(
                    Command::new("read-recipe")
                        .subcommand_required(false)
                        .about("read recipe")
                        .arg(
                            Arg::with_name("recipe")
                                .long("recipe")
                                .takes_value(true)
                                .help("provides name of recipe to view"),
                        ),
                )
                .subcommand(
                    Command::new("read-recipes")
                        .subcommand_required(false)
                        .about("read all recipes"),
                ),
        )
        // Update
        .subcommand(
            Command::new("update-item")
                .subcommand_required(false)
                .about("update commands")
                .subcommand(
                    Command::new("update-item")
                        .subcommand_required(false)
                        .about("update item")
                        .arg(
                            Arg::with_name("name")
                                .long("name")
                                .required(true)
                                .takes_value(true)
                                .multiple_values(true)
                                .help("provides name of item"),
                        ),
                )
                .subcommand(
                    Command::new("update-recipe")
                        .subcommand_required(false)
                        .about("update recipe")
                        .arg(
                            Arg::with_name("recipe")
                                .long("recipe")
                                .required(true)
                                .takes_value(true)
                                .multiple_values(true)
                                .help("provides name of recipe to be edited"),
                        )
                        .subcommand(
                            Command::new("add-ingredient")
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
                            Command::new("delete-ingredient")
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
                            Command::new("edit-ingredient")
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
                ),
        )
        ////////////////////////////////////////////////////////////////////////////////
        // Recipes
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
        // Groceries
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
        // List
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
        // Show
        .subcommand(
            Command::new("query").about("query commands").subcommand(
                Command::new("recipes")
                    .subcommand_required(false)
                    .about("query recipes"),
            ),
        )
        .subcommand(Command::new("show-list-sections").about("show shopping sections"))
        .subcommand(Command::new("show-recipes").about("show recipes"))
        .subcommand(
            Command::new("migrate-json-groceries-to-db")
                .about("transfer old version json file storage to SQLite db"),
        )
        .subcommand(Command::new("show-items-in-db").about("show items in SQLite db"))
}
