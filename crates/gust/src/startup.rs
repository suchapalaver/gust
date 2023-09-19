use std::str::FromStr;

use crate::{cli, CliError};
use api::Api;
use clap::ArgMatches;
use common::{
    commands::{Add, ApiCommand, Delete, Read, Update},
    item::{ItemName, Section},
    recipes::{Ingredients, Recipe},
};
use url::Url;

pub async fn run() -> Result<(), CliError> {
    let matches = cli().get_matches();

    let response = Api::new(
        matches
            .get_one::<String>("store")
            .expect("'store' has a default setting")
            .as_str(),
    )?
    .execute(match matches.subcommand() {
        Some(("add", matches)) => ApiCommand::Add(add(matches)?),
        Some(("delete", matches)) => ApiCommand::Delete(delete(matches)?),
        Some(("fetch", matches)) => fetch(matches)?,
        Some(("read", matches)) => ApiCommand::Read(read(matches)?),
        Some(("update", matches)) => ApiCommand::Update(update(matches)?),
        Some(("migrate-json-store", _)) => ApiCommand::MigrateJsonDbToSqlite,
        _ => unreachable!(),
    })
    .await?;

    println!("{response}");

    Ok(())
}

fn add(matches: &ArgMatches) -> Result<Add, CliError> {
    if let (Some(recipe), Some(ingredients)) = (
        matches.get_one::<String>("recipe"),
        matches.get_one::<String>("ingredients"),
    ) {
        let (recipe, ingredients) = (
            Recipe::from_str(recipe)?,
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
                matches
                    .get_one::<String>("item")
                    .expect("item required")
                    .as_str(),
            ))),
            Some(("list", matches)) => {
                if let Some(name) = matches.get_one::<String>("recipe") {
                    Ok(Add::list_recipe_from_name(Recipe::from_str(name)?))
                } else if let Some(name) = matches.get_one::<String>("item") {
                    Ok(Add::list_item_from_name(ItemName::from(name.as_str())))
                } else {
                    unimplemented!()
                }
            }
            _ => unreachable!(),
        }
    }
}

fn delete(matches: &ArgMatches) -> Result<Delete, CliError> {
    if let Some(name) = matches.get_one::<String>("recipe") {
        Ok(Delete::recipe_from_name(Recipe::from_str(name.as_str())?))
    } else if let Some(name) = matches.get_one::<String>("item") {
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

fn fetch(matches: &ArgMatches) -> Result<ApiCommand, url::ParseError> {
    let Some(url) = matches.get_one::<String>("url") else {
        unreachable!("Providing a URL is required")
    };
    let url = Url::parse(url)?;
    Ok(ApiCommand::FetchRecipe(url))
}

fn read(matches: &ArgMatches) -> Result<Read, CliError> {
    if let Some(name) = matches.get_one::<String>("recipe") {
        Ok(Read::recipe_from_name(Recipe::from_str(name.as_str())?))
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
    if let Some(("recipe", matches)) = matches.subcommand() {
        if let Some(name) = matches.get_one::<String>("recipe") {
            Ok(Update::recipe_from_name(Recipe::from_str(name.as_str())?))
        } else {
            todo!()
        }
    } else if let Some(("list", matches)) = matches.subcommand() {
        if let Some(("clear", _)) = matches.subcommand() {
            Ok(Update::RefreshList)
        } else {
            unimplemented!()
        }
    } else {
        unimplemented!()
    }
}
