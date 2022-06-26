use crate::ReadError;
use crate::ShoppingList;
use clap::ArgMatches;
use std::path::Path;

pub fn run(sync_matches: &ArgMatches) -> Result<(), ReadError> {
    let path = sync_matches.get_one::<String>("path").unwrap();

    match sync_matches.subcommand() {
        Some(("print", _)) => {
            let list = ShoppingList::from_path(path)?;
            list.print();
        }
        _ => {
            if crate::Groceries::from_path("groceries.json").is_err() {
                return Err(ReadError::LibraryNotFound);
            } else {
                match sync_matches.subcommand() {
                    Some(("create", sync_matches)) => {
                        let mut sl = ShoppingList::new();
                        if !sync_matches.contains_id("fresh") {
                            // Some(("used-saved", _)) => {
                            if Path::new(path).exists() {
                                // let path = "list.json";
                                sl = ShoppingList::from_path(path)?;
                            }
                        }
                        sl.prompt_add_recipes()?;

                        sl.prompt_add_groceries()?;

                        sl.prompt_save_list()?;
                    }
                    _ => unreachable!(),
                }
            }
        }
    }
    Ok(())
}

impl ShoppingList {
    // pub(crate) fn prompt_view_list(&self) -> Result<(), ReadError> {
    //     if !self.groceries.is_empty() {
    //         eprintln!(
    //             "\n\
    //     Print shopping list?\n\
    //     *y*\n\
    //     *any other key* to continue"
    //         );
    //
    //         if crate::prompt_for_y()? {
    //             self.print();
    //             println!();
    //         }
    //     }
    //     Ok(())
    // }

    pub(crate) fn prompt_add_recipes(&mut self) -> Result<(), ReadError> {
        eprintln!(
            "Add recipe ingredients to our list?\n\
                *y*\n\
                *any other key* to continue"
        );

        while crate::prompt_for_y()? {
            let groceries = crate::Groceries::from_path("groceries.json")?;

            for recipe in groceries.recipes.into_iter() {
                eprintln!(
                    "Shall we add ...\n\
                            {}?\n\
                            *y*\n\
                            *s* to skip to end of recipes\n\
                            *any other key* for next recipe",
                    recipe
                );

                match crate::get_user_input()?.as_str() {
                    "y" => {
                        if !self.recipes.contains(&recipe) {
                            self.add_recipe(recipe);
                        }
                    }
                    "s" => break,
                    &_ => continue,
                }
            }
            eprintln!(
                "Add any more recipe ingredients to our list?\n\
                    *y*\n\
                    *any other key* to continue"
            );
        }
        Ok(())
    }

    pub(crate) fn prompt_add_groceries(&mut self) -> Result<(), ReadError> {
        eprintln!(
            "Add groceries to shopping list?\n\
            *y*\n\
            *any other key* to skip"
        );

        while crate::prompt_for_y()? {
            self.add_groceries()?;
            eprintln!(
                "Add more groceries to shopping list?\n\
            *y*\n\
            *any other key* to skip"
            );
        }
        Ok(())
    }

    pub(crate) fn prompt_save_list(&mut self) -> Result<(), ReadError> {
        // don't save list if empty
        if !self.checklist.is_empty() && !self.groceries.is_empty() && !self.recipes.is_empty() {
            eprintln!(
                "Save current list?\n\
                *y*\n\
                *any other key* to continue"
            );

            if crate::prompt_for_y()? {
                self.save()?;
            }

            self.print();
        }
        Ok(())
    }
}
