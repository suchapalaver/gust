use serde::{Deserialize, Serialize};

use crate::{
    item::{Item, Section},
    recipes::{Ingredients, Recipe},
    Load,
};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Items {
    sections: Vec<Section>,
    collection: Vec<Item>,
    recipes: Vec<Recipe>,
}

impl Load for Items {
    type T = Items;
}

impl FromIterator<Item> for Items {
    fn from_iter<I: IntoIterator<Item = Item>>(iter: I) -> Self {
        let mut c = Items::new();

        for i in iter {
            c.add_item(i);
        }
        c
    }
}

impl Items {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn collection(&self) -> impl Iterator<Item = &Item> {
        self.collection.iter()
    }

    pub fn get_item_matches(&self, name: &str) -> impl Iterator<Item = &Item> {
        self.collection
            .iter()
            .filter(|item| item.matches(name))
            .collect::<Vec<_>>()
            .into_iter()
    }

    pub fn add_item(&mut self, item: Item) {
        self.collection.push(item);
    }

    pub fn delete_item(&mut self, name: &str) {
        self.collection = self
            .collection
            .drain(..)
            .filter(|item| item.name().as_str() != name)
            .collect();
    }

    pub fn items(&self) -> impl Iterator<Item = &Item> {
        self.sections
            .iter()
            .flat_map(|section| {
                self.collection.iter().filter(|item| {
                    item.section()
                        .map_or(false, |item_section| item_section.contains(section))
                })
            })
            .collect::<Vec<_>>()
            .into_iter()
    }

    pub fn recipes(&self) -> impl Iterator<Item = &Recipe> {
        self.recipes.iter()
    }

    pub fn add_recipe(&mut self, name: &str, ingredients: &str) {
        self.collection
            .iter_mut()
            .filter(|item| Ingredients::from_input_string(ingredients).contains(item.name()))
            .for_each(|item| item.add_recipe(name));

        self.recipes.push(name.into());
    }

    pub fn delete_recipe(&mut self, name: &str) {
        self.recipes = self
            .recipes
            .drain(..)
            .filter(|recipe| recipe.as_str() != name)
            .collect();

        for item in &mut self.collection {
            item.delete_recipe(name);
        }
    }

    pub fn recipe_ingredients(&self, recipe: &Recipe) -> impl Iterator<Item = &Item> {
        self.collection
            .iter()
            .filter(|item| {
                item.recipes()
                    .map_or(false, |recipes| recipes.contains(recipe))
            })
            .collect::<Vec<_>>()
            .into_iter()
    }
}
