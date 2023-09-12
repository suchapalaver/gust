use std::fmt::{self, Display};

use common::commands::{Add, ApiCommand, Delete, Read, Update};
use persistence::{
    models::{ItemInfo, Section},
    store::Store,
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
                display(results, ToDisplay::Items);
            }
            ApiCommand::Read(Read::Checklist) => {
                let items = self.store.checklist();
                display(items, ToDisplay::Checklist)
            }
            ApiCommand::Read(Read::Item(_name)) => todo!(),
            ApiCommand::Read(Read::Items) => todo!(),
            ApiCommand::Read(Read::List) => {
                let cmd = ApiCommand::Read(Read::Checklist);
                self.execute(&cmd);
                let items = self.store.list();
                display(items, ToDisplay::List)
            }
            ApiCommand::Read(Read::ListRecipes) => todo!(),
            ApiCommand::Read(Read::Recipe(recipe)) => {
                let _ = self.store.recipe_ingredients(recipe);
            }
            ApiCommand::Read(Read::Recipes) => {
                let results = self.store.recipes();
                display(results, ToDisplay::Recipes);
            }
            ApiCommand::Read(Read::Sections) => {
                let results = self.store.sections();
                display_sections(results, ToDisplay::Sections);
            }
            ApiCommand::Update(Update::Item(_name)) => todo!(),
            ApiCommand::Update(Update::Recipe(_name)) => todo!(),
        }
    }
}

enum ToDisplay {
    Checklist,
    Items,
    List,
    Recipes,
    Sections,
}

impl Display for ToDisplay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Checklist => write!(f, "checklist"),
            Self::Items => write!(f, "items"),
            Self::List => write!(f, "list"),
            Self::Recipes => write!(f, "recipes"),
            Self::Sections => write!(f, "sections"),
        }
    }
}

fn display<T: ItemInfo>(items: Vec<T>, to_display: ToDisplay) {
    println!(
        "{}{}",
        to_display.to_string().blue().bold(),
        ":".blue().bold()
    );
    for item in items {
        println!(" {} {}", "-".bold().blue(), item.name().blue());
    }
}

fn display_sections(items: Vec<Section>, to_display: ToDisplay) {
    display(items, to_display)
}
