use crate::{get_user_input, prompt_for_y, Groceries, Ingredients, Recipe};

pub fn run() -> Result<(), crate::ReadError> {
    eprintln!(
        "View the recipes we have \
	 in our library?\n\
	 --y\n\
	 --any other key to continue"
    );
    let path = "groceries.json";
    let mut groceries = Groceries::from_path(path)?;
    if prompt_for_y()? {
        eprintln!("Here are our recipes:");
        groceries.print_recipes();
        eprintln!();
    }
    eprintln!(
        "Add a recipe to our library?\n\
         --y\n\
         --any other key to continue"
    );
    if prompt_for_y()? {
        let recipe = {
            eprintln!("What's the name of the recipe?");

            let recipe_name = get_user_input()?;

            Recipe::new(recipe_name)?
        };

        eprintln!("Enter each ingredient separated by a comma");
        let ingredients_input = get_user_input()?;
        let ingredients = Ingredients::from_input_string(&ingredients_input)?;

        // 1st add new items to groceries
        for ingredient in ingredients.iter() {
            if groceries.collection.iter().all(|g| &g.name != ingredient) {
                let mut section_input_ok = false;
                let mut section_input = String::new();
                while !section_input_ok {
                    eprintln!(
                        "which section is {} in?\n\
                        *1* fresh
                        *2* pantry 
                        *3* protein 
                        *4* dairy 
                        *5* freezer",
                        ingredient
                    );

                    let input = crate::get_user_input()?;

                    section_input = match &input {
                        _ if input == "1" => {
                            section_input_ok = true;
                            "fresh".to_string()
                        }
                        _ if input == "2" => {
                            section_input_ok = true;
                            "pantry".to_string()
                        }
                        _ if input == "3" => {
                            section_input_ok = true;
                            "protein".to_string()
                        }
                        _ if input == "4" => {
                            section_input_ok = true;
                            "dairy".to_string()
                        }
                        _ if input == "5" => {
                            section_input_ok = true;
                            "freezer".to_string()
                        }
                        _ => {
                            eprintln!("re-enter section information");
                            continue;
                        }
                    };
                }
                let section = crate::GroceriesItemSection(section_input);

                let item = crate::GroceriesItem::new_initialized(ingredient.clone(), section);

                groceries.add_item(item);
            }
        }

        groceries.add_recipe(&recipe.0, &ingredients_input)?;
    }
    groceries.save("groceries.json")?;
    Ok(())
}
