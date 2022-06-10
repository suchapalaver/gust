use crate::{
    get_user_input,
    groceries::{Groceries, Ingredients, Recipe},
    prompt_for_y,
};

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

        let ingredients = get_user_input()?;

        groceries.add_recipe(recipe, Ingredients::from_input_string(ingredients)?)?;
    }
    groceries.save("groceries.json")?;
    Ok(())
}
