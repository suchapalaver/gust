use std::str::FromStr;

use api::Api;
use clap::ArgMatches;
use common::{
    commands::{Add, ApiCommand, Delete, Read, Update},
    item::{ItemName, Section},
    recipes::{Ingredients, RecipeName},
    ReadError,
};
use persistence::sqlite_db::{establish_connection, SqliteStore};
use thiserror::Error;

use crate::{cli, migrate_json_db::migrate_groceries, CliError};

#[derive(Error, Debug)]
pub enum GrusterylistError {
    #[error("Cli error: {0}")]
    CliError(#[from] CliError),

    #[error("Read error: {0}")]
    ReadError(#[from] ReadError),
}

pub fn run() -> Result<(), CliError> {
    let matches = cli().get_matches();

    let Some(val) = matches.get_one::<String>("db") else {
        unreachable!("'db' has a default setting")
    };

    let mut store = match val.as_str() {
        "sqlite" => SqliteStore::new(establish_connection()),
        "json" => unimplemented!(),
        _ => unreachable!(),
    };

    if let Some(("migrate-json-db", matches)) = matches.subcommand() {
        migrate_groceries(
            store.connection(),
            matches.get_one::<String>("path").unwrap().as_str(),
        )?;
    }

    let mut api = Api::new(store);

    let cmd = match matches.subcommand() {
        Some(("add", matches)) => ApiCommand::Add(add(matches)?),
        Some(("delete", matches)) => ApiCommand::Delete(delete(matches)),
        Some(("read", matches)) => ApiCommand::Read(read(matches)),
        Some(("update", matches)) => ApiCommand::Update(update(matches)),
        _ => unreachable!(),
    };

    api.execute(&cmd);

    Ok(())
}

fn add(matches: &ArgMatches) -> Result<Add, CliError> {
    if matches.contains_id("recipe") && matches.contains_id("ingredients") {
        Ok(Add::Recipe {
            recipe: RecipeName::from_str(matches.get_one::<String>("recipe").unwrap().as_str())
                .unwrap(),
            ingredients: Ingredients::try_from(
                matches.get_one::<String>("ingredients").unwrap().as_str(),
            )
            .unwrap(),
        })
    } else if matches.contains_id("item") {
        Ok(Add::Item {
            name: ItemName::from(matches.get_one::<String>("item").unwrap().as_str()),
            section: matches
                .get_one::<String>("section")
                .map(|section| Section::from(section.as_str())),
        })
    } else if matches.contains_id("checklist-item") {
        Ok(Add::ChecklistItem(ItemName::from(
            matches
                .get_one::<String>("checklist-item")
                .unwrap()
                .as_str(),
        )))
    } else {
        match matches.subcommand() {
            Some(("checklist", matches)) => Ok(Add::ChecklistItem(ItemName::from(
                matches.get_one::<String>("item").unwrap().as_str(),
            ))),
            Some(("list", matches)) => {
                if matches.contains_id("recipe") {
                    Ok(Add::ListRecipe(
                        RecipeName::from_str(matches.get_one::<String>("recipe").unwrap().as_str())
                            .unwrap(),
                    ))
                } else if matches.contains_id("item") {
                    Ok(Add::ListItem(ItemName::from(
                        matches.get_one::<String>("item").unwrap().as_str(),
                    )))
                } else {
                    Ok(Add::NewList)
                }
            }
            _ => unreachable!(),
        }
    }
}

fn delete(matches: &ArgMatches) -> Delete {
    if matches.contains_id("recipe") {
        Delete::Recipe(
            RecipeName::from_str(matches.get_one::<String>("recipe").unwrap().as_str()).unwrap(),
        )
    } else if matches.contains_id("item") {
        Delete::Item(ItemName::from(
            matches.get_one::<String>("recipe").unwrap().as_str(),
        ))
    } else {
        match matches.subcommand() {
            Some(("checklist", matches)) => {
                if matches.contains_id("checklist-item") {
                    Delete::ChecklistItem(ItemName::from(
                        matches
                            .get_one::<String>("checklist-item")
                            .unwrap()
                            .as_str(),
                    ))
                } else {
                    unimplemented!()
                }
            }
            _ => unimplemented!(),
        }
    }
}

fn read(matches: &ArgMatches) -> Read {
    if matches.contains_id("recipe") {
        Read::Recipe(
            RecipeName::from_str(matches.get_one::<String>("recipe").unwrap().as_str()).unwrap(),
        )
    } else if matches.contains_id("item") {
        Read::Item(ItemName::from(
            matches.get_one::<String>("item").unwrap().as_str(),
        ))
    } else {
        match matches.subcommand() {
            Some(("checklist", _matches)) => Read::Checklist,
            Some(("list", _matches)) => Read::List,
            Some(("library", _matches)) => Read::Items,
            Some(("recipes", _matches)) => Read::Recipes,
            Some(("sections", _matches)) => Read::Sections,
            _ => Read::All,
        }
    }
}

fn update(matches: &ArgMatches) -> Update {
    if matches.contains_id("recipe") {
        Update::Recipe(
            RecipeName::from_str(matches.get_one::<String>("recipe").unwrap().as_str()).unwrap(),
        )
    } else {
        unimplemented!()
    }
}
//     Some(("groceries", _)) => Ok(run_groceries::run()?),
//     Some(("list", _)) => Ok(run_shopping_list::run()?),
//     Some(("migrate-json-groceries-to-db", _)) => Ok(migrate_json_db::migrate_groceries()?),
//     Some(("recipes", sync_matches)) => Ok(run_recipes::run(sync_matches)?),
//     Some(("show-items-in-db", _)) => {
//         show::show_items();
//         Ok(())
//     }
//     Some(("show-list-sections", _)) => {
//         show::show_sections();
//         Ok(())
//     }
//     _ => unreachable!(),
// }
// }
