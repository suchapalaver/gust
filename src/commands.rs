use crate::{persistence::Store, show::display, Ingredients, ItemName, RecipeName, Section};

pub(crate) enum Add {
    ChecklistItem(ItemName),
    Item {
        name: ItemName,
        section: Option<Section>,
    },
    ListItem(ItemName),
    NewList,
    ListRecipe(RecipeName),
    Recipe {
        recipe: RecipeName,
        ingredients: Ingredients,
    },
}

pub(crate) enum Delete {
    ChecklistItem(ItemName),
    ClearChecklist,
    ClearList,
    Item(ItemName),
    ListItem(ItemName),
    Recipe(RecipeName),
}

pub(crate) enum Read {
    All,
    Checklist,
    Item(ItemName),
    Items,
    List,
    ListRecipes,
    Recipe(RecipeName),
    Recipes,
    Sections,
}

pub(crate) enum Update {
    Item(ItemName),
    Recipe(RecipeName),
}

pub(crate) enum ApiCommand {
    Add(Add),
    Delete(Delete),
    Read(Read),
    Update(Update),
}

impl ApiCommand {
    pub(crate) fn execute(&self, store: &mut Store) {
        match self {
            ApiCommand::Add(Add::ChecklistItem(name)) => store.add_checklist_item(name),
            ApiCommand::Add(Add::Recipe {
                recipe,
                ingredients,
            }) => store.add_recipe(recipe, ingredients),
            ApiCommand::Add(Add::Item { name, .. }) => store.add_item(name),
            ApiCommand::Add(Add::ListItem(name)) => store.add_list_item(name),
            ApiCommand::Add(Add::ListRecipe(recipe)) => todo!(),
            ApiCommand::Add(Add::NewList) => todo!(),
            ApiCommand::Delete(Delete::ChecklistItem(name)) => store.delete_checklist_item(name),
            ApiCommand::Delete(Delete::ClearChecklist) => todo!(),
            ApiCommand::Delete(Delete::ClearList) => todo!(),
            ApiCommand::Delete(Delete::Item(name)) => todo!(),
            ApiCommand::Delete(Delete::ListItem(name)) => todo!(),
            ApiCommand::Delete(Delete::Recipe(recipe)) => store.delete_recipe(recipe).unwrap(),
            ApiCommand::Read(Read::All) => store.show_items(),
            ApiCommand::Read(Read::Checklist) => {
                let items = store.checklist();
                display(items, "checklist")
            }
            ApiCommand::Read(Read::Item(name)) => todo!(),
            ApiCommand::Read(Read::Items) => todo!(),
            ApiCommand::Read(Read::List) => {
                let cmd = ApiCommand::Read(Read::Checklist);
                cmd.execute(store);
                let items = store.list();
                display(items, "list")
            }
            ApiCommand::Read(Read::ListRecipes) => todo!(),
            ApiCommand::Read(Read::Recipe(recipe)) => {
                let _ = store.recipe_ingredients(recipe);
            }
            ApiCommand::Read(Read::Recipes) => store.show_recipes(),
            ApiCommand::Read(Read::Sections) => store.show_sections(),
            ApiCommand::Update(Update::Item(name)) => todo!(),
            ApiCommand::Update(Update::Recipe(name)) => todo!(),
        }
    }
}
