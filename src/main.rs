use groceries::*;
use std::io::Result;

fn main() -> Result<()> {
    let greeting = "How are you?!\nLet's get the shopping done!";
    println!("{}", greeting);

    // ADD GROCERIES TO MASTER LIST
    add_to_master_groceries().unwrap();

    // ADD RECIPES TO RECIPES LIBRARY
    add_to_recipes_lib().unwrap();

    // ADD RECIPE INGREDIENTS TO LIST,
    // ADD TO SHOPPING LIST AND CHECKLIST,
    // OUTPUT
    // SAVE MOST RECENT LIST
    add_groceries(add_recipes(ShoppingList::get().unwrap()).unwrap()).unwrap();

    // ADD ANYTHING ELSE TO MASTER GROCERY AND RECIPE LISTS
    // ADD ANYTHING ELSE TO SHOPPINGLIST
    add_anything_else().unwrap();
    Ok(())
}
