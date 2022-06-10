use crate::{input, print_recipes, prompt_for_y};

pub fn run_recipes() -> Result<(), crate::ReadError> {
    eprintln!(
        "View the recipes we have \
	 in our library?\n\
	 --y\n\
	 --any other key to continue"
    );

    if prompt_for_y()? {
        eprintln!("Here are our recipes:");

        let _ = print_recipes()?;
    }

    eprintln!(
        "Add a recipe to our library?\n\
         --y\n\
         --any other key to continue"
    );

    if prompt_for_y()? {
        eprintln!("What's the name of the recipe?");

        let recipe_name = input()?;

        eprintln!("Enter each ingredient separated by a comma");

        let ingredients = input()?;

        let mut groceries = crate::Groceries::from_path("groceries.json")?;

        groceries.add_recipe(recipe_name, ingredients)?;

        let s = serde_json::to_string(&groceries)?;

        groceries.save("groceries.json", &s)?;
    }

    Ok(())
}
