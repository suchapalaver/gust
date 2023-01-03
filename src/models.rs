use crate::schema::{checklist, items, items_recipes, items_sections, recipes, sections};
use diesel::prelude::*;

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
