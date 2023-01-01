use question::Answer;
use question::Question;

use crate::Groceries;
use crate::GroceriesItem;
use crate::ReadError;

pub fn run() -> Result<(), ReadError> {
    Groceries::prompt_view_groceries()?;
    Groceries::prompt_add_groceries()?;
    Groceries::prompt_save()?;
    Ok(())
}

impl Groceries {
    pub(crate) fn prompt_view_groceries() -> Result<(), ReadError> {
        while Question::new("View the groceries in our library?")
            .default(question::Answer::NO)
            .show_defaults()
            .confirm()
            == Answer::YES
        {
            Self::view_groceries()?
        }
        Ok(())
    }

    pub(crate) fn view_groceries() -> Result<(), ReadError> {
        let path = "groceries.json";

        for item in Self::from_path(path)?.items() {
            eprintln!();
            eprintln!("{}", item);
            eprintln!();
        }
        Ok(())
    }

    pub(crate) fn prompt_add_groceries() -> Result<(), ReadError> {
        while Question::new("Add an item to our library?")
            .default(question::Answer::NO)
            .show_defaults()
            .confirm()
            == Answer::YES
        {
            Self::add_grocery_item()?
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
            Self::prompt_for_item()
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
            Self::prompt_for_section()
        }
    }

    fn add_grocery_item() -> Result<(), ReadError> {
        let item = Self::prompt_for_item();

        let section = Self::prompt_for_section();

        let mut groceries = if Self::from_path("groceries.json").is_err() {
            Self::new_initialized()?
        } else {
            Self::from_path("groceries.json")?
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
            let new_item = GroceriesItem::new(&item, &section);
            groceries.add_item(new_item);
        }
        Ok(())
    }

    pub(crate) fn prompt_save() -> Result<(), ReadError> {
        let path = "groceries.json";
        let groceries = if Self::from_path(path).is_err() {
            Self::new_initialized()?
        } else {
            Self::from_path(path)?
        };
        groceries.save(path)?;
        Ok(())
    }
}
