use clap::{builder::NonEmptyStringValueParser, Arg, Command, ValueHint};
use common::ReadError;
use persistence::store::ITEMS_JSON_PATH;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CliError {
    #[error("Invalid input: {0}")]
    ParseInputError(String),

    #[error("Read error: {0}")]
    ReadError(#[from] ReadError),
}

fn ingredient() -> Arg {
    Arg::new("ingredient")
        .long("ingredient")
        .num_args(1)
        .value_hint(ValueHint::Unknown)
        .value_parser(NonEmptyStringValueParser::new())
        .help("provides ingredient name")
}

fn ingredients() -> Arg {
    Arg::new("ingredients")
        .short('i')
        .long("ingredients")
        .num_args(1)
        .value_hint(ValueHint::Unknown)
        .value_parser(NonEmptyStringValueParser::new())
        .help("provides name of recipe to be added")
}

fn checklist_item() -> Arg {
    Arg::new("checklist-item")
        .long("checklist-item")
        .value_hint(ValueHint::Unknown)
        .value_parser(NonEmptyStringValueParser::new())
        .help("checklist-item name")
}

fn item() -> Arg {
    Arg::new("item")
        .long("item")
        .value_hint(ValueHint::Unknown)
        .value_parser(NonEmptyStringValueParser::new())
        .help("item name")
}

fn path() -> Arg {
    Arg::new("path")
        .long("path")
        .value_hint(ValueHint::FilePath)
        .help("provides path for shopping list")
}

fn recipe() -> Arg {
    Arg::new("recipe")
        .long("recipe")
        .value_hint(ValueHint::Unknown)
        .value_parser(NonEmptyStringValueParser::new())
        .help("provides recipe name")
}

fn section() -> Arg {
    Arg::new("section")
        .long("section")
        .value_hint(ValueHint::Unknown)
        .value_parser(NonEmptyStringValueParser::new())
        .help("provides item's section")
}

fn clear_checklist() -> Command {
    Command::new("clear")
        .subcommand_required(false)
        .about("delete everything from checklist")
}

fn clear_list() -> Command {
    Command::new("clear")
        .subcommand_required(false)
        .about("delete everything from list")
}

fn read_all_items() -> Command {
    Command::new("all")
        .subcommand_required(false)
        .about("read all items from library")
}

fn sections() -> Command {
    Command::new("sections").about("see sections")
}

fn checklist() -> Command {
    Command::new("checklist")
        .about("work with the checklist")
        .arg(item())
}

fn read_list() -> Command {
    Command::new("list").about("read the list")
}

fn list() -> Command {
    Command::new("list").about("work with the list")
}

fn add() -> Command {
    Command::new("add")
        .subcommand_required(false)
        .about("add stuff")
        .arg(item())
        .arg(section())
        .arg(recipe())
        .arg(ingredients())
        .arg(checklist_item())
        .subcommand(list().arg(item()).arg(recipe()))
}

fn delete() -> Command {
    Command::new("delete")
        .subcommand_required(false)
        .about("delete stuff")
        .subcommand(
            checklist()
                .subcommand(clear_checklist())
                .arg(recipe())
                .arg(checklist_item()),
        )
        .arg(recipe())
        .arg(item())
        .subcommand(list().subcommand(clear_list()).arg(recipe()).arg(item()))
}

fn read() -> Command {
    Command::new("read")
        .subcommand_required(false)
        .about("read stuff")
        .arg(item())
        .arg(recipe())
        .subcommand(read_list())
        .subcommand(checklist())
        .subcommand(read_all_items())
        .subcommand(
            Command::new("recipes")
                .subcommand_required(false)
                .about("read all recipes"),
        )
        .subcommand(sections())
}

fn update() -> Command {
    Command::new("update")
        .subcommand_required(false)
        .about("update stuff")
        .arg(item())
        .subcommand(
            Command::new("recipe")
                .subcommand_required(false)
                .about("update recipe")
                .arg(recipe())
                .arg(ingredient())
                .subcommand(
                    Command::new("delete-ingredient")
                        .about("delete an ingredient from a recipe")
                        .arg(ingredient()),
                )
                .subcommand(
                    Command::new("edit-ingredient")
                        .about("edits an ingredient in a recipe")
                        .arg(ingredient()),
                ),
        )
}

fn migrate_json_db() -> Command {
    Command::new("migrate-json-db")
        .subcommand_required(false)
        .about("migrate a JSON databse to Postgres")
        .arg(path().default_value(ITEMS_JSON_PATH))
}

pub fn cli() -> Command {
    Command::new("grusterylist")
        .about("grusterylist: makes grocery lists, written in rust")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(add())
        .subcommand(delete())
        .subcommand(read())
        .subcommand(update())
        .subcommand(migrate_json_db())
        .arg(
            Arg::new("db")
                .long("database")
                .num_args(1)
                .value_parser(["json", "sqlite"])
                .default_value("sqlite")
                .help("which database to use"),
        )
    ////////////////////////////////////////////////////////////////////////////////
    // Recipes
    /*
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
                    .arg(item()),
            )
            .subcommand(
                Command::new("add")
                    .subcommand_required(false)
                    .about("adds recipes to library")
                    .arg(item())
                    .arg(
                        Arg::new("ingredients")
                            .short('i')
                            .long("ingredients")
                            .required(true)
                            .num_args(1)
                            .value_hint(ValueHint::Unknown)
                            .value_parser(NonEmptyStringValueParser::new()),
                    ),
            )
            .subcommand(
                Command::new("delete")
                    .about("deletes recipe from library")
                    .arg(item()),
            )
            .subcommand(
                Command::new("delete-from-db")
                    .about("deletes recipe from db")
                    .arg(item()),
            )
            .subcommand(
                Command::new("edit")
                    .about("edits recipes in library")
                    .arg(recipe())
                    .subcommand(
                        Command::new("delete")
                            .about("delete an ingredient from a recipe")
                            .arg(
                                ingredient()
                            ),
                    )
                    .subcommand(
                        Command::new("add")
                            .about("adds an ingredient to a recipe")
                            .arg(
                                ingredient()
                            ),
                    )
                    .subcommand(
                        Command::new("edit")
                            .about("edits an ingredient in a recipe")
                            .arg(
                                ingredient()
                            ),
                    ),
            )
            // --path groceries.json
            .arg(path())
            .arg(recipe()),
    )
    // Groceries
    .subcommand(
        Command::new("groceries")
            .about("manages groceries library")
            .subcommand(Command::new("add").about("Adds grocery items to library"))
            .arg(path().default_value(ITEMS_JSON_PATH)),
    )
    // List
    .subcommand(
        Command::new("list")
            .about("makes shopping lists")
            .arg(path().default_value(LIST_JSON_PATH))
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
    */
}
