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
        eprintln!(
            "View the groceries in our library?\n\
             --y\n\
             --any other key to continue"
        );

        while crate::prompt_for_y()? {
            let path = "groceries.json";
            if let groceries = Groceries::from_path(path)? {
                eprintln!();
                groceries.print_groceries();
                eprintln!();
                eprintln!(
                    "View the groceries in our library?\n\
                        --y\n\
                        --any other key to continue"
                );
            }
        }
        Ok(())
    }

    pub(crate) fn prompt_add_groceries() -> Result<(), ReadError> {
        eprintln!(
            "Add groceries to our library?\n\
            --y\n\
            --any other key to exit"
        );

        while crate::prompt_for_y()? {
            Groceries::add_grocery_item()?;

            eprintln!(
                "Add more groceries to our library?\n\
            --y\n\
            --any other key to exit"
            );
        }
        Ok(())
    }

    fn add_grocery_item() -> Result<(), ReadError> {
        eprintln!(
            "Enter the item\n\
            e.g. 'bread'"
        );
        let name = crate::get_user_input()?;
        eprintln!(
            "Enter the section (fresh, pantry, protein, dairy, freezer)\n\
            e.g. 'fresh'"
        );
        let section = crate::get_user_input()?;

        let mut groceries = if Groceries::from_path("groceries.json").is_err() {
            Groceries::new_initialized("groceries.json")?
        } else {
            Groceries::from_path("groceries.json")?
        };

        if groceries.get_item(&name) != None {
            eprintln!("Item already in library");
        } else {
            let new_item = GroceriesItem::new(&name, &section);
            groceries.add_item(new_item);
        }
        Ok(())
    }

    pub(crate) fn prompt_save() -> Result<(), ReadError> {
        let path = "groceries.json";
        let groceries = if Groceries::from_path(path).is_err() {
            Groceries::new_initialized(path)?
        } else {
            Groceries::from_path(path)?
        };
        groceries.save(path)?;
        Ok(())
    }
}
