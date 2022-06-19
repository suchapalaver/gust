// use crate::{Groceries, Ingredients};
use crate::ReadError;
// use clap::Arg;
use clap::Command;

fn cli() -> Command<'static> {
    Command::new("grusterylist")
        .about("Makes grocery lists")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("recipes")
                .about("Manages recipes library")
                // .subcommand(
                //     Command::new("add")
                //         .subcommand_required(false)
                //         .about("Adds recipes to library")
                //         .arg(
                //             Arg::with_name("name")
                //                 .long("name")
                //                 .takes_value(true)
                //                 .help("Provides name of recipe to be added")
                //         )
                //         .arg(
                //             Arg::with_name("ingredients")
                //                 .long("ingredients")
                //                 .takes_value(true)
                //                 .help("Provides name of recipe to be added")
                //         ),
                // )
                // .subcommand(
                //     Command::new("delete")
                //         .about("Deletes recipe from library")
                //         .arg(Arg::with_name("name").required(true)),
                // ),
        )
        .subcommand(
            Command::new("groceries")
                .about("Manages groceries library")
                .subcommand(Command::new("add").about("Adds grocery items to library")),
        )
        .subcommand(Command::new("list").about("Makes shopping lists"))
}

pub fn run() -> Result<(), ReadError> {
    let matches = cli().get_matches();

    match matches.subcommand_name() {
        Some("recipes") => {
            // if let Some(sub_m) = matches.subcommand_matches("recipes") {
            //     match sub_m.subcommand_name() {
            //         Some("add") => {
            //             let name: String = sub_m
            //                 // .get_one::<String>("name");
            //                 .get_one::<String>("name")
            //                 .expect("name is required")
            //                 .to_owned();
            //             let ingredients = sub_m
            //                 // .value_of("ingredients").unwrap();
            //                 .get_one::<String>("ingredients")
            //                 .expect("ingredients input required")
            //                 .to_owned();
            //             let mut g = Groceries::from_path("groceries.json")?;
            //             let name = crate::Recipe(name.to_string());
            //             let ingredients = Ingredients::from_input_string(ingredients.to_string())?;
            //             g.add_recipe(name, ingredients);
            //             Ok(())
            //         }
            //         // Some()
            //         _ => unreachable!(),
            //     }
            // } else {
                crate::run_recipes::run()?;
                Ok(())
            // }
        }
        Some("groceries") => Ok(crate::run_groceries::run()?),
        Some("list") => Ok(crate::run_shopping_list::run()?),
        _ => unreachable!(),
    }
}
