use crate::item::Item;
use question::{Answer, Question};

// Returns `None` in case user wishes to skip being asked further.
pub fn user_wants_to_add_item_to_list(item: &Item) -> Option<bool> {
    let res = Question::new(&format!(
        "Do we need {}? (*y*, *n* for next item, *s* to skip to next section)",
        item.name()
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

pub fn item_matches(item: &Item) -> Answer {
    Question::new(&format!("is *{item}* a match?"))
        .default(question::Answer::NO)
        .show_defaults()
        .confirm()
}
