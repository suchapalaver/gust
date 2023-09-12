use crate::{
    groceries::Groceries, groceriesitem::Item, input::user_wants_to_add_item_to_list,
    sections::SECTIONS, shoppinglist::ShoppingList, ReadError,
};
impl ShoppingList {
    pub fn add_groceries(&mut self, groceries: Groceries) -> Result<(), ReadError> {
        // move everything off list to temp list
        let list_items: Vec<Item> = self.items.drain(..).collect();
        assert!(self.items.is_empty());
        let sections = SECTIONS;
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
                        let res = user_wants_to_add_item_to_list(&groceriesitem);

                        match res {
                            Some(true) => {
                                if !self.items.contains(&groceriesitem) {
                                    self.add_groceries_item(groceriesitem.clone());
                                }
                            }
                            Some(false) => continue,
                            None => break,
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
