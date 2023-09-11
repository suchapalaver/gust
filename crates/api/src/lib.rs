use common::commands::{Add, ApiCommand, Delete, Read, Update};
use db::{
    models::{ItemInfo, Section},
    persistence::Store,
};

use colored::Colorize;

pub fn execute(command: &ApiCommand, store: &mut Store) {
    match command {
        ApiCommand::Add(Add::ChecklistItem(name)) => store.add_checklist_item(name),
        ApiCommand::Add(Add::Recipe {
            recipe,
            ingredients,
        }) => store.add_recipe(recipe, ingredients),
        ApiCommand::Add(Add::Item { name, .. }) => store.add_item(name),
        ApiCommand::Add(Add::ListItem(name)) => store.add_list_item(name),
        ApiCommand::Add(Add::ListRecipe(_recipe)) => todo!(),
        ApiCommand::Add(Add::NewList) => todo!(),
        ApiCommand::Delete(Delete::ChecklistItem(name)) => store.delete_checklist_item(name),
        ApiCommand::Delete(Delete::ClearChecklist) => todo!(),
        ApiCommand::Delete(Delete::ClearList) => todo!(),
        ApiCommand::Delete(Delete::Item(_name)) => todo!(),
        ApiCommand::Delete(Delete::ListItem(_name)) => todo!(),
        ApiCommand::Delete(Delete::Recipe(recipe)) => store.delete_recipe(recipe).unwrap(),
        ApiCommand::Read(Read::All) => {
            let results = store.items();
            display(results, "items");
        }
        ApiCommand::Read(Read::Checklist) => {
            let items = store.checklist();
            display(items, "checklist")
        }
        ApiCommand::Read(Read::Item(_name)) => todo!(),
        ApiCommand::Read(Read::Items) => todo!(),
        ApiCommand::Read(Read::List) => {
            let cmd = ApiCommand::Read(Read::Checklist);
            execute(&cmd, store);
            let items = store.list();
            display(items, "list")
        }
        ApiCommand::Read(Read::ListRecipes) => todo!(),
        ApiCommand::Read(Read::Recipe(recipe)) => {
            let _ = store.recipe_ingredients(recipe);
        }
        ApiCommand::Read(Read::Recipes) => {
            let results = store.recipes();
            display(results, "recipes");
        }
        ApiCommand::Read(Read::Sections) => {
            let results = store.sections();
            display_sections(results, "sections");
        }
        ApiCommand::Update(Update::Item(_name)) => todo!(),
        ApiCommand::Update(Update::Recipe(_name)) => todo!(),
    }
}

pub(crate) fn display<T: ItemInfo>(items: Vec<T>, to_display: &str) {
    println!("{}{}", to_display.blue().bold(), ":".blue().bold());
    for item in items {
        println!(" {} {}", "-".bold().blue(), item.name().blue());
    }
}

pub(crate) fn display_sections(items: Vec<Section>, to_display: &str) {
    display(items, to_display)
}
