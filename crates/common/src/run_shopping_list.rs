use crate::{
    errors::ReadError,
    groceries::{Groceries, ITEMS_JSON_PATH},
    groceriesitem::Item,
    helpers::ReadWrite,
    sections::SECTIONS,
    shoppinglist::{ShoppingList, LIST_JSON_PATH},
};
use question::{Answer, Question};

impl ShoppingList {
    pub fn prompt_view_list(&self) -> Result<(), ReadError> {
        if !self.items.is_empty() {
            let res = Question::new("Print shopping list?")
                .default(question::Answer::NO)
                .show_defaults()
                .confirm();

            if res == Answer::YES {
                self.print();
                println!();
            }
        }
        Ok(())
    }

    pub fn prompt_add_recipes(&mut self) -> Result<(), ReadError> {
        while Question::new("Add more recipe ingredients to our list?")
            .default(question::Answer::NO)
            .show_defaults()
            .confirm()
            == Answer::YES
        {
            let groceries = Groceries::from_path(ITEMS_JSON_PATH)?;

            for recipe in groceries.recipes.into_iter() {
                let res = Question::new(&format!(
                    "Shall we add {}? (*y*, *n* for next recipe, *s* to skip to end of recipes)",
                    recipe
                ))
                .acceptable(vec!["y", "n", "s"])
                .until_acceptable()
                .default(Answer::RESPONSE("n".to_string()))
                .ask();

                match res {
                    Some(Answer::RESPONSE(res)) if &res == "y" => {
                        if !self.recipes.contains(&recipe) {
                            self.add_recipe(recipe);
                        }
                    }
                    Some(Answer::RESPONSE(res)) if &res == "s" => break,
                    _ => continue,
                }
            }
        }
        Ok(())
    }

    pub fn prompt_add_groceries(&mut self) -> Result<(), ReadError> {
        while Question::new("Add groceries to shopping list?")
            .default(question::Answer::NO)
            .show_defaults()
            .confirm()
            == Answer::YES
        {
            self.add_groceries()?;
        }
        Ok(())
    }

    pub fn add_groceries(&mut self) -> Result<(), ReadError> {
        // move everything off list to temp list
        let list_items: Vec<Item> = self.items.drain(..).collect();
        assert!(self.items.is_empty());
        let sections = SECTIONS;
        let groceries = Groceries::from_path(ITEMS_JSON_PATH)?;
        let groceries_by_section: Vec<Vec<Item>> = {
            sections
                .into_iter()
                .map(|section| {
                    let mut a: Vec<Item> = list_items
                        .iter()
                        .filter(|groceriesitem| groceriesitem.section.is_some())
                        .filter(|groceriesitem| {
                            groceriesitem.section.as_ref().unwrap().0 == section
                        })
                        .cloned()
                        .collect();

                    let b: Vec<Item> = groceries
                        .collection
                        .iter()
                        .filter(|groceriesitem| groceriesitem.section.is_some())
                        .filter(|groceriesitem| {
                            groceriesitem.section.as_ref().unwrap().0 == section
                                && !a.contains(groceriesitem)
                        })
                        .cloned()
                        .collect();
                    a.extend(b);
                    a
                })
                .collect()
        };
        for section in groceries_by_section {
            if !section.is_empty() {
                for groceriesitem in &section {
                    if !self.items.contains(groceriesitem)
                        && groceriesitem.recipes.is_some()
                        && groceriesitem
                            .recipes
                            .as_ref()
                            .unwrap()
                            .iter()
                            .any(|recipe| self.recipes.contains(recipe))
                    {
                        self.add_groceries_item(groceriesitem.clone());
                    }
                }
                for groceriesitem in section {
                    if !self.items.contains(&groceriesitem) {
                        let res = Question::new(&format!(
                            "Do we need {}? (*y*, *n* for next item, *s* to skip to next section)",
                            groceriesitem.name.0.to_lowercase()
                        ))
                        .acceptable(vec!["y", "n", "s"])
                        .until_acceptable()
                        .default(Answer::RESPONSE("n".to_string()))
                        .ask();

                        match res {
                            Some(Answer::RESPONSE(res)) if &res == "y" => {
                                if !self.items.contains(&groceriesitem) {
                                    self.add_groceries_item(groceriesitem.clone());
                                }
                            }
                            Some(Answer::RESPONSE(res)) if &res == "s" => break,
                            _ => continue,
                        }
                    }
                }
            }
        }
        Ok(())
    }

    pub fn prompt_save_list(&mut self) -> Result<(), ReadError> {
        // don't save list if empty
        if !self.checklist.is_empty() && !self.items.is_empty() && !self.recipes.is_empty() {
            let res = Question::new("Save current list?")
                .default(question::Answer::NO)
                .show_defaults()
                .confirm();

            if res == Answer::YES {
                self.save(LIST_JSON_PATH)?;
            }

            self.print();
        }
        Ok(())
    }
}
