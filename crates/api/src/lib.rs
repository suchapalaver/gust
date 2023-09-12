use std::fmt::{self, Display};

use common::commands::{Add, ApiCommand, Delete, Read, Update};
use persistence::{
    models::{ItemInfo, Section},
    store::{Storage, Store},
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
            ApiCommand::Add(cmd) => match cmd {
                Add::ChecklistItem(name) => self.store.add_checklist_item(name),
                Add::Recipe {
                    recipe,
                    ingredients,
                } => self.store.add_recipe(recipe, ingredients),
                Add::Item { name, .. } => self.store.add_item(name),
                Add::ListItem(name) => self.store.add_list_item(name),
                Add::ListRecipe(_recipe) => todo!(),
                Add::NewList => todo!(),
            },
            ApiCommand::Delete(cmd) => match cmd {
                Delete::ChecklistItem(name) => self.store.delete_checklist_item(name),
                Delete::ClearChecklist => todo!(),
                Delete::ClearList => todo!(),
                Delete::Item(_name) => todo!(),
                Delete::ListItem(_name) => todo!(),
                Delete::Recipe(recipe) => self.store.delete_recipe(recipe).unwrap(),
            },
            ApiCommand::Read(cmd) => match cmd {
                Read::All => {
                    let results = self.store.items();
                    display(results, ToDisplay::Items);
                }
                Read::Checklist => {
                    let items = self.store.checklist();
                    display(items, ToDisplay::Checklist)
                }
                Read::Item(_name) => todo!(),
                Read::Items => todo!(),
                Read::List => {
                    let cmd = ApiCommand::Read(Read::Checklist);
                    self.execute(&cmd);
                    let items = self.store.list();
                    display(items, ToDisplay::List)
                }
                Read::ListRecipes => todo!(),
                Read::Recipe(recipe) => {
                    let _ = self.store.recipe_ingredients(recipe);
                }
                Read::Recipes => {
                    let results = self.store.recipes();
                    display(results, ToDisplay::Recipes);
                }
                Read::Sections => {
                    let results = self.store.sections();
                    display_sections(results, ToDisplay::Sections);
                }
            },
            ApiCommand::Update(cmd) => match cmd {
                Update::Item(_name) => todo!(),
                Update::Recipe(_name) => todo!(),
            },
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
