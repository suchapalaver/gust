use std::str::FromStr;

use crate::{ cli, CliError };
use api::{ Api, ApiError };
use clap::ArgMatches;
use common::{
    commands::{ Add, ApiCommand, Delete, Read, Update },
    item::{ Name, Section },
    recipes::{ Ingredients, Recipe },
};
use tracing::instrument;
use url::Url;

#[instrument]
pub async fn run() -> Result<(), CliError> {
    let matches = cli().get_matches();

    let api = Api::new(
        matches
            .get_one::<String>("store")
            .expect("'store' has a default setting")
            .parse()
            .map_err(ApiError::from)?
    ).await?;

    let api_dispatch = api.dispatch().await?;

    let command = match matches.subcommand() {
        Some(("add", matches)) => ApiCommand::Add(add(matches)?),
        Some(("delete", matches)) => ApiCommand::Delete(delete(matches)?),
        Some(("fetch", matches)) => fetch(matches)?,
        Some(("read", matches)) => ApiCommand::Read(read(matches)?),
        Some(("update", matches)) => ApiCommand::Update(update(matches)?),
        Some(("migrate-json-store", _)) => ApiCommand::MigrateJsonDbToSqlite,
        _ => unreachable!(),
    };

    let response = api_dispatch.dispatch(command).await?;

    println!("{response}");

    Ok(())
}

fn add(matches: &ArgMatches) -> Result<Add, CliError> {
    if
        let (Some(recipe), Some(ingredients)) = (
            matches.get_one::<String>("recipe"),
            matches.get_one::<String>("ingredients"),
        )
    {
        let (recipe, ingredients) = (
            Recipe::from_str(recipe.trim())?,
            Ingredients::from_input_string(ingredients.trim()),
        );

        Ok(Add::recipe_from_name_and_ingredients(recipe, ingredients))
    } else if let Some(name) = matches.get_one::<String>("item") {
        Ok(
            Add::item_from_name_and_section(
                Name::from(name.trim()),
                matches.get_one::<String>("section").map(|section| Section::from(section.trim()))
            )
        )
    } else if let Some(item) = matches.get_one::<String>("checklist-item") {
        Ok(Add::checklist_item_from_name(Name::from(item.trim())))
    } else {
        match matches.subcommand() {
            Some(("checklist", matches)) =>
                Ok(
                    Add::checklist_item_from_name(
                        Name::from(matches.get_one::<String>("item").expect("item required").trim())
                    )
                ),
            Some(("list", matches)) => {
                if let Some(name) = matches.get_one::<String>("recipe") {
                    Ok(Add::list_recipe_from_name(Recipe::from_str(name.trim())?))
                } else if let Some(name) = matches.get_one::<String>("item") {
                    Ok(Add::list_item_from_name(Name::from(name.trim())))
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
        Ok(Delete::recipe_from_name(Recipe::from_str(name.trim())?))
    } else if let Some(name) = matches.get_one::<String>("item") {
        Ok(Delete::item_from_name(Name::from(name.trim())))
    } else {
        match matches.subcommand() {
            Some(("checklist", matches)) => {
                let Some(name) = matches.get_one::<String>("checklist-item") else {
                    unimplemented!()
                };
                Ok(Delete::ChecklistItem(Name::from(name.trim())))
            }
            _ => unimplemented!(),
        }
    }
}

fn fetch(matches: &ArgMatches) -> Result<ApiCommand, url::ParseError> {
    let Some(url) = matches.get_one::<String>("url") else {
        unreachable!("Providing a URL is required")
    };
    let url = Url::parse(url.trim())?;
    Ok(ApiCommand::FetchRecipe(url))
}

fn read(matches: &ArgMatches) -> Result<Read, CliError> {
    if let Some(name) = matches.get_one::<String>("recipe") {
        Ok(Read::recipe_from_name(Recipe::from_str(name.trim())?))
    } else if let Some(name) = matches.get_one::<String>("item") {
        Ok(Read::item_from_name(Name::from(name.trim())))
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
    match matches.subcommand() {
        Some(("recipe", matches)) => {
            let Some(name) = matches.get_one::<String>("recipe") else { todo!() };
            Ok(Update::recipe_from_name(Recipe::from_str(name.trim())?))
        }
        Some(("list", matches)) => {
            let Some(("clear", _)) = matches.subcommand() else { unimplemented!() };
            Ok(Update::RefreshList)
        }
        _ => unimplemented!(),
    }
}
