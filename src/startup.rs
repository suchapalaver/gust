use clap::ArgMatches;
use tracing::info;

use crate::{
    cli,
    commands::{Add, ApiCommand},
    CliError, Ingredients, ItemName, RecipeName, Section,
};

pub fn run() -> Result<(), CliError> {
    let matches = cli().get_matches();

    let cmd = match matches.subcommand() {
        Some(("add", matches)) => add(matches)?,
        Some(("delete", matches)) => delete(matches)?,
        Some(("read", matches)) => read(matches)?,
        Some(("update", matches)) => update(matches)?,
        _ => unreachable!(),
    };

    Ok(())
}

fn add(matches: &ArgMatches) -> Result<ApiCommand, CliError> {
    if let Some(arg) = matches.get_one::<String>("checklist-item") {
        Ok(ApiCommand::Add(Add::ChecklistItem(ItemName::from(
            arg.as_str(),
        ))))
    } else if let Some(matches) = matches.subcommand_matches("item") {
        let name_elems: Vec<_> = matches
            .values_of("name")
            .expect("name is required")
            .collect();

        let name = ItemName::from(name_elems.join(" ").as_str());

        info!("item: {name}");

        let section: Vec<_> = matches
            .values_of("section")
            .expect("section name required")
            .collect();

        let section = Section(section.join(" "));

        info!("section: {section}");

        Ok(ApiCommand::Add(Add::Item {
            name,
            section: Some(section),
        }))
    } else if let Some(arg) = matches.get_one::<String>("list-item") {
        Ok(ApiCommand::Add(Add::ListItem(ItemName::from(arg.as_str()))))
    } else if let Some(matches) = matches.subcommand_matches("recipe") {
        let name_elems: Vec<_> = matches
            .values_of("name")
            .expect("name is required")
            .collect();

        let recipe = RecipeName(name_elems.join(" "));

        info!("recipe: {recipe}");

        let ingredient_vec: Vec<_> = matches
            .values_of("ingredients")
            .expect("ingredients required")
            .collect();

        let ingredients = Ingredients::from_input_string(ingredient_vec.join(", ").as_str())?;

        info!("ingredients: {ingredients:?}");

        Ok(ApiCommand::Add(Add::Recipe {
            recipe,
            ingredients,
        }))
    } else {
        unreachable!()
    }
}

fn delete(matches: &ArgMatches) -> Result<ApiCommand, CliError> {
    todo!()
}

fn read(matches: &ArgMatches) -> Result<ApiCommand, CliError> {
    todo!()
}

fn update(matches: &ArgMatches) -> Result<ApiCommand, CliError> {
    todo!()
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
