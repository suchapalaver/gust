use crate::{item::Item, recipes::Recipe, sections::SECTIONS};
use question::{Answer, Question};

pub fn user_wants_to_add_item() -> bool {
    Question::new("Add an item to our library?")
        .default(question::Answer::NO)
        .show_defaults()
        .confirm()
        == Answer::YES
}

pub fn user_wants_to_print_list() -> bool {
    Question::new("Print shopping list?")
        .default(question::Answer::NO)
        .show_defaults()
        .confirm()
        == Answer::YES
}

pub fn user_wants_to_add_more_recipe_ingredients_to_list() -> bool {
    Question::new("Add more recipe ingredients to our list?")
        .default(question::Answer::NO)
        .show_defaults()
        .confirm()
        == Answer::YES
}

pub fn user_wants_to_add_items_to_list() -> bool {
    Question::new("Add items to list?")
        .default(question::Answer::NO)
        .show_defaults()
        .confirm()
        == Answer::YES
}

// Returns `None` in case user wishes to skip being asked further.
pub fn user_wants_to_add_item_to_list(item: &Item) -> Option<bool> {
    let res = Question::new(&format!(
        "Do we need {}? (*y*, *n* for next item, *s* to skip to next section)",
        item.name.as_str().to_lowercase()
    ))
    .acceptable(vec!["y", "n", "s"])
    .until_acceptable()
    .default(Answer::RESPONSE("n".to_string()))
    .ask();

    match res {
        Some(Answer::RESPONSE(res)) if &res == "y" => Some(true),
        Some(Answer::RESPONSE(res)) if &res == "s" => None,
        _ => Some(false),
    }
}

pub fn user_wants_to_save_list() -> bool {
    Question::new("Save current list?")
        .default(question::Answer::NO)
        .show_defaults()
        .confirm()
        == Answer::YES
}

// Returns `None` in case user wishes to skip being asked further.
pub fn user_wants_to_add_recipe_to_list(recipe: &Recipe) -> Option<bool> {
    let res = Question::new(&format!(
        "Shall we add {}? (*y*, *n* for next recipe, *s* to skip to end of recipes)",
        recipe
    ))
    .acceptable(vec!["y", "n", "s"])
    .until_acceptable()
    .default(Answer::RESPONSE("n".to_string()))
    .ask();

    match res {
        Some(Answer::RESPONSE(res)) if &res == "y" => Some(true),
        Some(Answer::RESPONSE(res)) if &res == "s" => None,
        _ => Some(false),
    }
}

pub fn item_from_user() -> String {
    let ans = Question::new(
        "What is the item?\n\
        e.g. 'bread'",
    )
    .ask();

    if let Some(Answer::RESPONSE(ans)) = ans {
        ans
    } else {
        item_from_user()
    }
}

pub fn section_from_user() -> String {
    if let Some(Answer::RESPONSE(ans)) = Question::new(
        "What is the section?\n\
            e.g. 'bread'",
    )
    .acceptable(SECTIONS.to_vec())
    .until_acceptable()
    .ask()
    {
        ans
    } else {
        section_from_user()
    }
}

pub fn item_matches(item: &Item) -> bool {
    Question::new(&format!("is *{}* a match?", item))
        .default(question::Answer::NO)
        .show_defaults()
        .confirm()
        == Answer::YES
}
