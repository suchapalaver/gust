use common::{errors::ReadError, groceries::Groceries, groceriesitem::Item, helpers::ReadWrite};
use question::{Answer, Question};

fn view_groceries() -> Result<(), ReadError> {
    let path = "groceries.json";

    for item in Groceries::from_path(path)?.items() {
        eprintln!();
        eprintln!("{}", item);
        eprintln!();
    }
    Ok(())
}

fn prompt_add_groceries() -> Result<(), ReadError> {
    while Question::new("Add an item to our library?")
        .default(question::Answer::NO)
        .show_defaults()
        .confirm()
        == Answer::YES
    {
        add_grocery_item()?
    }
    Ok(())
}

fn prompt_for_item() -> String {
    let ans = Question::new(
        "What is the item?\n\
        e.g. 'bread'",
    )
    .ask();

    if let Some(Answer::RESPONSE(ans)) = ans {
        ans
    } else {
        prompt_for_item()
    }
}

fn prompt_for_section() -> String {
    let ans = Question::new(
        "What is the section?\n\
            e.g. 'bread'",
    )
    .acceptable(vec!["fresh", "pantry", "protein", "dairy", "freezer"])
    .until_acceptable()
    .ask();

    if let Some(Answer::RESPONSE(ans)) = ans {
        ans
    } else {
        prompt_for_section()
    }
}

fn add_grocery_item() -> Result<(), ReadError> {
    let item = prompt_for_item();

    let section = prompt_for_section();

    let mut groceries = if Groceries::from_path("groceries.json").is_err() {
        Groceries::default()
    } else {
        Groceries::from_path("groceries.json")?
    };

    let mut present = false;

    for item in groceries.get_item_matches(&item) {
        let ans = Question::new(&format!("is *{}* a match?", item))
            .default(question::Answer::NO)
            .show_defaults()
            .confirm();

        if ans == Answer::YES {
            present = true;
            break;
        }
    }

    if present {
        eprintln!("Item already in library");
    } else {
        let new_item = Item::new(&item, &section);
        groceries.add_item(new_item);
    }
    Ok(())
}

fn prompt_save() -> Result<(), ReadError> {
    let path = "groceries.json";
    let groceries = Groceries::from_path(path).unwrap_or_default();
    groceries.save(path)?;
    Ok(())
}
