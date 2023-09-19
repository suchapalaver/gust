use api::ApiError;
use clap::{builder::NonEmptyStringValueParser, Arg, Command, ValueHint};
use common::ReadError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CliError {
    #[error("Api error: {0}")]
    ApiError(#[from] ApiError),

    #[error("Invalid input: {0}")]
    ParseInputError(String),

    #[error("Read error: {0}")]
    ReadError(#[from] ReadError),

    #[error("URL parse error: {0}")]
    UrlParseError(#[from] url::ParseError),
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

fn url() -> Arg {
    Arg::new("url")
        .long("url")
        .required(true)
        .value_hint(ValueHint::Url)
        .value_parser(NonEmptyStringValueParser::new())
        .help(
            "URL for recipe, e.g. 'https://www.bbc.co.uk/food/recipes/scrambledeggandtoast_75736'",
        )
}

fn clear_checklist() -> Command {
    Command::new("clear")
        .subcommand_required(false)
        .about("delete everything from checklist")
}

fn refresh_list() -> Command {
    Command::new("clear")
        .subcommand_required(false)
        .about("refresh list")
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
        .subcommand(list().arg(recipe()).arg(item()))
}

fn fetch() -> Command {
    Command::new("fetch")
        .subcommand_required(false)
        .about("fetch recipes from a URL")
        .arg(url())
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
        .subcommand(list().subcommand(refresh_list()))
}

fn migrate() -> Command {
    Command::new("migrate-json-store")
        .subcommand_required(false)
        .about("migrate JSON store to Sqlite database")
}

pub fn cli() -> Command {
    Command::new("gust")
        .about("gust: rust-powered grocery list creator")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(add())
        .subcommand(delete())
        .subcommand(fetch())
        .subcommand(read())
        .subcommand(update())
        .subcommand(migrate())
        .arg(
            Arg::new("store")
                .long("database")
                .num_args(1)
                .value_parser(["json", "sqlite"])
                .default_value("json")
                .help("which database to use"),
        )
}
