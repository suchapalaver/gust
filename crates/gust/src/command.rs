use common::{
    commands::{Add, ApiCommand, Delete, Read, Update},
    item::Name,
    recipes::{Ingredients, Recipe},
    section::Section,
};

use clap::ArgMatches;
use url::Url;

use crate::CliError;

pub enum UserCommand {
    Add(Add),
    Delete(Delete),
    Export,
    FetchRecipe(Url),
    ImportFromJson,
    Read(Read),
    Update(Update),
}

impl TryFrom<ArgMatches> for UserCommand {
    type Error = CliError;

    fn try_from(matches: ArgMatches) -> Result<Self, Self::Error> {
        match matches.subcommand() {
            Some(("add", matches)) => Ok(UserCommand::Add(
                if let (Some(recipe), Some(ingredients)) = (
                    matches.get_one::<String>("recipe"),
                    matches.get_one::<String>("ingredients"),
                ) {
                    Add::recipe_from_name_and_ingredients(
                        Recipe::from_input_string(recipe),
                        Ingredients::from_input_string(ingredients),
                    )
                } else if let Some(name) = matches.get_one::<String>("item") {
                    Add::item_from_name_and_section(
                        Name::from(name.as_str()),
                        matches
                            .get_one::<String>("section")
                            .map(|section| Section::from(section.trim())),
                    )
                } else if let Some(item) = matches.get_one::<String>("checklist-item") {
                    Add::checklist_item_from_name(Name::from(item.as_str()))
                } else {
                    match matches.subcommand() {
                        Some(("checklist", matches)) => Add::checklist_item_from_name(Name::from(
                            matches
                                .get_one::<String>("item")
                                .expect("item required")
                                .as_str(),
                        )),
                        Some(("list", matches)) => {
                            if let Some(name) = matches.get_one::<String>("recipe") {
                                Add::list_recipe_from_name(name.as_str().into())
                            } else if let Some(name) = matches.get_one::<String>("item") {
                                Add::list_item_from_name(Name::from(name.as_str()))
                            } else {
                                unimplemented!()
                            }
                        }
                        _ => unreachable!(),
                    }
                },
            )),
            Some(("delete", matches)) => Ok(UserCommand::Delete(
                if let Some(name) = matches.get_one::<String>("recipe") {
                    Delete::recipe_from_name(name.as_str().into())
                } else if let Some(name) = matches.get_one::<String>("item") {
                    Delete::item_from_name(Name::from(name.as_str()))
                } else {
                    match matches.subcommand() {
                        Some(("checklist", matches)) => {
                            let Some(name) = matches.get_one::<String>("checklist-item") else {
                                unimplemented!()
                            };
                            Delete::ChecklistItem(Name::from(name.as_str()))
                        }
                        _ => unimplemented!(),
                    }
                },
            )),
            Some(("fetch", matches)) => {
                let Some(url) = matches.get_one::<String>("url") else {
                    unreachable!("Providing a URL is required")
                };
                let url: Url = Url::parse(url)?;
                Ok(UserCommand::FetchRecipe(url))
            }
            Some(("read", matches)) => Ok(UserCommand::Read(
                if let Some(name) = matches.get_one::<String>("recipe") {
                    Read::recipe_from_name(name.as_str().into())
                } else if let Some(name) = matches.get_one::<String>("item") {
                    Read::item_from_name(Name::from(name.as_str()))
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
            Some(("update", matches)) => Ok(UserCommand::Update(match matches.subcommand() {
                Some(("recipe", matches)) => {
                    let Some(name) = matches.get_one::<String>("recipe") else {
                        todo!()
                    };
                    Update::recipe_from_name(name.as_str().into())
                }
                Some(("list", matches)) => {
                    let Some(("clear", _)) = matches.subcommand() else {
                        unimplemented!()
                    };
                    Update::RefreshList
                }
                _ => unimplemented!(),
            })),
            Some(("import", _)) => Ok(UserCommand::ImportFromJson),
            Some(("export", _)) => Ok(UserCommand::Export),
            _ => unreachable!(),
        }
    }
}

impl From<UserCommand> for ApiCommand {
    fn from(command: UserCommand) -> Self {
        match command {
            UserCommand::Add(cmd) => Self::Add(cmd),
            UserCommand::Delete(cmd) => Self::Delete(cmd),
            UserCommand::Export => Self::Export,
            UserCommand::FetchRecipe(cmd) => Self::FetchRecipe(cmd),
            UserCommand::ImportFromJson => Self::ImportFromJson,
            UserCommand::Read(cmd) => Self::Read(cmd),
            UserCommand::Update(cmd) => Self::Update(cmd),
        }
    }
}
