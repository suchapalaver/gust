use common::commands::{Add, ApiCommand, Delete, Read, Update};
use db::{
    models::{ItemInfo, Section},
    persistence::Store,
};

use colored::Colorize;

pub struct Api {
    store: Store,
}

impl Api {
    pub fn new(store: Store) -> Self {
        Self { store }
    }

    pub fn execute(&mut self, command: &ApiCommand) {
        match command {
            ApiCommand::Add(Add::ChecklistItem(name)) => self.store.add_checklist_item(name),
            ApiCommand::Add(Add::Recipe {
                recipe,
                ingredients,
            }) => self.store.add_recipe(recipe, ingredients),
            ApiCommand::Add(Add::Item { name, .. }) => self.store.add_item(name),
            ApiCommand::Add(Add::ListItem(name)) => self.store.add_list_item(name),
            ApiCommand::Add(Add::ListRecipe(_recipe)) => todo!(),
            ApiCommand::Add(Add::NewList) => todo!(),
            ApiCommand::Delete(Delete::ChecklistItem(name)) => {
                self.store.delete_checklist_item(name)
            }
            ApiCommand::Delete(Delete::ClearChecklist) => todo!(),
            ApiCommand::Delete(Delete::ClearList) => todo!(),
            ApiCommand::Delete(Delete::Item(_name)) => todo!(),
            ApiCommand::Delete(Delete::ListItem(_name)) => todo!(),
            ApiCommand::Delete(Delete::Recipe(recipe)) => self.store.delete_recipe(recipe).unwrap(),
            ApiCommand::Read(Read::All) => {
                let results = self.store.items();
                display(results, "items");
            }
            ApiCommand::Read(Read::Checklist) => {
                let items = self.store.checklist();
                display(items, "checklist")
            }
            ApiCommand::Read(Read::Item(_name)) => todo!(),
            ApiCommand::Read(Read::Items) => todo!(),
            ApiCommand::Read(Read::List) => {
                let cmd = ApiCommand::Read(Read::Checklist);
                self.execute(&cmd);
                let items = self.store.list();
                display(items, "list")
            }
            ApiCommand::Read(Read::ListRecipes) => todo!(),
            ApiCommand::Read(Read::Recipe(recipe)) => {
                let _ = self.store.recipe_ingredients(recipe);
            }
            ApiCommand::Read(Read::Recipes) => {
                let results = self.store.recipes();
                display(results, "recipes");
            }
            ApiCommand::Read(Read::Sections) => {
                let results = self.store.sections();
                display_sections(results, "sections");
            }
            ApiCommand::Update(Update::Item(_name)) => todo!(),
            ApiCommand::Update(Update::Recipe(_name)) => todo!(),
        }
    }
}

fn display<T: ItemInfo>(items: Vec<T>, to_display: &str) {
    println!("{}{}", to_display.blue().bold(), ":".blue().bold());
    for item in items {
        println!(" {} {}", "-".bold().blue(), item.name().blue());
    }
}

fn display_sections(items: Vec<Section>, to_display: &str) {
    display(items, to_display)
}
