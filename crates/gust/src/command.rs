use std::str::FromStr;

use common::{
    commands::{Add, ApiCommand, Delete, Read, Update},
    item::{Name, Section},
    recipes::{Ingredients, Recipe},
};

use clap::ArgMatches;
use url::Url;

use crate::CliError;

pub enum GustCommand {
    Add(Add),
    Delete(Delete),
    FetchRecipe(Url),
    MigrateJsonDbToSqlite,
    Read(Read),
    Update(Update),
}

impl TryFrom<ArgMatches> for GustCommand {
    type Error = CliError;

    fn try_from(matches: ArgMatches) -> Result<Self, Self::Error> {
        match matches.subcommand() {
            Some(("add", matches)) => Ok(GustCommand::Add(
                if let (Some(recipe), Some(ingredients)) = (
                    matches.get_one::<String>("recipe"),
                    matches.get_one::<String>("ingredients"),
                ) {
                    let (recipe, ingredients) = (
                        Recipe::from_str(recipe.trim())?,
                        Ingredients::from_input_string(ingredients.trim()),
                    );

                    Add::recipe_from_name_and_ingredients(recipe, ingredients)
                } else if let Some(name) = matches.get_one::<String>("item") {
                    Add::item_from_name_and_section(
                        Name::from(name.trim()),
                        matches
                            .get_one::<String>("section")
                            .map(|section| Section::from(section.trim())),
                    )
                } else if let Some(item) = matches.get_one::<String>("checklist-item") {
                    Add::checklist_item_from_name(Name::from(item.trim()))
                } else {
                    match matches.subcommand() {
                        Some(("checklist", matches)) => Add::checklist_item_from_name(Name::from(
                            matches
                                .get_one::<String>("item")
                                .expect("item required")
                                .trim(),
                        )),
                        Some(("list", matches)) => {
                            if let Some(name) = matches.get_one::<String>("recipe") {
                                Add::list_recipe_from_name(Recipe::from_str(name.trim())?)
                            } else if let Some(name) = matches.get_one::<String>("item") {
                                Add::list_item_from_name(Name::from(name.trim()))
                            } else {
                                unimplemented!()
                            }
                        }
                        _ => unreachable!(),
                    }
                },
            )),
            Some(("delete", matches)) => Ok(GustCommand::Delete(
                if let Some(name) = matches.get_one::<String>("recipe") {
                    Delete::recipe_from_name(Recipe::from_str(name.trim())?)
                } else if let Some(name) = matches.get_one::<String>("item") {
                    Delete::item_from_name(Name::from(name.trim()))
                } else {
                    match matches.subcommand() {
                        Some(("checklist", matches)) => {
                            let Some(name) = matches.get_one::<String>("checklist-item") else {
                                unimplemented!()
                            };
                            Delete::ChecklistItem(Name::from(name.trim()))
                        }
                        _ => unimplemented!(),
                    }
                },
            )),
            Some(("fetch", matches)) => {
                let Some(url) = matches.get_one::<String>("url") else {
                    unreachable!("Providing a URL is required")
                };
                let url: Url = Url::parse(url.trim())?;
                Ok(GustCommand::FetchRecipe(url))
            }
            Some(("read", matches)) => Ok(GustCommand::Read(
                if let Some(name) = matches.get_one::<String>("recipe") {
                    Read::recipe_from_name(Recipe::from_str(name.trim())?)
                } else if let Some(name) = matches.get_one::<String>("item") {
                    Read::item_from_name(Name::from(name.trim()))
                } else {
                    match matches.subcommand() {
                        Some(("checklist", _matches)) => Read::Checklist,
                        Some(("list", _matches)) => Read::List,
                        Some(("library", _matches)) => Read::All,
                        Some(("recipes", _matches)) => Read::Recipes,
                        Some(("sections", _matches)) => Read::Sections,
                        _ => Read::All,
                    }
                },
            )),
            Some(("update", matches)) => Ok(GustCommand::Update(match matches.subcommand() {
                Some(("recipe", matches)) => {
                    let Some(name) = matches.get_one::<String>("recipe") else {
                        todo!()
                    };
                    Update::recipe_from_name(Recipe::from_str(name.trim())?)
                }
                Some(("list", matches)) => {
                    let Some(("clear", _)) = matches.subcommand() else {
                        unimplemented!()
                    };
                    Update::RefreshList
                }
                _ => unimplemented!(),
            })),
            Some(("migrate-json-store", _)) => Ok(GustCommand::MigrateJsonDbToSqlite),
            _ => unreachable!(),
        }
    }
}

impl From<GustCommand> for ApiCommand {
    fn from(command: GustCommand) -> Self {
        match command {
            GustCommand::Add(cmd) => Self::Add(cmd),
            GustCommand::Delete(cmd) => Self::Delete(cmd),
            GustCommand::FetchRecipe(cmd) => Self::FetchRecipe(cmd),
            GustCommand::MigrateJsonDbToSqlite => Self::MigrateJsonDbToSqlite,
            GustCommand::Read(cmd) => Self::Read(cmd),
            GustCommand::Update(cmd) => Self::Update(cmd),
        }
    }
}
