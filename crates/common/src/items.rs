use std::str::FromStr;

use question::{Answer, Question};
use serde::{Deserialize, Serialize};

use crate::{
    item::{Item, ItemName, Section},
    recipes::{Ingredients, RecipeName},
    Load, ReadError,
};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Items {
    pub sections: Vec<Section>,
    pub collection: Vec<Item>,
    pub recipes: Vec<RecipeName>,
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
            .position(|x| x.name == ItemName::from(name))
            .ok_or(ReadError::ItemNotFound)
        {
            self.collection.remove(i);
        }
        Ok(())
    }

    pub fn items(&self) -> impl Iterator<Item = &Item> {
        self.sections
            .iter()
            .flat_map(|sec| {
                self.collection
                    .iter()
                    .filter(|x| x.section.is_some())
                    .filter(|x| x.section.as_ref().unwrap().as_str().contains(sec.as_str()))
            })
            .collect::<Vec<_>>()
            .into_iter()
    }

    pub fn recipes(&self) -> impl Iterator<Item = &RecipeName> {
        self.recipes.iter()
    }

    // check if ingredients already in lib or add them if not
    pub fn check_recipe_ingredients(&mut self, ingredients: &str) {
        let ingredients = Ingredients::from_input_string(ingredients);
        // add new items to groceries
        for ingredient in ingredients.iter() {
            if self.collection.iter().all(|item| &item.name != ingredient) {
                let res = Question::new(&format!(
                    "Which section is {ingredient} in?\n\
                    *1* fresh
                    *2* pantry 
                    *3* protein 
                    *4* dairy 
                    *5* freezer"
                ))
                .acceptable(vec!["1", "2", "3", "4", "5"])
                .until_acceptable()
                .ask();

                let section_input = match res {
                    Some(Answer::RESPONSE(res)) if &res == "1" => "fresh".to_string(),
                    Some(Answer::RESPONSE(res)) if &res == "2" => "pantry".to_string(),
                    Some(Answer::RESPONSE(res)) if &res == "3" => "protein".to_string(),
                    Some(Answer::RESPONSE(res)) if &res == "4" => "dairy".to_string(),
                    Some(Answer::RESPONSE(res)) if &res == "5" => "freezer".to_string(),
                    _ => unreachable!(),
                };

                let section = Section::from(section_input.as_str());

                let item = Item::new(ingredient.as_str()).with_section(section.as_str());

                self.add_item(item);
            }
        }
    }

    pub fn add_recipe(&mut self, name: &str, ingredients: &str) {
        let recipe = RecipeName::from_str(name).unwrap();

        let ingredients = Ingredients::from_input_string(ingredients);

        self.collection
            .iter_mut()
            .filter(|x| ingredients.contains(&x.name))
            .for_each(|x| match x.recipes.as_mut() {
                Some(recipes) => recipes.push(recipe.clone()),
                None => x.recipes = Some(vec![recipe.clone()]),
            });

        self.recipes.push(recipe);
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
        for item in self.collection.iter_mut() {
            if let Some(recipes) = item.recipes.as_mut() {
                if let Some(i) = recipes.iter().position(|recipe| recipe.as_str() == name) {
                    recipes.remove(i);
                }
            }
        }
        Ok(())
    }

    pub fn recipe_ingredients(&self, recipe: &str) -> impl Iterator<Item = &Item> {
        self.collection
            .iter()
            .filter(|item| item.recipes.is_some())
            .filter(|item| {
                item.recipes
                    .as_ref()
                    .unwrap()
                    .contains(&RecipeName::from_str(recipe).unwrap())
            })
            .collect::<Vec<_>>()
            .into_iter()
    }
}
