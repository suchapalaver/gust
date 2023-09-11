use crate::{groceriesitem::Item, sections::SECTIONS};
use question::{Answer, Question};

pub fn user_wants_to_add_item() -> bool {
    Question::new("Add an item to our library?")
        .default(question::Answer::NO)
        .show_defaults()
        .confirm()
        == Answer::YES
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
