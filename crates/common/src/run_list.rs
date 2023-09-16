use crate::{
    input::user_wants_to_add_item_to_list, item::Item, items::Items, list::List,
    sections::SECTIONS, ReadError,
};
impl List {
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
                        .filter(|item| {
                            if let Some(item_sec) = &item.section {
                                item_sec.as_str() == section
                            } else {
                                false
                            }
                        })
                        .cloned()
                        .collect();

                    let b: Vec<Item> = groceries
                        .collection
                        .iter()
                        .filter(|item| {
                            if let Some(item_sec) = &item.section {
                                item_sec.as_str() == section && !a.contains(item)
                            } else {
                                false
                            }
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
                    if !self.items.contains(item) {
                        if let Some(recipes) = &item.recipes {
                            if recipes.iter().any(|recipe| self.recipes.contains(recipe)) {
                                self.add_item(item.clone());
                            }
                        }
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
