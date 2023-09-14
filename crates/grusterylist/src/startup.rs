use std::str::FromStr;

use crate::{cli, CliError};
use api::Api;
use clap::ArgMatches;
use common::{
    commands::{Add, ApiCommand, Delete, Read, Update},
    item::{ItemName, Section},
    recipes::{Ingredients, RecipeName},
};
use persistence::{
    json::{migrate::migrate_groceries, JsonStore},
    sqlite::{establish_connection, SqliteStore},
    store::Store,
};

pub fn run() -> Result<(), CliError> {
    let matches = cli().get_matches();

    let Some(val) = matches.get_one::<String>("db") else {
        unreachable!("'db' has a default setting")
    };

    let store = match val.as_str() {
        "sqlite" => {
            let mut store = SqliteStore::new(establish_connection());
            if let Some(("migrate-json-db", _)) = matches.subcommand() {
                migrate_groceries(&mut JsonStore::default(), store.connection())?;
                println!("Migration complete");
                return Ok(());
            }
            Store::from(store)
        }
        "json" => Store::from(JsonStore::default()),
        _ => unreachable!(),
    };

    let response = Api::new(store).execute(match matches.subcommand() {
        Some(("add", matches)) => ApiCommand::Add(add(matches)?),
        Some(("delete", matches)) => ApiCommand::Delete(delete(matches)?),
        Some(("read", matches)) => ApiCommand::Read(read(matches)?),
        Some(("update", matches)) => ApiCommand::Update(update(matches)?),
        _ => unreachable!(),
    })?;

    println!("{response}");

    Ok(())
}

fn add(matches: &ArgMatches) -> Result<Add, CliError> {
    if let (Some(recipe), Some(ingredients)) = (
        matches.get_one::<String>("recipe"),
        matches.get_one::<String>("ingredients"),
    ) {
        let (recipe, ingredients) = (
            RecipeName::from_str(recipe)?,
            Ingredients::from_input_string(ingredients),
        );

        Ok(Add::recipe_from_name_and_ingredients(recipe, ingredients))
    } else if let Some(name) = matches.get_one::<String>("item") {
        Ok(Add::item_from_name_and_section(
            ItemName::from(name.as_str()),
            matches
                .get_one::<String>("section")
                .map(|section| Section::from(section.as_str())),
        ))
    } else if let Some(item) = matches.get_one::<String>("checklist-item") {
        Ok(Add::checklist_item_from_name(ItemName::from(item.as_str())))
    } else {
        match matches.subcommand() {
            Some(("checklist", matches)) => Ok(Add::checklist_item_from_name(ItemName::from(
                matches.get_one::<String>("item").unwrap().as_str(),
            ))),
            Some(("list", matches)) => {
                if let Some(name) = matches.get_one::<String>("recipe") {
                    Ok(Add::list_recipe_from_name(RecipeName::from_str(name)?))
                } else if let Some(name) = matches.get_one::<String>("item") {
                    Ok(Add::list_item_from_name(ItemName::from(name.as_str())))
                } else {
                    Ok(Add::new_list())
                }
            }
            _ => unreachable!(),
        }
    }
}

fn delete(matches: &ArgMatches) -> Result<Delete, CliError> {
    if let Some(name) = matches.get_one::<String>("recipe") {
        Ok(Delete::recipe_from_name(RecipeName::from_str(
            name.as_str(),
        )?))
    } else if let Some(name) = matches.get_one::<String>("recipe") {
        Ok(Delete::item_from_name(ItemName::from(name.as_str())))
    } else {
        match matches.subcommand() {
            Some(("checklist", matches)) => {
                if let Some(name) = matches.get_one::<String>("checklist-item") {
                    Ok(Delete::ChecklistItem(ItemName::from(name.as_str())))
                } else {
                    unimplemented!()
                }
            }
            _ => unimplemented!(),
        }
    }
}

fn read(matches: &ArgMatches) -> Result<Read, CliError> {
    if let Some(name) = matches.get_one::<String>("recipe") {
        Ok(Read::recipe_from_name(RecipeName::from_str(name.as_str())?))
    } else if let Some(name) = matches.get_one::<String>("item") {
        Ok(Read::item_from_name(ItemName::from(name.as_str())))
    } else {
        match matches.subcommand() {
            Some(("checklist", _matches)) => Ok(Read::Checklist),
            Some(("list", _matches)) => Ok(Read::List),
            Some(("library", _matches)) => Ok(Read::All),
            Some(("recipes", _matches)) => Ok(Read::Recipes),
            Some(("sections", _matches)) => Ok(Read::Sections),
            _ => Ok(Read::All),
        }
    }
}

fn update(matches: &ArgMatches) -> Result<Update, CliError> {
    if let Some(name) = matches.get_one::<String>("recipe") {
        Ok(Update::recipe_from_name(RecipeName::from_str(
            name.as_str(),
        )?))
    } else {
        unimplemented!()
    }
}
