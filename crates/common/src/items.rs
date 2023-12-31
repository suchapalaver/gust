use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::{
    item::{Item, Name, Section},
    recipes::{Ingredients, Recipe},
    Load, ReadError,
};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Items {
    pub sections: Vec<Section>,
    pub collection: Vec<Item>,
    pub recipes: Vec<Recipe>,
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

    pub fn delete_item(&mut self, name: &str) -> Result<(), ReadError> {
        if let Ok(i) = self
            .collection
            .iter()
            .position(|x| x.name == Name::from(name))
            .ok_or(ReadError::ItemNotFound)
        {
            self.collection.remove(i);
        }
        Ok(())
    }

    pub fn items(&self) -> impl Iterator<Item = &Item> {
        self.sections
            .iter()
            .flat_map(|section| {
                self.collection.iter().filter(|item| {
                    let Some(item_section) = &item.section else {
                        return false;
                    };
                    item_section.as_str().contains(section.as_str())
                })
            })
            .collect::<Vec<_>>()
            .into_iter()
    }

    pub fn recipes(&self) -> impl Iterator<Item = &Recipe> {
        self.recipes.iter()
    }

    pub fn add_recipe(&mut self, name: &str, ingredients: &str) -> Result<(), ReadError> {
        let recipe = Recipe::from_str(name)?;

        let ingredients = Ingredients::from_input_string(ingredients);

        self.collection
            .iter_mut()
            .filter(|x| ingredients.contains(&x.name))
            .for_each(|x| match x.recipes.as_mut() {
                Some(recipes) => recipes.push(recipe.clone()),
                None => x.recipes = Some(vec![recipe.clone()]),
            });

        self.recipes.push(recipe);
        Ok(())
    }

    pub fn delete_recipe(&mut self, name: &str) -> Result<(), ReadError> {
        if let Ok(i) = self
            .recipes
            .iter()
            .position(|recipe| recipe.as_str() == name)
            .ok_or(ReadError::ItemNotFound)
        {
            self.recipes.remove(i);
        }
        for item in &mut self.collection {
            if let Some(recipes) = item.recipes.as_mut() {
                if let Some(i) = recipes.iter().position(|recipe| recipe.as_str() == name) {
                    recipes.remove(i);
                }
            }
        }
        Ok(())
    }

    pub fn recipe_ingredients(
        &self,
        recipe: &str,
    ) -> Result<impl Iterator<Item = &Item>, ReadError> {
        let recipe = Recipe::from_str(recipe)?;
        Ok(self
            .collection
            .iter()
            .filter(|item| {
                let Some(recipes) = &item.recipes else {
                    return false;
                };
                recipes.contains(&recipe)
            })
            .collect::<Vec<_>>()
            .into_iter())
    }
}
