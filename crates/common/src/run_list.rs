use crate::{
    input::user_wants_to_add_item_to_list, item::Item, items::Items, list::ShoppingList,
    sections::SECTIONS, ReadError,
};
impl ShoppingList {
    pub fn add_groceries(&mut self, groceries: Items) -> Result<(), ReadError> {
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
                        .filter(|item| item.section.is_some())
                        .filter(|item| item.section.as_ref().unwrap().as_str() == section)
                        .cloned()
                        .collect();

                    let b: Vec<Item> = groceries
                        .collection
                        .iter()
                        .filter(|item| item.section.is_some())
                        .filter(|item| {
                            item.section.as_ref().unwrap().as_str() == section && !a.contains(item)
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
                for item in &section {
                    if !self.items.contains(item)
                        && item.recipes.is_some()
                        && item
                            .recipes
                            .as_ref()
                            .unwrap()
                            .iter()
                            .any(|recipe| self.recipes.contains(recipe))
                    {
                        self.add_item(item.clone());
                    }
                }
                for item in section {
                    if !self.items.contains(&item) {
                        let res = user_wants_to_add_item_to_list(&item);

                        match res {
                            Some(true) => {
                                if !self.items.contains(&item) {
                                    self.add_item(item.clone());
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
