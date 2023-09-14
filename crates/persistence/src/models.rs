use crate::schema::{checklist, items, items_recipes, items_sections, list, recipes, sections};
use common::recipes::RecipeName;
use diesel::prelude::*;

pub trait ItemInfo {
    fn name(&self) -> &str;
}

#[derive(Queryable)]
#[diesel(table_name = checklist)]
pub struct ChecklistItem {
    pub item_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = checklist)]
pub struct NewChecklistItem {
    pub item_id: i32,
}

#[derive(Queryable)]
#[diesel(table_name = items)]
pub struct Item {
    pub id: i32,
    pub name: String,
}

impl From<Item> for common::item::Item {
    fn from(item: Item) -> common::item::Item {
        common::item::Item::new(item.name)
    }
}

impl ItemInfo for Item {
    fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Queryable)]
#[diesel(table_name = list)]
pub struct ListItem {
    pub item_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = list)]
pub struct NewListItem {
    pub item_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = items)]
pub struct NewItem<'a> {
    pub name: &'a str,
}

#[derive(Insertable)]
#[diesel(table_name = recipes)]
pub struct NewRecipe<'a> {
    pub name: &'a str,
}

#[derive(Queryable)]
#[diesel(table_name = recipes)]
pub struct Recipe {
    pub id: i32,
    pub name: String,
}

impl From<Recipe> for RecipeName {
    fn from(recipe: Recipe) -> RecipeName {
        RecipeName::new_unchecked(recipe.name)
    }
}

impl ItemInfo for Recipe {
    fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Insertable)]
#[diesel(table_name = sections)]
pub struct NewSection<'a> {
    pub name: &'a str,
}

#[derive(Queryable)]
#[diesel(table_name = sections)]
pub struct Section {
    pub id: i32,
    pub name: String,
}

impl ItemInfo for Section {
    fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Insertable)]
#[diesel(table_name = items_recipes)]
pub struct NewItemRecipe {
    pub item_id: i32,
    pub recipe_id: i32,
}

#[derive(Queryable)]
#[diesel(table_name = items_recipes)]
pub struct ItemRecipe {
    pub item_id: i32,
    pub recipe_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = items_sections)]
pub struct NewItemSection {
    pub item_id: i32,
    pub section_id: i32,
}

#[derive(Queryable)]
#[diesel(table_name = items_sections)]
pub struct ItemSection {
    pub item_id: i32,
    pub section_id: i32,
}
