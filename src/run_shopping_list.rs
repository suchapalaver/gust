use crate::ReadError;
use crate::ShoppingList;

pub fn run() -> Result<(), ReadError> {
    if let Err(e) = crate::Groceries::from_path("groceries.json") {
        eprintln!(
            "
            {e}\n\
            Run `grusterylist groceries` to create a groceries library
            "
                )
    } else {
        let mut sl = ShoppingList::new();
        eprintln!(
            "\n\
        Use most recently saved list?\n\
        *y*\n\
        *any other key* for fresh list"
        );
        if crate::prompt_for_y()? {
            let path = "list.json";
            sl = ShoppingList::from_path(path)?;
        }

        // view list if using saved list
        if !sl.groceries.is_empty() {
            eprintln!(
                "\n\
            Print shopping list?\n\
            *y*\n\
            *any other key* to continue"
            );

            if crate::prompt_for_y()? {
                sl.print();
                println!();
            }
        }

        // add recipes to shoppinglist
        eprintln!(
            "Add recipe ingredients to our list?\n\
                *y*\n\
                *any other key* to continue"
        );
        while crate::prompt_for_y()? {
            let path = "groceries.json";

            let groceries = crate::Groceries::from_path(path)?;
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
                        if !sl.recipes.contains(&recipe) {
                        sl.add_recipe(recipe);
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
    eprintln!(
        "Add groceries to shopping list?\n\
            *y*\n\
            *any other key* to skip"
    );
    

        while crate::prompt_for_y()? {
            sl.add_groceries()?;
            eprintln!(
                "Add more groceries to shopping list?\n\
            *y*\n\
            *any other key* to skip"
            );
        }

        // overwrite saved list with current list
        eprintln!(
            "Save current list?\n\
                *y*\n\
                *any other key* to continue"
        );

        if crate::prompt_for_y()? {
            sl.save()?;
        }

    sl.print();
    }
    Ok(())
}
