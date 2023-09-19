use crate::schema::{checklist, items, items_recipes, items_sections, list, recipes, sections};
use common::recipes::Recipe;
use diesel::prelude::*;

pub trait ItemInfo {
    fn name(&self) -> &str;
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
pub struct RecipeModel {
    pub id: i32,
    pub name: String,
}

impl From<RecipeModel> for Recipe {
    fn from(recipe: RecipeModel) -> Recipe {
        Recipe::new_unchecked(recipe.name)
    }
}

impl ItemInfo for RecipeModel {
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

impl From<Section> for common::sections::Section {
    fn from(value: Section) -> common::sections::Section {
        common::sections::Section::new(value.name)
    }
}

#[derive(Queryable)]
#[diesel(table_name = checklist)]
pub struct ChecklistItem {
    pub id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = checklist)]
pub struct NewChecklistItem {
    pub id: i32,
}

#[derive(Queryable)]
#[diesel(table_name = list)]
pub struct ListItem {
    pub id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = list)]
pub struct NewListItem {
    pub id: i32,
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
