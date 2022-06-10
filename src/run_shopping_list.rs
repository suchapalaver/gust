use crate::GroceriesItem;
use crate::ReadError;
use crate::ShoppingList;

pub fn run() -> Result<(), ReadError> {
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
    let groceries = crate::Groceries::from_path("groceries.json")?;

    while crate::prompt_for_y()? {
        // move everything off list to temp list
        let list_items: Vec<GroceriesItem> = sl.groceries.drain(..).collect();
        assert!(sl.groceries.is_empty());
        let sections = vec!["fresh", "pantry", "dairy", "protein", "freezer"];
        let groceries_by_section: Vec<Vec<GroceriesItem>> = {
            sections
                .into_iter()
                .map(|section| {
                    let mut a: Vec<GroceriesItem> = list_items
                        .iter()
                        .filter(|groceriesitem| groceriesitem.section.0 == section)
                        .cloned()
                        .collect();

                    let b: Vec<GroceriesItem> = groceries
                        .collection
                        .iter()
                        .filter(|groceriesitem| {
                            groceriesitem.section.0 == section && !a.contains(groceriesitem)
                        })
                        .cloned()
                        .collect();
                    a.extend(b);
                    a
                })
                .collect()
        };
        for section in groceries_by_section {
            if !section.is_empty() {
                for groceriesitem in &section {
                    if !sl.groceries.contains(groceriesitem)
                        && groceriesitem
                            .recipes
                            .iter()
                            .any(|recipe| sl.recipes.contains(&*recipe))
                    {
                        sl.add_groceries_item(groceriesitem.clone());
                    }
                }
                for groceriesitem in section {
                    if !sl.groceries.contains(&groceriesitem) {
                        eprintln!(
                            "Do we need {}?\n\
                                *y*\n\
                                *any other key* for next item\n\
                                *s* for next section",
                            groceriesitem.name.0.to_lowercase()
                        );

                        match crate::get_user_input()?.as_str() {
                            "y" => {
                                if !sl.groceries.contains(&groceriesitem) {
                                    sl.add_groceries_item(groceriesitem.clone());
                                }
                            }
                            "s" => break,
                            &_ => continue,
                        }
                    }
                }
            }
        }
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

    Ok(())
}
