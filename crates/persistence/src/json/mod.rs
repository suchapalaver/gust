pub mod migrate;

use std::{
    collections::HashSet,
    fs::{self},
    path::{Path, PathBuf},
};

use common::{
    input::item_matches,
    item::{Item, Section},
    items::Items,
    list::List,
    recipes::{Ingredients, RecipeName},
    Load,
};

use crate::store::{Storage, StoreError};

pub const ITEMS_JSON_PATH: &str = "groceries.json";

pub const LIST_JSON_PATH: &str = "list.json";

pub struct JsonStore {
    items: PathBuf,
    list: PathBuf,
}

impl Default for JsonStore {
    fn default() -> Self {
        Self {
            items: PathBuf::from(ITEMS_JSON_PATH),
            list: PathBuf::from(LIST_JSON_PATH),
        }
    }
}

impl JsonStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_items_path(mut self, path: &Path) -> Self {
        self.items = path.to_path_buf();
        self
    }

    pub fn with_list_path(mut self, path: &Path) -> Self {
        self.list = path.to_path_buf();
        self
    }

    pub fn save_items(&self, object: impl serde::Serialize) -> Result<(), StoreError> {
        let s = serde_json::to_string(&object)?;

        Ok(fs::write(&self.items, s)?)
    }
    // TODO: I don't think it makes much sense to have these saved as separate JSON files.
    pub fn save_list(&self, object: impl serde::Serialize) -> Result<(), StoreError> {
        let s = serde_json::to_string(&object)?;

        Ok(fs::write(&self.list, s)?)
    }
}

impl Storage for JsonStore {
    fn add_item(&mut self, item: &common::item::ItemName) -> Result<(), StoreError> {
        let mut groceries = self.items()?;

        let mut present = false;

        for item in groceries.get_item_matches(item.as_str()) {
            if item_matches(item) {
                present = true;
                break;
            }
        }

        if present {
            eprintln!("Item already in library");
        } else {
            let new_item = Item::new(item.as_str());
            groceries.add_item(new_item);
            todo!();
        }
        Ok(())
    }

    fn add_checklist_item(&mut self, _item: &common::item::ItemName) -> Result<(), StoreError> {
        todo!()
    }

    fn add_list_item(&mut self, _item: &common::item::ItemName) -> Result<(), StoreError> {
        todo!()
    }

    fn add_recipe(
        &mut self,
        _recipe: &RecipeName,
        _ingredients: &common::recipes::Ingredients,
    ) -> Result<(), StoreError> {
        todo!()
    }

    fn checklist(&mut self) -> Result<Vec<Item>, StoreError> {
        todo!()
    }

    fn delete_checklist_item(&mut self, _item: &common::item::ItemName) -> Result<(), StoreError> {
        todo!()
    }

    fn delete_recipe(&mut self, _recipe: &RecipeName) -> Result<(), StoreError> {
        todo!()
    }

    fn items(&mut self) -> Result<Items, StoreError> {
        Ok(Items::from_json(&self.items)?)
    }

    fn list(&mut self) -> Result<List, StoreError> {
        let list = List::from_json(&self.list)?;
        Ok(list)
    }

    fn recipe_ingredients(
        &mut self,
        recipe: &RecipeName,
    ) -> Result<Option<Ingredients>, StoreError> {
        let lib = self.items()?;
        Ok(Some(
            lib.recipe_ingredients(&recipe.to_string())
                .map(|item| item.name.clone())
                .collect(),
        ))
    }

    fn sections(&mut self) -> Result<Vec<Section>, StoreError> {
        todo!()
    }

    fn recipes(&mut self) -> Result<Vec<RecipeName>, StoreError> {
        let mut recipes: HashSet<RecipeName> = HashSet::new();

        {
            let groceries = self.items()?;

            for item in groceries.collection {
                if let Some(item_recipes) = item.recipes {
                    for recipe in item_recipes {
                        recipes.insert(recipe);
                    }
                }
            }

            for recipe in groceries.recipes {
                recipes.insert(recipe);
            }
        }

        {
            let list = self.list()?;

            for recipe in list.recipes {
                recipes.insert(recipe);
            }
        }
        Ok(recipes.into_iter().collect())
    }
}

#[cfg(test)]
pub mod test {
    use crate::store::Store;

    use super::*;

    use assert_fs::prelude::*;
    use common::item::{ItemName, Section};

    fn test_json_file() -> Result<assert_fs::NamedTempFile, Box<dyn std::error::Error>> {
        let file = assert_fs::NamedTempFile::new("test1.json")?;
        file.write_str(
            r#"
            {"sections":["fresh","pantry","protein","dairy","freezer"],"collection":[{"name":"eggs","section":"dairy","is_ingredient":true,"recipes":["oatmeal chocolate chip cookies","fried eggs for breakfast","turkey meatballs"]},{"name":"milk","section":"dairy","is_ingredient":true,"recipes":[]},{"name":"lemons","section":"fresh","is_ingredient":true,"recipes":["chicken breasts with lemon","hummus","sheet-pan chicken with jammy tomatoes","flue flighter chicken stew"]},{"name":"ginger","section":"fresh","is_ingredient":true,"recipes":["sheet pan salmon with broccoli"]},{"name":"spinach","section":"fresh","is_ingredient":true,"recipes":["fried eggs for breakfast","flue flighter chicken stew"]},{"name":"garlic","section":"fresh","is_ingredient":true,"recipes":["sheet pan salmon with broccoli","crispy tofu with cashews and blistered snap peas","chicken breasts with lemon","hummus","tomato pasta","crispy sheet-pan noodles","flue flighter chicken stew","sheet-pan chicken with jammy tomatoes","swordfish pasta"]},{"name":"yellow onion","section":"fresh","is_ingredient":true,"recipes":["flue flighter chicken stew"]},{"name":"fizzy water","section":"dairy","is_ingredient":false,"recipes":[]},{"name":"kale","section":"fresh","is_ingredient":true,"recipes":[]},{"name":"beer","section":"dairy","is_ingredient":false,"recipes":[]},{"name":"parsley","section":"fresh","is_ingredient":true,"recipes":["turkey meatballs","flue flighter chicken stew","sheet-pan chicken with jammy tomatoes","swordfish pasta"]},{"name":"kefir","section":"dairy","is_ingredient":false,"recipes":[]},{"name":"kimchi","section":"dairy","is_ingredient":false,"recipes":[]},{"name":"sour cream","section":"dairy","is_ingredient":true,"recipes":[]},{"name":"potatoes","section":"fresh","is_ingredient":true,"recipes":[]},{"name":"broccoli","section":"fresh","is_ingredient":true,"recipes":["sheet pan salmon with broccoli"]},{"name":"asparagus","section":"fresh","is_ingredient":true,"recipes":[]},{"name":"dill","section":"fresh","is_ingredient":true,"recipes":[]},{"name":"red onion","section":"fresh","is_ingredient":true,"recipes":[]},{"name":"unsalted butter","section":"dairy","is_ingredient":true,"recipes":["chicken breasts with lemon","oatmeal chocolate chip cookies","fried eggs for breakfast"]},{"name":"scallions","section":"fresh","is_ingredient":true,"recipes":["sheet pan salmon with broccoli","crispy tofu with cashews and blistered snap peas"]},{"name":"mozzarella","section":"dairy","is_ingredient":true,"recipes":[]},{"name":"cucumbers","section":"fresh","is_ingredient":true,"recipes":[]},{"name":"greek yogurt","section":"dairy","is_ingredient":true,"recipes":[]},{"name":"cream cheese","section":"dairy","is_ingredient":true,"recipes":[]},{"name":"sweet potato","section":"fresh","is_ingredient":false,"recipes":[]},{"name":"sausages","section":"protein","is_ingredient":true,"recipes":[]},{"name":"tofu","section":"protein","is_ingredient":true,"recipes":["crispy tofu with cashews and blistered snap peas","crispy sheet-pan noodles"]},{"name":"short grain brown rice","section":"pantry","is_ingredient":true,"recipes":["sheet pan salmon with broccoli","flue flighter chicken stew"]},{"name":"tahini","section":"pantry","is_ingredient":true,"recipes":["hummus"]},{"name":"chicken stock","section":"pantry","is_ingredient":true,"recipes":[]},{"name":"orzo","section":"pantry","is_ingredient":true,"recipes":[]},{"name":"pasta","section":"pantry","is_ingredient":true,"recipes":["tomato pasta","swordfish pasta"]},{"name":"bread","section":"pantry","is_ingredient":true,"recipes":["fried eggs for breakfast","peanut butter and jelly on toast","turkey and cheese sandwiches"]},{"name":"coffee","section":"pantry","is_ingredient":true,"recipes":[]},{"name":"cumin","section":"pantry","is_ingredient":true,"recipes":[]},{"name":"coconut milk (unsweetened)","section":"pantry","is_ingredient":true,"recipes":["crispy tofu with cashews and blistered snap peas"]},{"name":"tortilla chips","section":"pantry","is_ingredient":true,"recipes":[]},{"name":"Ritz crackers","section":"pantry","is_ingredient":true,"recipes":[]},{"name":"black beans","section":"pantry","is_ingredient":true,"recipes":[]},{"name":"mustard","section":"pantry","is_ingredient":true,"recipes":["turkey and cheese sandwiches"]},{"name":"chips","section":"pantry","is_ingredient":true,"recipes":[]},{"name":"popcorn","section":"pantry","is_ingredient":true,"recipes":[]},{"name":"olive oil","section":"pantry","is_ingredient":true,"recipes":["sheet pan salmon with broccoli","chicken breasts with lemon","hummus","tomato pasta","sheet-pan chicken with jammy tomatoes","turkey meatballs","swordfish pasta"]},{"name":"honey","section":"pantry","is_ingredient":true,"recipes":["sheet pan salmon with broccoli","crispy tofu with cashews and blistered snap peas"]},{"name":"black pepper","section":"pantry","is_ingredient":true,"recipes":["sheet pan salmon with broccoli","sheet-pan chicken with jammy tomatoes"]},{"name":"apple cider vinegar","section":"pantry","is_ingredient":true,"recipes":[]},{"name":"pickles","section":"pantry","is_ingredient":true,"recipes":[]},{"name":"jasmine rice","section":"pantry","is_ingredient":true,"recipes":[]},{"name":"rice vinegar","section":"pantry","is_ingredient":true,"recipes":["sheet pan salmon with broccoli","crispy tofu with cashews and blistered snap peas"]},{"name":"balsamic vinegar","section":"pantry","is_ingredient":true,"recipes":[]},{"name":"vegetable oil","section":"pantry","is_ingredient":true,"recipes":["crispy tofu with cashews and blistered snap peas","crispy sheet-pan noodles"]},{"name":"baking soda","section":"pantry","is_ingredient":true,"recipes":[]},{"name":"mayonnaise","section":"pantry","is_ingredient":true,"recipes":["turkey and cheese sandwiches"]},{"name":"cannellini beans","section":"pantry","is_ingredient":true,"recipes":[]},{"name":"whole-wheat tortillas","section":"pantry","is_ingredient":true,"recipes":[]},{"name":"dumplings","section":"freezer","is_ingredient":false,"recipes":[]},{"name":"edamame","section":"freezer","is_ingredient":false,"recipes":[]},{"name":"ice cream","section":"freezer","is_ingredient":false,"recipes":[]},{"name":"old fashioned rolled oats","section":"pantry","is_ingredient":true,"recipes":["oatmeal chocolate chip cookies"]},{"name":"chocolate chips","section":"pantry","is_ingredient":true,"recipes":["oatmeal chocolate chip cookies"]},{"name":"baking powder","section":"pantry","is_ingredient":true,"recipes":["oatmeal chocolate chip cookies"]},{"name":"baking soda","section":"pantry","is_ingredient":true,"recipes":["oatmeal chocolate chip cookies"]},{"name":"salt","section":"pantry","is_ingredient":true,"recipes":["sheet pan salmon with broccoli","oatmeal chocolate chip cookies","crispy sheet-pan noodles","sheet-pan chicken with jammy tomatoes"]},{"name":"white sugar","section":"pantry","is_ingredient":true,"recipes":["oatmeal chocolate chip cookies"]},{"name":"vanilla extract","section":"pantry","is_ingredient":true,"recipes":["oatmeal chocolate chip cookies"]},{"name":"whole-wheat flour","section":"pantry","is_ingredient":true,"recipes":["oatmeal chocolate chip cookies"]},{"name":"tomatoes","section":"fresh","is_ingredient":true,"recipes":["tomato pasta"]},{"name":"basil","section":"fresh","is_ingredient":true,"recipes":["tomato pasta"]},{"name":"parmigiana","section":"dairy","is_ingredient":true,"recipes":["tomato pasta","turkey meatballs"]},{"name":"1/2 & 1/2","section":"dairy","is_ingredient":true,"recipes":["fried eggs for breakfast"]},{"name":"feta","section":"dairy","is_ingredient":true,"recipes":["fried eggs for breakfast"]},{"name":"instant ramen noodles","section":"pantry","is_ingredient":true,"recipes":["crispy sheet-pan noodles"]},{"name":"sesame oil","section":"pantry","is_ingredient":true,"recipes":["sheet pan salmon with broccoli","crispy sheet-pan noodles"]},{"name":"soy sauce","section":"pantry","is_ingredient":true,"recipes":["sheet pan salmon with broccoli","crispy tofu with cashews and blistered snap peas","crispy sheet-pan noodles"]},{"name":"baby bok choy","section":"fresh","is_ingredient":true,"recipes":["crispy sheet-pan noodles"]},{"name":"cilantro","section":"fresh","is_ingredient":true,"recipes":["crispy sheet-pan noodles"]},{"name":"hoisin","section":"pantry","is_ingredient":true,"recipes":["crispy sheet-pan noodles"]},{"name":"maple syrup","section":"pantry","is_ingredient":true,"recipes":["crispy sheet-pan noodles"]},{"name":"sesame seeds","section":"pantry","is_ingredient":true,"recipes":["sheet pan salmon with broccoli","crispy sheet-pan noodles"]},{"name":"ground turkey","section":"protein","is_ingredient":true,"recipes":["turkey meatballs"]},{"name":"panko bread crumbs","section":"pantry","is_ingredient":true,"recipes":["turkey meatballs"]},{"name":"garlic powder","section":"pantry","is_ingredient":true,"recipes":["turkey meatballs"]},{"name":"skinless boneless chicken thighs","section":"protein","is_ingredient":true,"recipes":["flue flighter chicken stew","sheet-pan chicken with jammy tomatoes"]},{"name":"carrots","section":"fresh","is_ingredient":true,"recipes":["flue flighter chicken stew"]},{"name":"red pepper flakes","section":"pantry","is_ingredient":true,"recipes":["flue flighter chicken stew","crispy tofu with cashews and blistered snap peas"]},{"name":"chicken broth","section":"pantry","is_ingredient":true,"recipes":["flue flighter chicken stew","chicken breasts with lemon"]},{"name":"string beans","section":"fresh","is_ingredient":false,"recipes":[]},{"name":"peaches","section":"fresh","is_ingredient":false,"recipes":[]},{"name":"whipped cream","section":"dairy","is_ingredient":false,"recipes":[]},{"name":"kiwi fruit","section":"fresh","is_ingredient":false,"recipes":[]},{"name":"marscapone cheese","section":"dairy","is_ingredient":false,"recipes":[]},{"name":"swordfish","section":"protein","is_ingredient":true,"recipes":["swordfish pasta"]},{"name":"eggplant","section":"fresh","is_ingredient":true,"recipes":["swordfish pasta"]},{"name":"tomato puree","section":"pantry","is_ingredient":true,"recipes":["swordfish pasta"]},{"name":"pine nuts","section":"pantry","is_ingredient":true,"recipes":["swordfish pasta"]},{"name":"french bread","section":"pantry","is_ingredient":false,"recipes":[]},{"name":"cayenne pepper","section":"pantry","is_ingredient":false,"recipes":[]}],"recipes":["oatmeal chocolate chip cookies","tomato pasta","fried eggs for breakfast","crispy sheet-pan noodles","turkey meatballs","flue flighter chicken stew","sheet-pan chicken with jammy tomatoes","turkey and cheese sandwiches","peanut butter and jelly on toast","cheese and apple snack","hummus","chicken breasts with lemon","crispy tofu with cashews and blistered snap peas","swordfish pasta"]}"#)?;
        Ok(file)
    }

    fn items() -> Items {
        let file = test_json_file().unwrap();
        let mut store = Store::Json(JsonStore::new().with_items_path(file.path()));
        store.items().unwrap()
    }

    #[test]
    fn test_groceries_default() -> Result<(), Box<dyn std::error::Error>> {
        let default_items = Items::default();
        insta::assert_json_snapshot!(default_items, @r#"
      {
        "sections": [],
        "collection": [],
        "recipes": []
      }
      "#);
        Ok(())
    }

    #[test]
    fn test_save_items() -> Result<(), Box<dyn std::error::Error>> {
        let mut store = JsonStore::new().with_items_path(&PathBuf::from("test_groceries.json"));
        let items = Items::default();
        insta::assert_json_snapshot!(items, @r#"
    {
      "sections": [],
      "collection": [],
      "recipes": []
    }
    "#);
        store.save_items(items)?;
        let items = store.items().unwrap();
        insta::assert_json_snapshot!(items, @r#"
    {
      "sections": [],
      "collection": [],
      "recipes": []
    }
    "#);
        std::fs::remove_file(store.items)?;
        Ok(())
    }

    #[test]
    fn test_delete_recipe() -> Result<(), Box<dyn std::error::Error>> {
        let mut items = items();
        insta::assert_json_snapshot!(items.recipes, @r#"
        [
          "oatmeal chocolate chip cookies",
          "tomato pasta",
          "fried eggs for breakfast",
          "crispy sheet-pan noodles",
          "turkey meatballs",
          "flue flighter chicken stew",
          "sheet-pan chicken with jammy tomatoes",
          "turkey and cheese sandwiches",
          "peanut butter and jelly on toast",
          "cheese and apple snack",
          "hummus",
          "chicken breasts with lemon",
          "crispy tofu with cashews and blistered snap peas",
          "swordfish pasta"
        ]
        "#);
        items.delete_recipe("oatmeal chocolate chip cookies")?;
        insta::assert_json_snapshot!(items.recipes, @r#"
        [
          "tomato pasta",
          "fried eggs for breakfast",
          "crispy sheet-pan noodles",
          "turkey meatballs",
          "flue flighter chicken stew",
          "sheet-pan chicken with jammy tomatoes",
          "turkey and cheese sandwiches",
          "peanut butter and jelly on toast",
          "cheese and apple snack",
          "hummus",
          "chicken breasts with lemon",
          "crispy tofu with cashews and blistered snap peas",
          "swordfish pasta"
        ]
        "#);
        Ok(())
    }

    #[test]
    fn test_delete_item() -> Result<(), Box<dyn std::error::Error>> {
        let mut items = items();
        insta::assert_json_snapshot!(items.collection, @r###"
        [
          {
            "name": "eggs",
            "section": "dairy",
            "recipes": [
              "oatmeal chocolate chip cookies",
              "fried eggs for breakfast",
              "turkey meatballs"
            ]
          },
          {
            "name": "milk",
            "section": "dairy",
            "recipes": []
          },
          {
            "name": "lemons",
            "section": "fresh",
            "recipes": [
              "chicken breasts with lemon",
              "hummus",
              "sheet-pan chicken with jammy tomatoes",
              "flue flighter chicken stew"
            ]
          },
          {
            "name": "ginger",
            "section": "fresh",
            "recipes": [
              "sheet pan salmon with broccoli"
            ]
          },
          {
            "name": "spinach",
            "section": "fresh",
            "recipes": [
              "fried eggs for breakfast",
              "flue flighter chicken stew"
            ]
          },
          {
            "name": "garlic",
            "section": "fresh",
            "recipes": [
              "sheet pan salmon with broccoli",
              "crispy tofu with cashews and blistered snap peas",
              "chicken breasts with lemon",
              "hummus",
              "tomato pasta",
              "crispy sheet-pan noodles",
              "flue flighter chicken stew",
              "sheet-pan chicken with jammy tomatoes",
              "swordfish pasta"
            ]
          },
          {
            "name": "yellow onion",
            "section": "fresh",
            "recipes": [
              "flue flighter chicken stew"
            ]
          },
          {
            "name": "fizzy water",
            "section": "dairy",
            "recipes": []
          },
          {
            "name": "kale",
            "section": "fresh",
            "recipes": []
          },
          {
            "name": "beer",
            "section": "dairy",
            "recipes": []
          },
          {
            "name": "parsley",
            "section": "fresh",
            "recipes": [
              "turkey meatballs",
              "flue flighter chicken stew",
              "sheet-pan chicken with jammy tomatoes",
              "swordfish pasta"
            ]
          },
          {
            "name": "kefir",
            "section": "dairy",
            "recipes": []
          },
          {
            "name": "kimchi",
            "section": "dairy",
            "recipes": []
          },
          {
            "name": "sour cream",
            "section": "dairy",
            "recipes": []
          },
          {
            "name": "potatoes",
            "section": "fresh",
            "recipes": []
          },
          {
            "name": "broccoli",
            "section": "fresh",
            "recipes": [
              "sheet pan salmon with broccoli"
            ]
          },
          {
            "name": "asparagus",
            "section": "fresh",
            "recipes": []
          },
          {
            "name": "dill",
            "section": "fresh",
            "recipes": []
          },
          {
            "name": "red onion",
            "section": "fresh",
            "recipes": []
          },
          {
            "name": "unsalted butter",
            "section": "dairy",
            "recipes": [
              "chicken breasts with lemon",
              "oatmeal chocolate chip cookies",
              "fried eggs for breakfast"
            ]
          },
          {
            "name": "scallions",
            "section": "fresh",
            "recipes": [
              "sheet pan salmon with broccoli",
              "crispy tofu with cashews and blistered snap peas"
            ]
          },
          {
            "name": "mozzarella",
            "section": "dairy",
            "recipes": []
          },
          {
            "name": "cucumbers",
            "section": "fresh",
            "recipes": []
          },
          {
            "name": "greek yogurt",
            "section": "dairy",
            "recipes": []
          },
          {
            "name": "cream cheese",
            "section": "dairy",
            "recipes": []
          },
          {
            "name": "sweet potato",
            "section": "fresh",
            "recipes": []
          },
          {
            "name": "sausages",
            "section": "protein",
            "recipes": []
          },
          {
            "name": "tofu",
            "section": "protein",
            "recipes": [
              "crispy tofu with cashews and blistered snap peas",
              "crispy sheet-pan noodles"
            ]
          },
          {
            "name": "short grain brown rice",
            "section": "pantry",
            "recipes": [
              "sheet pan salmon with broccoli",
              "flue flighter chicken stew"
            ]
          },
          {
            "name": "tahini",
            "section": "pantry",
            "recipes": [
              "hummus"
            ]
          },
          {
            "name": "chicken stock",
            "section": "pantry",
            "recipes": []
          },
          {
            "name": "orzo",
            "section": "pantry",
            "recipes": []
          },
          {
            "name": "pasta",
            "section": "pantry",
            "recipes": [
              "tomato pasta",
              "swordfish pasta"
            ]
          },
          {
            "name": "bread",
            "section": "pantry",
            "recipes": [
              "fried eggs for breakfast",
              "peanut butter and jelly on toast",
              "turkey and cheese sandwiches"
            ]
          },
          {
            "name": "coffee",
            "section": "pantry",
            "recipes": []
          },
          {
            "name": "cumin",
            "section": "pantry",
            "recipes": []
          },
          {
            "name": "coconut milk (unsweetened)",
            "section": "pantry",
            "recipes": [
              "crispy tofu with cashews and blistered snap peas"
            ]
          },
          {
            "name": "tortilla chips",
            "section": "pantry",
            "recipes": []
          },
          {
            "name": "Ritz crackers",
            "section": "pantry",
            "recipes": []
          },
          {
            "name": "black beans",
            "section": "pantry",
            "recipes": []
          },
          {
            "name": "mustard",
            "section": "pantry",
            "recipes": [
              "turkey and cheese sandwiches"
            ]
          },
          {
            "name": "chips",
            "section": "pantry",
            "recipes": []
          },
          {
            "name": "popcorn",
            "section": "pantry",
            "recipes": []
          },
          {
            "name": "olive oil",
            "section": "pantry",
            "recipes": [
              "sheet pan salmon with broccoli",
              "chicken breasts with lemon",
              "hummus",
              "tomato pasta",
              "sheet-pan chicken with jammy tomatoes",
              "turkey meatballs",
              "swordfish pasta"
            ]
          },
          {
            "name": "honey",
            "section": "pantry",
            "recipes": [
              "sheet pan salmon with broccoli",
              "crispy tofu with cashews and blistered snap peas"
            ]
          },
          {
            "name": "black pepper",
            "section": "pantry",
            "recipes": [
              "sheet pan salmon with broccoli",
              "sheet-pan chicken with jammy tomatoes"
            ]
          },
          {
            "name": "apple cider vinegar",
            "section": "pantry",
            "recipes": []
          },
          {
            "name": "pickles",
            "section": "pantry",
            "recipes": []
          },
          {
            "name": "jasmine rice",
            "section": "pantry",
            "recipes": []
          },
          {
            "name": "rice vinegar",
            "section": "pantry",
            "recipes": [
              "sheet pan salmon with broccoli",
              "crispy tofu with cashews and blistered snap peas"
            ]
          },
          {
            "name": "balsamic vinegar",
            "section": "pantry",
            "recipes": []
          },
          {
            "name": "vegetable oil",
            "section": "pantry",
            "recipes": [
              "crispy tofu with cashews and blistered snap peas",
              "crispy sheet-pan noodles"
            ]
          },
          {
            "name": "baking soda",
            "section": "pantry",
            "recipes": []
          },
          {
            "name": "mayonnaise",
            "section": "pantry",
            "recipes": [
              "turkey and cheese sandwiches"
            ]
          },
          {
            "name": "cannellini beans",
            "section": "pantry",
            "recipes": []
          },
          {
            "name": "whole-wheat tortillas",
            "section": "pantry",
            "recipes": []
          },
          {
            "name": "dumplings",
            "section": "freezer",
            "recipes": []
          },
          {
            "name": "edamame",
            "section": "freezer",
            "recipes": []
          },
          {
            "name": "ice cream",
            "section": "freezer",
            "recipes": []
          },
          {
            "name": "old fashioned rolled oats",
            "section": "pantry",
            "recipes": [
              "oatmeal chocolate chip cookies"
            ]
          },
          {
            "name": "chocolate chips",
            "section": "pantry",
            "recipes": [
              "oatmeal chocolate chip cookies"
            ]
          },
          {
            "name": "baking powder",
            "section": "pantry",
            "recipes": [
              "oatmeal chocolate chip cookies"
            ]
          },
          {
            "name": "baking soda",
            "section": "pantry",
            "recipes": [
              "oatmeal chocolate chip cookies"
            ]
          },
          {
            "name": "salt",
            "section": "pantry",
            "recipes": [
              "sheet pan salmon with broccoli",
              "oatmeal chocolate chip cookies",
              "crispy sheet-pan noodles",
              "sheet-pan chicken with jammy tomatoes"
            ]
          },
          {
            "name": "white sugar",
            "section": "pantry",
            "recipes": [
              "oatmeal chocolate chip cookies"
            ]
          },
          {
            "name": "vanilla extract",
            "section": "pantry",
            "recipes": [
              "oatmeal chocolate chip cookies"
            ]
          },
          {
            "name": "whole-wheat flour",
            "section": "pantry",
            "recipes": [
              "oatmeal chocolate chip cookies"
            ]
          },
          {
            "name": "tomatoes",
            "section": "fresh",
            "recipes": [
              "tomato pasta"
            ]
          },
          {
            "name": "basil",
            "section": "fresh",
            "recipes": [
              "tomato pasta"
            ]
          },
          {
            "name": "parmigiana",
            "section": "dairy",
            "recipes": [
              "tomato pasta",
              "turkey meatballs"
            ]
          },
          {
            "name": "1/2 & 1/2",
            "section": "dairy",
            "recipes": [
              "fried eggs for breakfast"
            ]
          },
          {
            "name": "feta",
            "section": "dairy",
            "recipes": [
              "fried eggs for breakfast"
            ]
          },
          {
            "name": "instant ramen noodles",
            "section": "pantry",
            "recipes": [
              "crispy sheet-pan noodles"
            ]
          },
          {
            "name": "sesame oil",
            "section": "pantry",
            "recipes": [
              "sheet pan salmon with broccoli",
              "crispy sheet-pan noodles"
            ]
          },
          {
            "name": "soy sauce",
            "section": "pantry",
            "recipes": [
              "sheet pan salmon with broccoli",
              "crispy tofu with cashews and blistered snap peas",
              "crispy sheet-pan noodles"
            ]
          },
          {
            "name": "baby bok choy",
            "section": "fresh",
            "recipes": [
              "crispy sheet-pan noodles"
            ]
          },
          {
            "name": "cilantro",
            "section": "fresh",
            "recipes": [
              "crispy sheet-pan noodles"
            ]
          },
          {
            "name": "hoisin",
            "section": "pantry",
            "recipes": [
              "crispy sheet-pan noodles"
            ]
          },
          {
            "name": "maple syrup",
            "section": "pantry",
            "recipes": [
              "crispy sheet-pan noodles"
            ]
          },
          {
            "name": "sesame seeds",
            "section": "pantry",
            "recipes": [
              "sheet pan salmon with broccoli",
              "crispy sheet-pan noodles"
            ]
          },
          {
            "name": "ground turkey",
            "section": "protein",
            "recipes": [
              "turkey meatballs"
            ]
          },
          {
            "name": "panko bread crumbs",
            "section": "pantry",
            "recipes": [
              "turkey meatballs"
            ]
          },
          {
            "name": "garlic powder",
            "section": "pantry",
            "recipes": [
              "turkey meatballs"
            ]
          },
          {
            "name": "skinless boneless chicken thighs",
            "section": "protein",
            "recipes": [
              "flue flighter chicken stew",
              "sheet-pan chicken with jammy tomatoes"
            ]
          },
          {
            "name": "carrots",
            "section": "fresh",
            "recipes": [
              "flue flighter chicken stew"
            ]
          },
          {
            "name": "red pepper flakes",
            "section": "pantry",
            "recipes": [
              "flue flighter chicken stew",
              "crispy tofu with cashews and blistered snap peas"
            ]
          },
          {
            "name": "chicken broth",
            "section": "pantry",
            "recipes": [
              "flue flighter chicken stew",
              "chicken breasts with lemon"
            ]
          },
          {
            "name": "string beans",
            "section": "fresh",
            "recipes": []
          },
          {
            "name": "peaches",
            "section": "fresh",
            "recipes": []
          },
          {
            "name": "whipped cream",
            "section": "dairy",
            "recipes": []
          },
          {
            "name": "kiwi fruit",
            "section": "fresh",
            "recipes": []
          },
          {
            "name": "marscapone cheese",
            "section": "dairy",
            "recipes": []
          },
          {
            "name": "swordfish",
            "section": "protein",
            "recipes": [
              "swordfish pasta"
            ]
          },
          {
            "name": "eggplant",
            "section": "fresh",
            "recipes": [
              "swordfish pasta"
            ]
          },
          {
            "name": "tomato puree",
            "section": "pantry",
            "recipes": [
              "swordfish pasta"
            ]
          },
          {
            "name": "pine nuts",
            "section": "pantry",
            "recipes": [
              "swordfish pasta"
            ]
          },
          {
            "name": "french bread",
            "section": "pantry",
            "recipes": []
          },
          {
            "name": "cayenne pepper",
            "section": "pantry",
            "recipes": []
          }
        ]
        "###);
        items.delete_item("eggs")?;
        insta::assert_json_snapshot!(items.collection, @r###"
        [
          {
            "name": "milk",
            "section": "dairy",
            "recipes": []
          },
          {
            "name": "lemons",
            "section": "fresh",
            "recipes": [
              "chicken breasts with lemon",
              "hummus",
              "sheet-pan chicken with jammy tomatoes",
              "flue flighter chicken stew"
            ]
          },
          {
            "name": "ginger",
            "section": "fresh",
            "recipes": [
              "sheet pan salmon with broccoli"
            ]
          },
          {
            "name": "spinach",
            "section": "fresh",
            "recipes": [
              "fried eggs for breakfast",
              "flue flighter chicken stew"
            ]
          },
          {
            "name": "garlic",
            "section": "fresh",
            "recipes": [
              "sheet pan salmon with broccoli",
              "crispy tofu with cashews and blistered snap peas",
              "chicken breasts with lemon",
              "hummus",
              "tomato pasta",
              "crispy sheet-pan noodles",
              "flue flighter chicken stew",
              "sheet-pan chicken with jammy tomatoes",
              "swordfish pasta"
            ]
          },
          {
            "name": "yellow onion",
            "section": "fresh",
            "recipes": [
              "flue flighter chicken stew"
            ]
          },
          {
            "name": "fizzy water",
            "section": "dairy",
            "recipes": []
          },
          {
            "name": "kale",
            "section": "fresh",
            "recipes": []
          },
          {
            "name": "beer",
            "section": "dairy",
            "recipes": []
          },
          {
            "name": "parsley",
            "section": "fresh",
            "recipes": [
              "turkey meatballs",
              "flue flighter chicken stew",
              "sheet-pan chicken with jammy tomatoes",
              "swordfish pasta"
            ]
          },
          {
            "name": "kefir",
            "section": "dairy",
            "recipes": []
          },
          {
            "name": "kimchi",
            "section": "dairy",
            "recipes": []
          },
          {
            "name": "sour cream",
            "section": "dairy",
            "recipes": []
          },
          {
            "name": "potatoes",
            "section": "fresh",
            "recipes": []
          },
          {
            "name": "broccoli",
            "section": "fresh",
            "recipes": [
              "sheet pan salmon with broccoli"
            ]
          },
          {
            "name": "asparagus",
            "section": "fresh",
            "recipes": []
          },
          {
            "name": "dill",
            "section": "fresh",
            "recipes": []
          },
          {
            "name": "red onion",
            "section": "fresh",
            "recipes": []
          },
          {
            "name": "unsalted butter",
            "section": "dairy",
            "recipes": [
              "chicken breasts with lemon",
              "oatmeal chocolate chip cookies",
              "fried eggs for breakfast"
            ]
          },
          {
            "name": "scallions",
            "section": "fresh",
            "recipes": [
              "sheet pan salmon with broccoli",
              "crispy tofu with cashews and blistered snap peas"
            ]
          },
          {
            "name": "mozzarella",
            "section": "dairy",
            "recipes": []
          },
          {
            "name": "cucumbers",
            "section": "fresh",
            "recipes": []
          },
          {
            "name": "greek yogurt",
            "section": "dairy",
            "recipes": []
          },
          {
            "name": "cream cheese",
            "section": "dairy",
            "recipes": []
          },
          {
            "name": "sweet potato",
            "section": "fresh",
            "recipes": []
          },
          {
            "name": "sausages",
            "section": "protein",
            "recipes": []
          },
          {
            "name": "tofu",
            "section": "protein",
            "recipes": [
              "crispy tofu with cashews and blistered snap peas",
              "crispy sheet-pan noodles"
            ]
          },
          {
            "name": "short grain brown rice",
            "section": "pantry",
            "recipes": [
              "sheet pan salmon with broccoli",
              "flue flighter chicken stew"
            ]
          },
          {
            "name": "tahini",
            "section": "pantry",
            "recipes": [
              "hummus"
            ]
          },
          {
            "name": "chicken stock",
            "section": "pantry",
            "recipes": []
          },
          {
            "name": "orzo",
            "section": "pantry",
            "recipes": []
          },
          {
            "name": "pasta",
            "section": "pantry",
            "recipes": [
              "tomato pasta",
              "swordfish pasta"
            ]
          },
          {
            "name": "bread",
            "section": "pantry",
            "recipes": [
              "fried eggs for breakfast",
              "peanut butter and jelly on toast",
              "turkey and cheese sandwiches"
            ]
          },
          {
            "name": "coffee",
            "section": "pantry",
            "recipes": []
          },
          {
            "name": "cumin",
            "section": "pantry",
            "recipes": []
          },
          {
            "name": "coconut milk (unsweetened)",
            "section": "pantry",
            "recipes": [
              "crispy tofu with cashews and blistered snap peas"
            ]
          },
          {
            "name": "tortilla chips",
            "section": "pantry",
            "recipes": []
          },
          {
            "name": "Ritz crackers",
            "section": "pantry",
            "recipes": []
          },
          {
            "name": "black beans",
            "section": "pantry",
            "recipes": []
          },
          {
            "name": "mustard",
            "section": "pantry",
            "recipes": [
              "turkey and cheese sandwiches"
            ]
          },
          {
            "name": "chips",
            "section": "pantry",
            "recipes": []
          },
          {
            "name": "popcorn",
            "section": "pantry",
            "recipes": []
          },
          {
            "name": "olive oil",
            "section": "pantry",
            "recipes": [
              "sheet pan salmon with broccoli",
              "chicken breasts with lemon",
              "hummus",
              "tomato pasta",
              "sheet-pan chicken with jammy tomatoes",
              "turkey meatballs",
              "swordfish pasta"
            ]
          },
          {
            "name": "honey",
            "section": "pantry",
            "recipes": [
              "sheet pan salmon with broccoli",
              "crispy tofu with cashews and blistered snap peas"
            ]
          },
          {
            "name": "black pepper",
            "section": "pantry",
            "recipes": [
              "sheet pan salmon with broccoli",
              "sheet-pan chicken with jammy tomatoes"
            ]
          },
          {
            "name": "apple cider vinegar",
            "section": "pantry",
            "recipes": []
          },
          {
            "name": "pickles",
            "section": "pantry",
            "recipes": []
          },
          {
            "name": "jasmine rice",
            "section": "pantry",
            "recipes": []
          },
          {
            "name": "rice vinegar",
            "section": "pantry",
            "recipes": [
              "sheet pan salmon with broccoli",
              "crispy tofu with cashews and blistered snap peas"
            ]
          },
          {
            "name": "balsamic vinegar",
            "section": "pantry",
            "recipes": []
          },
          {
            "name": "vegetable oil",
            "section": "pantry",
            "recipes": [
              "crispy tofu with cashews and blistered snap peas",
              "crispy sheet-pan noodles"
            ]
          },
          {
            "name": "baking soda",
            "section": "pantry",
            "recipes": []
          },
          {
            "name": "mayonnaise",
            "section": "pantry",
            "recipes": [
              "turkey and cheese sandwiches"
            ]
          },
          {
            "name": "cannellini beans",
            "section": "pantry",
            "recipes": []
          },
          {
            "name": "whole-wheat tortillas",
            "section": "pantry",
            "recipes": []
          },
          {
            "name": "dumplings",
            "section": "freezer",
            "recipes": []
          },
          {
            "name": "edamame",
            "section": "freezer",
            "recipes": []
          },
          {
            "name": "ice cream",
            "section": "freezer",
            "recipes": []
          },
          {
            "name": "old fashioned rolled oats",
            "section": "pantry",
            "recipes": [
              "oatmeal chocolate chip cookies"
            ]
          },
          {
            "name": "chocolate chips",
            "section": "pantry",
            "recipes": [
              "oatmeal chocolate chip cookies"
            ]
          },
          {
            "name": "baking powder",
            "section": "pantry",
            "recipes": [
              "oatmeal chocolate chip cookies"
            ]
          },
          {
            "name": "baking soda",
            "section": "pantry",
            "recipes": [
              "oatmeal chocolate chip cookies"
            ]
          },
          {
            "name": "salt",
            "section": "pantry",
            "recipes": [
              "sheet pan salmon with broccoli",
              "oatmeal chocolate chip cookies",
              "crispy sheet-pan noodles",
              "sheet-pan chicken with jammy tomatoes"
            ]
          },
          {
            "name": "white sugar",
            "section": "pantry",
            "recipes": [
              "oatmeal chocolate chip cookies"
            ]
          },
          {
            "name": "vanilla extract",
            "section": "pantry",
            "recipes": [
              "oatmeal chocolate chip cookies"
            ]
          },
          {
            "name": "whole-wheat flour",
            "section": "pantry",
            "recipes": [
              "oatmeal chocolate chip cookies"
            ]
          },
          {
            "name": "tomatoes",
            "section": "fresh",
            "recipes": [
              "tomato pasta"
            ]
          },
          {
            "name": "basil",
            "section": "fresh",
            "recipes": [
              "tomato pasta"
            ]
          },
          {
            "name": "parmigiana",
            "section": "dairy",
            "recipes": [
              "tomato pasta",
              "turkey meatballs"
            ]
          },
          {
            "name": "1/2 & 1/2",
            "section": "dairy",
            "recipes": [
              "fried eggs for breakfast"
            ]
          },
          {
            "name": "feta",
            "section": "dairy",
            "recipes": [
              "fried eggs for breakfast"
            ]
          },
          {
            "name": "instant ramen noodles",
            "section": "pantry",
            "recipes": [
              "crispy sheet-pan noodles"
            ]
          },
          {
            "name": "sesame oil",
            "section": "pantry",
            "recipes": [
              "sheet pan salmon with broccoli",
              "crispy sheet-pan noodles"
            ]
          },
          {
            "name": "soy sauce",
            "section": "pantry",
            "recipes": [
              "sheet pan salmon with broccoli",
              "crispy tofu with cashews and blistered snap peas",
              "crispy sheet-pan noodles"
            ]
          },
          {
            "name": "baby bok choy",
            "section": "fresh",
            "recipes": [
              "crispy sheet-pan noodles"
            ]
          },
          {
            "name": "cilantro",
            "section": "fresh",
            "recipes": [
              "crispy sheet-pan noodles"
            ]
          },
          {
            "name": "hoisin",
            "section": "pantry",
            "recipes": [
              "crispy sheet-pan noodles"
            ]
          },
          {
            "name": "maple syrup",
            "section": "pantry",
            "recipes": [
              "crispy sheet-pan noodles"
            ]
          },
          {
            "name": "sesame seeds",
            "section": "pantry",
            "recipes": [
              "sheet pan salmon with broccoli",
              "crispy sheet-pan noodles"
            ]
          },
          {
            "name": "ground turkey",
            "section": "protein",
            "recipes": [
              "turkey meatballs"
            ]
          },
          {
            "name": "panko bread crumbs",
            "section": "pantry",
            "recipes": [
              "turkey meatballs"
            ]
          },
          {
            "name": "garlic powder",
            "section": "pantry",
            "recipes": [
              "turkey meatballs"
            ]
          },
          {
            "name": "skinless boneless chicken thighs",
            "section": "protein",
            "recipes": [
              "flue flighter chicken stew",
              "sheet-pan chicken with jammy tomatoes"
            ]
          },
          {
            "name": "carrots",
            "section": "fresh",
            "recipes": [
              "flue flighter chicken stew"
            ]
          },
          {
            "name": "red pepper flakes",
            "section": "pantry",
            "recipes": [
              "flue flighter chicken stew",
              "crispy tofu with cashews and blistered snap peas"
            ]
          },
          {
            "name": "chicken broth",
            "section": "pantry",
            "recipes": [
              "flue flighter chicken stew",
              "chicken breasts with lemon"
            ]
          },
          {
            "name": "string beans",
            "section": "fresh",
            "recipes": []
          },
          {
            "name": "peaches",
            "section": "fresh",
            "recipes": []
          },
          {
            "name": "whipped cream",
            "section": "dairy",
            "recipes": []
          },
          {
            "name": "kiwi fruit",
            "section": "fresh",
            "recipes": []
          },
          {
            "name": "marscapone cheese",
            "section": "dairy",
            "recipes": []
          },
          {
            "name": "swordfish",
            "section": "protein",
            "recipes": [
              "swordfish pasta"
            ]
          },
          {
            "name": "eggplant",
            "section": "fresh",
            "recipes": [
              "swordfish pasta"
            ]
          },
          {
            "name": "tomato puree",
            "section": "pantry",
            "recipes": [
              "swordfish pasta"
            ]
          },
          {
            "name": "pine nuts",
            "section": "pantry",
            "recipes": [
              "swordfish pasta"
            ]
          },
          {
            "name": "french bread",
            "section": "pantry",
            "recipes": []
          },
          {
            "name": "cayenne pepper",
            "section": "pantry",
            "recipes": []
          }
        ]
        "###);
        Ok(())
    }

    #[test]
    fn test_groceries() -> Result<(), Box<dyn std::error::Error>> {
        let mut items = items();
        insta::assert_json_snapshot!(items, @r###"
        {
          "sections": [
            "fresh",
            "pantry",
            "protein",
            "dairy",
            "freezer"
          ],
          "collection": [
            {
              "name": "eggs",
              "section": "dairy",
              "recipes": [
                "oatmeal chocolate chip cookies",
                "fried eggs for breakfast",
                "turkey meatballs"
              ]
            },
            {
              "name": "milk",
              "section": "dairy",
              "recipes": []
            },
            {
              "name": "lemons",
              "section": "fresh",
              "recipes": [
                "chicken breasts with lemon",
                "hummus",
                "sheet-pan chicken with jammy tomatoes",
                "flue flighter chicken stew"
              ]
            },
            {
              "name": "ginger",
              "section": "fresh",
              "recipes": [
                "sheet pan salmon with broccoli"
              ]
            },
            {
              "name": "spinach",
              "section": "fresh",
              "recipes": [
                "fried eggs for breakfast",
                "flue flighter chicken stew"
              ]
            },
            {
              "name": "garlic",
              "section": "fresh",
              "recipes": [
                "sheet pan salmon with broccoli",
                "crispy tofu with cashews and blistered snap peas",
                "chicken breasts with lemon",
                "hummus",
                "tomato pasta",
                "crispy sheet-pan noodles",
                "flue flighter chicken stew",
                "sheet-pan chicken with jammy tomatoes",
                "swordfish pasta"
              ]
            },
            {
              "name": "yellow onion",
              "section": "fresh",
              "recipes": [
                "flue flighter chicken stew"
              ]
            },
            {
              "name": "fizzy water",
              "section": "dairy",
              "recipes": []
            },
            {
              "name": "kale",
              "section": "fresh",
              "recipes": []
            },
            {
              "name": "beer",
              "section": "dairy",
              "recipes": []
            },
            {
              "name": "parsley",
              "section": "fresh",
              "recipes": [
                "turkey meatballs",
                "flue flighter chicken stew",
                "sheet-pan chicken with jammy tomatoes",
                "swordfish pasta"
              ]
            },
            {
              "name": "kefir",
              "section": "dairy",
              "recipes": []
            },
            {
              "name": "kimchi",
              "section": "dairy",
              "recipes": []
            },
            {
              "name": "sour cream",
              "section": "dairy",
              "recipes": []
            },
            {
              "name": "potatoes",
              "section": "fresh",
              "recipes": []
            },
            {
              "name": "broccoli",
              "section": "fresh",
              "recipes": [
                "sheet pan salmon with broccoli"
              ]
            },
            {
              "name": "asparagus",
              "section": "fresh",
              "recipes": []
            },
            {
              "name": "dill",
              "section": "fresh",
              "recipes": []
            },
            {
              "name": "red onion",
              "section": "fresh",
              "recipes": []
            },
            {
              "name": "unsalted butter",
              "section": "dairy",
              "recipes": [
                "chicken breasts with lemon",
                "oatmeal chocolate chip cookies",
                "fried eggs for breakfast"
              ]
            },
            {
              "name": "scallions",
              "section": "fresh",
              "recipes": [
                "sheet pan salmon with broccoli",
                "crispy tofu with cashews and blistered snap peas"
              ]
            },
            {
              "name": "mozzarella",
              "section": "dairy",
              "recipes": []
            },
            {
              "name": "cucumbers",
              "section": "fresh",
              "recipes": []
            },
            {
              "name": "greek yogurt",
              "section": "dairy",
              "recipes": []
            },
            {
              "name": "cream cheese",
              "section": "dairy",
              "recipes": []
            },
            {
              "name": "sweet potato",
              "section": "fresh",
              "recipes": []
            },
            {
              "name": "sausages",
              "section": "protein",
              "recipes": []
            },
            {
              "name": "tofu",
              "section": "protein",
              "recipes": [
                "crispy tofu with cashews and blistered snap peas",
                "crispy sheet-pan noodles"
              ]
            },
            {
              "name": "short grain brown rice",
              "section": "pantry",
              "recipes": [
                "sheet pan salmon with broccoli",
                "flue flighter chicken stew"
              ]
            },
            {
              "name": "tahini",
              "section": "pantry",
              "recipes": [
                "hummus"
              ]
            },
            {
              "name": "chicken stock",
              "section": "pantry",
              "recipes": []
            },
            {
              "name": "orzo",
              "section": "pantry",
              "recipes": []
            },
            {
              "name": "pasta",
              "section": "pantry",
              "recipes": [
                "tomato pasta",
                "swordfish pasta"
              ]
            },
            {
              "name": "bread",
              "section": "pantry",
              "recipes": [
                "fried eggs for breakfast",
                "peanut butter and jelly on toast",
                "turkey and cheese sandwiches"
              ]
            },
            {
              "name": "coffee",
              "section": "pantry",
              "recipes": []
            },
            {
              "name": "cumin",
              "section": "pantry",
              "recipes": []
            },
            {
              "name": "coconut milk (unsweetened)",
              "section": "pantry",
              "recipes": [
                "crispy tofu with cashews and blistered snap peas"
              ]
            },
            {
              "name": "tortilla chips",
              "section": "pantry",
              "recipes": []
            },
            {
              "name": "Ritz crackers",
              "section": "pantry",
              "recipes": []
            },
            {
              "name": "black beans",
              "section": "pantry",
              "recipes": []
            },
            {
              "name": "mustard",
              "section": "pantry",
              "recipes": [
                "turkey and cheese sandwiches"
              ]
            },
            {
              "name": "chips",
              "section": "pantry",
              "recipes": []
            },
            {
              "name": "popcorn",
              "section": "pantry",
              "recipes": []
            },
            {
              "name": "olive oil",
              "section": "pantry",
              "recipes": [
                "sheet pan salmon with broccoli",
                "chicken breasts with lemon",
                "hummus",
                "tomato pasta",
                "sheet-pan chicken with jammy tomatoes",
                "turkey meatballs",
                "swordfish pasta"
              ]
            },
            {
              "name": "honey",
              "section": "pantry",
              "recipes": [
                "sheet pan salmon with broccoli",
                "crispy tofu with cashews and blistered snap peas"
              ]
            },
            {
              "name": "black pepper",
              "section": "pantry",
              "recipes": [
                "sheet pan salmon with broccoli",
                "sheet-pan chicken with jammy tomatoes"
              ]
            },
            {
              "name": "apple cider vinegar",
              "section": "pantry",
              "recipes": []
            },
            {
              "name": "pickles",
              "section": "pantry",
              "recipes": []
            },
            {
              "name": "jasmine rice",
              "section": "pantry",
              "recipes": []
            },
            {
              "name": "rice vinegar",
              "section": "pantry",
              "recipes": [
                "sheet pan salmon with broccoli",
                "crispy tofu with cashews and blistered snap peas"
              ]
            },
            {
              "name": "balsamic vinegar",
              "section": "pantry",
              "recipes": []
            },
            {
              "name": "vegetable oil",
              "section": "pantry",
              "recipes": [
                "crispy tofu with cashews and blistered snap peas",
                "crispy sheet-pan noodles"
              ]
            },
            {
              "name": "baking soda",
              "section": "pantry",
              "recipes": []
            },
            {
              "name": "mayonnaise",
              "section": "pantry",
              "recipes": [
                "turkey and cheese sandwiches"
              ]
            },
            {
              "name": "cannellini beans",
              "section": "pantry",
              "recipes": []
            },
            {
              "name": "whole-wheat tortillas",
              "section": "pantry",
              "recipes": []
            },
            {
              "name": "dumplings",
              "section": "freezer",
              "recipes": []
            },
            {
              "name": "edamame",
              "section": "freezer",
              "recipes": []
            },
            {
              "name": "ice cream",
              "section": "freezer",
              "recipes": []
            },
            {
              "name": "old fashioned rolled oats",
              "section": "pantry",
              "recipes": [
                "oatmeal chocolate chip cookies"
              ]
            },
            {
              "name": "chocolate chips",
              "section": "pantry",
              "recipes": [
                "oatmeal chocolate chip cookies"
              ]
            },
            {
              "name": "baking powder",
              "section": "pantry",
              "recipes": [
                "oatmeal chocolate chip cookies"
              ]
            },
            {
              "name": "baking soda",
              "section": "pantry",
              "recipes": [
                "oatmeal chocolate chip cookies"
              ]
            },
            {
              "name": "salt",
              "section": "pantry",
              "recipes": [
                "sheet pan salmon with broccoli",
                "oatmeal chocolate chip cookies",
                "crispy sheet-pan noodles",
                "sheet-pan chicken with jammy tomatoes"
              ]
            },
            {
              "name": "white sugar",
              "section": "pantry",
              "recipes": [
                "oatmeal chocolate chip cookies"
              ]
            },
            {
              "name": "vanilla extract",
              "section": "pantry",
              "recipes": [
                "oatmeal chocolate chip cookies"
              ]
            },
            {
              "name": "whole-wheat flour",
              "section": "pantry",
              "recipes": [
                "oatmeal chocolate chip cookies"
              ]
            },
            {
              "name": "tomatoes",
              "section": "fresh",
              "recipes": [
                "tomato pasta"
              ]
            },
            {
              "name": "basil",
              "section": "fresh",
              "recipes": [
                "tomato pasta"
              ]
            },
            {
              "name": "parmigiana",
              "section": "dairy",
              "recipes": [
                "tomato pasta",
                "turkey meatballs"
              ]
            },
            {
              "name": "1/2 & 1/2",
              "section": "dairy",
              "recipes": [
                "fried eggs for breakfast"
              ]
            },
            {
              "name": "feta",
              "section": "dairy",
              "recipes": [
                "fried eggs for breakfast"
              ]
            },
            {
              "name": "instant ramen noodles",
              "section": "pantry",
              "recipes": [
                "crispy sheet-pan noodles"
              ]
            },
            {
              "name": "sesame oil",
              "section": "pantry",
              "recipes": [
                "sheet pan salmon with broccoli",
                "crispy sheet-pan noodles"
              ]
            },
            {
              "name": "soy sauce",
              "section": "pantry",
              "recipes": [
                "sheet pan salmon with broccoli",
                "crispy tofu with cashews and blistered snap peas",
                "crispy sheet-pan noodles"
              ]
            },
            {
              "name": "baby bok choy",
              "section": "fresh",
              "recipes": [
                "crispy sheet-pan noodles"
              ]
            },
            {
              "name": "cilantro",
              "section": "fresh",
              "recipes": [
                "crispy sheet-pan noodles"
              ]
            },
            {
              "name": "hoisin",
              "section": "pantry",
              "recipes": [
                "crispy sheet-pan noodles"
              ]
            },
            {
              "name": "maple syrup",
              "section": "pantry",
              "recipes": [
                "crispy sheet-pan noodles"
              ]
            },
            {
              "name": "sesame seeds",
              "section": "pantry",
              "recipes": [
                "sheet pan salmon with broccoli",
                "crispy sheet-pan noodles"
              ]
            },
            {
              "name": "ground turkey",
              "section": "protein",
              "recipes": [
                "turkey meatballs"
              ]
            },
            {
              "name": "panko bread crumbs",
              "section": "pantry",
              "recipes": [
                "turkey meatballs"
              ]
            },
            {
              "name": "garlic powder",
              "section": "pantry",
              "recipes": [
                "turkey meatballs"
              ]
            },
            {
              "name": "skinless boneless chicken thighs",
              "section": "protein",
              "recipes": [
                "flue flighter chicken stew",
                "sheet-pan chicken with jammy tomatoes"
              ]
            },
            {
              "name": "carrots",
              "section": "fresh",
              "recipes": [
                "flue flighter chicken stew"
              ]
            },
            {
              "name": "red pepper flakes",
              "section": "pantry",
              "recipes": [
                "flue flighter chicken stew",
                "crispy tofu with cashews and blistered snap peas"
              ]
            },
            {
              "name": "chicken broth",
              "section": "pantry",
              "recipes": [
                "flue flighter chicken stew",
                "chicken breasts with lemon"
              ]
            },
            {
              "name": "string beans",
              "section": "fresh",
              "recipes": []
            },
            {
              "name": "peaches",
              "section": "fresh",
              "recipes": []
            },
            {
              "name": "whipped cream",
              "section": "dairy",
              "recipes": []
            },
            {
              "name": "kiwi fruit",
              "section": "fresh",
              "recipes": []
            },
            {
              "name": "marscapone cheese",
              "section": "dairy",
              "recipes": []
            },
            {
              "name": "swordfish",
              "section": "protein",
              "recipes": [
                "swordfish pasta"
              ]
            },
            {
              "name": "eggplant",
              "section": "fresh",
              "recipes": [
                "swordfish pasta"
              ]
            },
            {
              "name": "tomato puree",
              "section": "pantry",
              "recipes": [
                "swordfish pasta"
              ]
            },
            {
              "name": "pine nuts",
              "section": "pantry",
              "recipes": [
                "swordfish pasta"
              ]
            },
            {
              "name": "french bread",
              "section": "pantry",
              "recipes": []
            },
            {
              "name": "cayenne pepper",
              "section": "pantry",
              "recipes": []
            }
          ],
          "recipes": [
            "oatmeal chocolate chip cookies",
            "tomato pasta",
            "fried eggs for breakfast",
            "crispy sheet-pan noodles",
            "turkey meatballs",
            "flue flighter chicken stew",
            "sheet-pan chicken with jammy tomatoes",
            "turkey and cheese sandwiches",
            "peanut butter and jelly on toast",
            "cheese and apple snack",
            "hummus",
            "chicken breasts with lemon",
            "crispy tofu with cashews and blistered snap peas",
            "swordfish pasta"
          ]
        }
        "###);

        let item = Item {
            name: ItemName::from("cumquats"),
            section: Some(Section::from("fresh")),
            recipes: Some(vec![RecipeName::from("cumquat chutney")]),
        };
        let recipe = "cumquat chutney";

        let ingredients = "kumquats, carrots, dried apricots, dried cranberries, chili, onion, garlic, cider vinegar, granulated sugar, honey, kosher salt, cardamom, cloves, coriander, ginger, black peppercorns";

        items.add_item(item);
        items.add_recipe(recipe, ingredients);

        insta::assert_json_snapshot!(items, @r###"
        {
          "sections": [
            "fresh",
            "pantry",
            "protein",
            "dairy",
            "freezer"
          ],
          "collection": [
            {
              "name": "eggs",
              "section": "dairy",
              "recipes": [
                "oatmeal chocolate chip cookies",
                "fried eggs for breakfast",
                "turkey meatballs"
              ]
            },
            {
              "name": "milk",
              "section": "dairy",
              "recipes": []
            },
            {
              "name": "lemons",
              "section": "fresh",
              "recipes": [
                "chicken breasts with lemon",
                "hummus",
                "sheet-pan chicken with jammy tomatoes",
                "flue flighter chicken stew"
              ]
            },
            {
              "name": "ginger",
              "section": "fresh",
              "recipes": [
                "sheet pan salmon with broccoli",
                "cumquat chutney"
              ]
            },
            {
              "name": "spinach",
              "section": "fresh",
              "recipes": [
                "fried eggs for breakfast",
                "flue flighter chicken stew"
              ]
            },
            {
              "name": "garlic",
              "section": "fresh",
              "recipes": [
                "sheet pan salmon with broccoli",
                "crispy tofu with cashews and blistered snap peas",
                "chicken breasts with lemon",
                "hummus",
                "tomato pasta",
                "crispy sheet-pan noodles",
                "flue flighter chicken stew",
                "sheet-pan chicken with jammy tomatoes",
                "swordfish pasta",
                "cumquat chutney"
              ]
            },
            {
              "name": "yellow onion",
              "section": "fresh",
              "recipes": [
                "flue flighter chicken stew"
              ]
            },
            {
              "name": "fizzy water",
              "section": "dairy",
              "recipes": []
            },
            {
              "name": "kale",
              "section": "fresh",
              "recipes": []
            },
            {
              "name": "beer",
              "section": "dairy",
              "recipes": []
            },
            {
              "name": "parsley",
              "section": "fresh",
              "recipes": [
                "turkey meatballs",
                "flue flighter chicken stew",
                "sheet-pan chicken with jammy tomatoes",
                "swordfish pasta"
              ]
            },
            {
              "name": "kefir",
              "section": "dairy",
              "recipes": []
            },
            {
              "name": "kimchi",
              "section": "dairy",
              "recipes": []
            },
            {
              "name": "sour cream",
              "section": "dairy",
              "recipes": []
            },
            {
              "name": "potatoes",
              "section": "fresh",
              "recipes": []
            },
            {
              "name": "broccoli",
              "section": "fresh",
              "recipes": [
                "sheet pan salmon with broccoli"
              ]
            },
            {
              "name": "asparagus",
              "section": "fresh",
              "recipes": []
            },
            {
              "name": "dill",
              "section": "fresh",
              "recipes": []
            },
            {
              "name": "red onion",
              "section": "fresh",
              "recipes": []
            },
            {
              "name": "unsalted butter",
              "section": "dairy",
              "recipes": [
                "chicken breasts with lemon",
                "oatmeal chocolate chip cookies",
                "fried eggs for breakfast"
              ]
            },
            {
              "name": "scallions",
              "section": "fresh",
              "recipes": [
                "sheet pan salmon with broccoli",
                "crispy tofu with cashews and blistered snap peas"
              ]
            },
            {
              "name": "mozzarella",
              "section": "dairy",
              "recipes": []
            },
            {
              "name": "cucumbers",
              "section": "fresh",
              "recipes": []
            },
            {
              "name": "greek yogurt",
              "section": "dairy",
              "recipes": []
            },
            {
              "name": "cream cheese",
              "section": "dairy",
              "recipes": []
            },
            {
              "name": "sweet potato",
              "section": "fresh",
              "recipes": []
            },
            {
              "name": "sausages",
              "section": "protein",
              "recipes": []
            },
            {
              "name": "tofu",
              "section": "protein",
              "recipes": [
                "crispy tofu with cashews and blistered snap peas",
                "crispy sheet-pan noodles"
              ]
            },
            {
              "name": "short grain brown rice",
              "section": "pantry",
              "recipes": [
                "sheet pan salmon with broccoli",
                "flue flighter chicken stew"
              ]
            },
            {
              "name": "tahini",
              "section": "pantry",
              "recipes": [
                "hummus"
              ]
            },
            {
              "name": "chicken stock",
              "section": "pantry",
              "recipes": []
            },
            {
              "name": "orzo",
              "section": "pantry",
              "recipes": []
            },
            {
              "name": "pasta",
              "section": "pantry",
              "recipes": [
                "tomato pasta",
                "swordfish pasta"
              ]
            },
            {
              "name": "bread",
              "section": "pantry",
              "recipes": [
                "fried eggs for breakfast",
                "peanut butter and jelly on toast",
                "turkey and cheese sandwiches"
              ]
            },
            {
              "name": "coffee",
              "section": "pantry",
              "recipes": []
            },
            {
              "name": "cumin",
              "section": "pantry",
              "recipes": []
            },
            {
              "name": "coconut milk (unsweetened)",
              "section": "pantry",
              "recipes": [
                "crispy tofu with cashews and blistered snap peas"
              ]
            },
            {
              "name": "tortilla chips",
              "section": "pantry",
              "recipes": []
            },
            {
              "name": "Ritz crackers",
              "section": "pantry",
              "recipes": []
            },
            {
              "name": "black beans",
              "section": "pantry",
              "recipes": []
            },
            {
              "name": "mustard",
              "section": "pantry",
              "recipes": [
                "turkey and cheese sandwiches"
              ]
            },
            {
              "name": "chips",
              "section": "pantry",
              "recipes": []
            },
            {
              "name": "popcorn",
              "section": "pantry",
              "recipes": []
            },
            {
              "name": "olive oil",
              "section": "pantry",
              "recipes": [
                "sheet pan salmon with broccoli",
                "chicken breasts with lemon",
                "hummus",
                "tomato pasta",
                "sheet-pan chicken with jammy tomatoes",
                "turkey meatballs",
                "swordfish pasta"
              ]
            },
            {
              "name": "honey",
              "section": "pantry",
              "recipes": [
                "sheet pan salmon with broccoli",
                "crispy tofu with cashews and blistered snap peas",
                "cumquat chutney"
              ]
            },
            {
              "name": "black pepper",
              "section": "pantry",
              "recipes": [
                "sheet pan salmon with broccoli",
                "sheet-pan chicken with jammy tomatoes"
              ]
            },
            {
              "name": "apple cider vinegar",
              "section": "pantry",
              "recipes": []
            },
            {
              "name": "pickles",
              "section": "pantry",
              "recipes": []
            },
            {
              "name": "jasmine rice",
              "section": "pantry",
              "recipes": []
            },
            {
              "name": "rice vinegar",
              "section": "pantry",
              "recipes": [
                "sheet pan salmon with broccoli",
                "crispy tofu with cashews and blistered snap peas"
              ]
            },
            {
              "name": "balsamic vinegar",
              "section": "pantry",
              "recipes": []
            },
            {
              "name": "vegetable oil",
              "section": "pantry",
              "recipes": [
                "crispy tofu with cashews and blistered snap peas",
                "crispy sheet-pan noodles"
              ]
            },
            {
              "name": "baking soda",
              "section": "pantry",
              "recipes": []
            },
            {
              "name": "mayonnaise",
              "section": "pantry",
              "recipes": [
                "turkey and cheese sandwiches"
              ]
            },
            {
              "name": "cannellini beans",
              "section": "pantry",
              "recipes": []
            },
            {
              "name": "whole-wheat tortillas",
              "section": "pantry",
              "recipes": []
            },
            {
              "name": "dumplings",
              "section": "freezer",
              "recipes": []
            },
            {
              "name": "edamame",
              "section": "freezer",
              "recipes": []
            },
            {
              "name": "ice cream",
              "section": "freezer",
              "recipes": []
            },
            {
              "name": "old fashioned rolled oats",
              "section": "pantry",
              "recipes": [
                "oatmeal chocolate chip cookies"
              ]
            },
            {
              "name": "chocolate chips",
              "section": "pantry",
              "recipes": [
                "oatmeal chocolate chip cookies"
              ]
            },
            {
              "name": "baking powder",
              "section": "pantry",
              "recipes": [
                "oatmeal chocolate chip cookies"
              ]
            },
            {
              "name": "baking soda",
              "section": "pantry",
              "recipes": [
                "oatmeal chocolate chip cookies"
              ]
            },
            {
              "name": "salt",
              "section": "pantry",
              "recipes": [
                "sheet pan salmon with broccoli",
                "oatmeal chocolate chip cookies",
                "crispy sheet-pan noodles",
                "sheet-pan chicken with jammy tomatoes"
              ]
            },
            {
              "name": "white sugar",
              "section": "pantry",
              "recipes": [
                "oatmeal chocolate chip cookies"
              ]
            },
            {
              "name": "vanilla extract",
              "section": "pantry",
              "recipes": [
                "oatmeal chocolate chip cookies"
              ]
            },
            {
              "name": "whole-wheat flour",
              "section": "pantry",
              "recipes": [
                "oatmeal chocolate chip cookies"
              ]
            },
            {
              "name": "tomatoes",
              "section": "fresh",
              "recipes": [
                "tomato pasta"
              ]
            },
            {
              "name": "basil",
              "section": "fresh",
              "recipes": [
                "tomato pasta"
              ]
            },
            {
              "name": "parmigiana",
              "section": "dairy",
              "recipes": [
                "tomato pasta",
                "turkey meatballs"
              ]
            },
            {
              "name": "1/2 & 1/2",
              "section": "dairy",
              "recipes": [
                "fried eggs for breakfast"
              ]
            },
            {
              "name": "feta",
              "section": "dairy",
              "recipes": [
                "fried eggs for breakfast"
              ]
            },
            {
              "name": "instant ramen noodles",
              "section": "pantry",
              "recipes": [
                "crispy sheet-pan noodles"
              ]
            },
            {
              "name": "sesame oil",
              "section": "pantry",
              "recipes": [
                "sheet pan salmon with broccoli",
                "crispy sheet-pan noodles"
              ]
            },
            {
              "name": "soy sauce",
              "section": "pantry",
              "recipes": [
                "sheet pan salmon with broccoli",
                "crispy tofu with cashews and blistered snap peas",
                "crispy sheet-pan noodles"
              ]
            },
            {
              "name": "baby bok choy",
              "section": "fresh",
              "recipes": [
                "crispy sheet-pan noodles"
              ]
            },
            {
              "name": "cilantro",
              "section": "fresh",
              "recipes": [
                "crispy sheet-pan noodles"
              ]
            },
            {
              "name": "hoisin",
              "section": "pantry",
              "recipes": [
                "crispy sheet-pan noodles"
              ]
            },
            {
              "name": "maple syrup",
              "section": "pantry",
              "recipes": [
                "crispy sheet-pan noodles"
              ]
            },
            {
              "name": "sesame seeds",
              "section": "pantry",
              "recipes": [
                "sheet pan salmon with broccoli",
                "crispy sheet-pan noodles"
              ]
            },
            {
              "name": "ground turkey",
              "section": "protein",
              "recipes": [
                "turkey meatballs"
              ]
            },
            {
              "name": "panko bread crumbs",
              "section": "pantry",
              "recipes": [
                "turkey meatballs"
              ]
            },
            {
              "name": "garlic powder",
              "section": "pantry",
              "recipes": [
                "turkey meatballs"
              ]
            },
            {
              "name": "skinless boneless chicken thighs",
              "section": "protein",
              "recipes": [
                "flue flighter chicken stew",
                "sheet-pan chicken with jammy tomatoes"
              ]
            },
            {
              "name": "carrots",
              "section": "fresh",
              "recipes": [
                "flue flighter chicken stew",
                "cumquat chutney"
              ]
            },
            {
              "name": "red pepper flakes",
              "section": "pantry",
              "recipes": [
                "flue flighter chicken stew",
                "crispy tofu with cashews and blistered snap peas"
              ]
            },
            {
              "name": "chicken broth",
              "section": "pantry",
              "recipes": [
                "flue flighter chicken stew",
                "chicken breasts with lemon"
              ]
            },
            {
              "name": "string beans",
              "section": "fresh",
              "recipes": []
            },
            {
              "name": "peaches",
              "section": "fresh",
              "recipes": []
            },
            {
              "name": "whipped cream",
              "section": "dairy",
              "recipes": []
            },
            {
              "name": "kiwi fruit",
              "section": "fresh",
              "recipes": []
            },
            {
              "name": "marscapone cheese",
              "section": "dairy",
              "recipes": []
            },
            {
              "name": "swordfish",
              "section": "protein",
              "recipes": [
                "swordfish pasta"
              ]
            },
            {
              "name": "eggplant",
              "section": "fresh",
              "recipes": [
                "swordfish pasta"
              ]
            },
            {
              "name": "tomato puree",
              "section": "pantry",
              "recipes": [
                "swordfish pasta"
              ]
            },
            {
              "name": "pine nuts",
              "section": "pantry",
              "recipes": [
                "swordfish pasta"
              ]
            },
            {
              "name": "french bread",
              "section": "pantry",
              "recipes": []
            },
            {
              "name": "cayenne pepper",
              "section": "pantry",
              "recipes": []
            },
            {
              "name": "cumquats",
              "section": "fresh",
              "recipes": [
                "cumquat chutney"
              ]
            }
          ],
          "recipes": [
            "oatmeal chocolate chip cookies",
            "tomato pasta",
            "fried eggs for breakfast",
            "crispy sheet-pan noodles",
            "turkey meatballs",
            "flue flighter chicken stew",
            "sheet-pan chicken with jammy tomatoes",
            "turkey and cheese sandwiches",
            "peanut butter and jelly on toast",
            "cheese and apple snack",
            "hummus",
            "chicken breasts with lemon",
            "crispy tofu with cashews and blistered snap peas",
            "swordfish pasta",
            "cumquat chutney"
          ]
        }
        "###);

        Ok(())
    }

    #[test]
    fn test_delete_item_from_list() -> Result<(), Box<dyn std::error::Error>> {
        let file = create_test_checklist_json_file().unwrap();
        let mut store = Store::Json(JsonStore::new().with_list_path(file.path()));

        let mut shopping_list = store.list().unwrap();
        let item = Item {
            name: ItemName::from("kumquats"),
            section: Some(Section::from("fresh")),
            recipes: None,
        };
        shopping_list.add_item(item);
        insta::assert_json_snapshot!(shopping_list.items, @r###"
        [
          {
            "name": "garlic",
            "section": "fresh",
            "recipes": [
              "sheet pan salmon with broccoli",
              "crispy tofu with cashews and blistered snap peas",
              "chicken breasts with lemon",
              "hummus",
              "tomato pasta",
              "crispy sheet-pan noodles",
              "flue flighter chicken stew",
              "sheet-pan chicken with jammy tomatoes",
              "swordfish pasta"
            ]
          },
          {
            "name": "tomatoes",
            "section": "fresh",
            "recipes": [
              "tomato pasta"
            ]
          },
          {
            "name": "basil",
            "section": "fresh",
            "recipes": [
              "tomato pasta"
            ]
          },
          {
            "name": "lemons",
            "section": "fresh",
            "recipes": [
              "chicken breasts with lemon",
              "hummus",
              "sheet-pan chicken with jammy tomatoes",
              "flue flighter chicken stew"
            ]
          },
          {
            "name": "pasta",
            "section": "pantry",
            "recipes": [
              "tomato pasta",
              "swordfish pasta"
            ]
          },
          {
            "name": "olive oil",
            "section": "pantry",
            "recipes": [
              "sheet pan salmon with broccoli",
              "chicken breasts with lemon",
              "hummus",
              "tomato pasta",
              "sheet-pan chicken with jammy tomatoes",
              "turkey meatballs",
              "swordfish pasta"
            ]
          },
          {
            "name": "short grain brown rice",
            "section": "pantry",
            "recipes": [
              "sheet pan salmon with broccoli",
              "flue flighter chicken stew"
            ]
          },
          {
            "name": "parmigiana",
            "section": "dairy",
            "recipes": [
              "tomato pasta",
              "turkey meatballs"
            ]
          },
          {
            "name": "eggs",
            "section": "dairy",
            "recipes": [
              "oatmeal chocolate chip cookies",
              "fried eggs for breakfast",
              "turkey meatballs"
            ]
          },
          {
            "name": "sausages",
            "section": "protein",
            "recipes": []
          },
          {
            "name": "dumplings",
            "section": "freezer",
            "recipes": []
          },
          {
            "name": "kumquats",
            "section": "fresh",
            "recipes": null
          }
        ]
        "###);
        shopping_list.delete_groceries_item("kumquats")?;
        insta::assert_json_snapshot!(shopping_list.items, @r###"
        [
          {
            "name": "garlic",
            "section": "fresh",
            "recipes": [
              "sheet pan salmon with broccoli",
              "crispy tofu with cashews and blistered snap peas",
              "chicken breasts with lemon",
              "hummus",
              "tomato pasta",
              "crispy sheet-pan noodles",
              "flue flighter chicken stew",
              "sheet-pan chicken with jammy tomatoes",
              "swordfish pasta"
            ]
          },
          {
            "name": "tomatoes",
            "section": "fresh",
            "recipes": [
              "tomato pasta"
            ]
          },
          {
            "name": "basil",
            "section": "fresh",
            "recipes": [
              "tomato pasta"
            ]
          },
          {
            "name": "lemons",
            "section": "fresh",
            "recipes": [
              "chicken breasts with lemon",
              "hummus",
              "sheet-pan chicken with jammy tomatoes",
              "flue flighter chicken stew"
            ]
          },
          {
            "name": "pasta",
            "section": "pantry",
            "recipes": [
              "tomato pasta",
              "swordfish pasta"
            ]
          },
          {
            "name": "olive oil",
            "section": "pantry",
            "recipes": [
              "sheet pan salmon with broccoli",
              "chicken breasts with lemon",
              "hummus",
              "tomato pasta",
              "sheet-pan chicken with jammy tomatoes",
              "turkey meatballs",
              "swordfish pasta"
            ]
          },
          {
            "name": "short grain brown rice",
            "section": "pantry",
            "recipes": [
              "sheet pan salmon with broccoli",
              "flue flighter chicken stew"
            ]
          },
          {
            "name": "parmigiana",
            "section": "dairy",
            "recipes": [
              "tomato pasta",
              "turkey meatballs"
            ]
          },
          {
            "name": "eggs",
            "section": "dairy",
            "recipes": [
              "oatmeal chocolate chip cookies",
              "fried eggs for breakfast",
              "turkey meatballs"
            ]
          },
          {
            "name": "sausages",
            "section": "protein",
            "recipes": []
          },
          {
            "name": "dumplings",
            "section": "freezer",
            "recipes": []
          }
        ]
        "###);
        Ok(())
    }

    fn create_test_checklist_json_file(
    ) -> Result<assert_fs::NamedTempFile, Box<dyn std::error::Error>> {
        let file = assert_fs::NamedTempFile::new("test3.json")?;
        file.write_str(
            r#"
            {"checklist":[],"recipes":["tomato pasta"],"items":[{"name":"garlic","section":"fresh","is_ingredient":true,"recipes":["sheet pan salmon with broccoli","crispy tofu with cashews and blistered snap peas","chicken breasts with lemon","hummus","tomato pasta","crispy sheet-pan noodles","flue flighter chicken stew","sheet-pan chicken with jammy tomatoes","swordfish pasta"]},{"name":"tomatoes","section":"fresh","is_ingredient":true,"recipes":["tomato pasta"]},{"name":"basil","section":"fresh","is_ingredient":true,"recipes":["tomato pasta"]},{"name":"lemons","section":"fresh","is_ingredient":true,"recipes":["chicken breasts with lemon","hummus","sheet-pan chicken with jammy tomatoes","flue flighter chicken stew"]},{"name":"pasta","section":"pantry","is_ingredient":true,"recipes":["tomato pasta","swordfish pasta"]},{"name":"olive oil","section":"pantry","is_ingredient":true,"recipes":["sheet pan salmon with broccoli","chicken breasts with lemon","hummus","tomato pasta","sheet-pan chicken with jammy tomatoes","turkey meatballs","swordfish pasta"]},{"name":"short grain brown rice","section":"pantry","is_ingredient":true,"recipes":["sheet pan salmon with broccoli","flue flighter chicken stew"]},{"name":"parmigiana","section":"dairy","is_ingredient":true,"recipes":["tomato pasta","turkey meatballs"]},{"name":"eggs","section":"dairy","is_ingredient":true,"recipes":["oatmeal chocolate chip cookies","fried eggs for breakfast","turkey meatballs"]},{"name":"sausages","section":"protein","is_ingredient":true,"recipes":[]},{"name":"dumplings","section":"freezer","is_ingredient":false,"recipes":[]}]}
            "#
        )?;
        Ok(file)
    }

    fn checklist() -> List {
        let file = create_test_checklist_json_file().unwrap();
        println!("{file:?}");
        let store = JsonStore::new().with_list_path(file.path());
        println!();
        let list = List::from_json(file.path()).unwrap();
        println!("{list:?}");
        println!();
        println!("{:?}", store.list);
        let mut store = Store::Json(store);
        store.list().unwrap()
    }

    #[test]
    fn test_delete_checklist_item() -> Result<(), Box<dyn std::error::Error>> {
        let mut shopping_list = checklist();
        let item = Item {
            name: ItemName::from("kumquats"),
            section: Some(Section::from("fresh")),
            recipes: None,
        };
        shopping_list.add_checklist_item(item);
        insta::assert_json_snapshot!(shopping_list.checklist, @r###"
        [
          {
            "name": "kumquats",
            "section": "fresh",
            "recipes": null
          }
        ]
        "###);
        shopping_list.delete_checklist_item("kumquats")?;
        insta::assert_json_snapshot!(shopping_list.checklist, @"[]");
        Ok(())
    }

    #[test]
    fn test_delete_recipe_from_list() -> Result<(), Box<dyn std::error::Error>> {
        let mut shopping_list = checklist();
        insta::assert_json_snapshot!(shopping_list.recipes, @r#"
        [
          "tomato pasta"
        ]
        "#);
        shopping_list.delete_recipe("tomato pasta")?;
        insta::assert_json_snapshot!(shopping_list.recipes, @"[]");
        Ok(())
    }

    #[test]
    fn json_from_file() -> Result<(), Box<dyn std::error::Error>> {
        let list = checklist();

        insta::assert_json_snapshot!(list, @r###"
        {
          "checklist": [],
          "recipes": [
            "tomato pasta"
          ],
          "items": [
            {
              "name": "garlic",
              "section": "fresh",
              "recipes": [
                "sheet pan salmon with broccoli",
                "crispy tofu with cashews and blistered snap peas",
                "chicken breasts with lemon",
                "hummus",
                "tomato pasta",
                "crispy sheet-pan noodles",
                "flue flighter chicken stew",
                "sheet-pan chicken with jammy tomatoes",
                "swordfish pasta"
              ]
            },
            {
              "name": "tomatoes",
              "section": "fresh",
              "recipes": [
                "tomato pasta"
              ]
            },
            {
              "name": "basil",
              "section": "fresh",
              "recipes": [
                "tomato pasta"
              ]
            },
            {
              "name": "lemons",
              "section": "fresh",
              "recipes": [
                "chicken breasts with lemon",
                "hummus",
                "sheet-pan chicken with jammy tomatoes",
                "flue flighter chicken stew"
              ]
            },
            {
              "name": "pasta",
              "section": "pantry",
              "recipes": [
                "tomato pasta",
                "swordfish pasta"
              ]
            },
            {
              "name": "olive oil",
              "section": "pantry",
              "recipes": [
                "sheet pan salmon with broccoli",
                "chicken breasts with lemon",
                "hummus",
                "tomato pasta",
                "sheet-pan chicken with jammy tomatoes",
                "turkey meatballs",
                "swordfish pasta"
              ]
            },
            {
              "name": "short grain brown rice",
              "section": "pantry",
              "recipes": [
                "sheet pan salmon with broccoli",
                "flue flighter chicken stew"
              ]
            },
            {
              "name": "parmigiana",
              "section": "dairy",
              "recipes": [
                "tomato pasta",
                "turkey meatballs"
              ]
            },
            {
              "name": "eggs",
              "section": "dairy",
              "recipes": [
                "oatmeal chocolate chip cookies",
                "fried eggs for breakfast",
                "turkey meatballs"
              ]
            },
            {
              "name": "sausages",
              "section": "protein",
              "recipes": []
            },
            {
              "name": "dumplings",
              "section": "freezer",
              "recipes": []
            }
          ]
        }
        "###);
        Ok(())
    }

    #[test]
    fn test_add_groceries_item_and_add_recipe() -> Result<(), Box<dyn std::error::Error>> {
        let mut list = checklist();
        insta::assert_json_snapshot!(list, @r###"
        {
          "checklist": [],
          "recipes": [
            "tomato pasta"
          ],
          "items": [
            {
              "name": "garlic",
              "section": "fresh",
              "recipes": [
                "sheet pan salmon with broccoli",
                "crispy tofu with cashews and blistered snap peas",
                "chicken breasts with lemon",
                "hummus",
                "tomato pasta",
                "crispy sheet-pan noodles",
                "flue flighter chicken stew",
                "sheet-pan chicken with jammy tomatoes",
                "swordfish pasta"
              ]
            },
            {
              "name": "tomatoes",
              "section": "fresh",
              "recipes": [
                "tomato pasta"
              ]
            },
            {
              "name": "basil",
              "section": "fresh",
              "recipes": [
                "tomato pasta"
              ]
            },
            {
              "name": "lemons",
              "section": "fresh",
              "recipes": [
                "chicken breasts with lemon",
                "hummus",
                "sheet-pan chicken with jammy tomatoes",
                "flue flighter chicken stew"
              ]
            },
            {
              "name": "pasta",
              "section": "pantry",
              "recipes": [
                "tomato pasta",
                "swordfish pasta"
              ]
            },
            {
              "name": "olive oil",
              "section": "pantry",
              "recipes": [
                "sheet pan salmon with broccoli",
                "chicken breasts with lemon",
                "hummus",
                "tomato pasta",
                "sheet-pan chicken with jammy tomatoes",
                "turkey meatballs",
                "swordfish pasta"
              ]
            },
            {
              "name": "short grain brown rice",
              "section": "pantry",
              "recipes": [
                "sheet pan salmon with broccoli",
                "flue flighter chicken stew"
              ]
            },
            {
              "name": "parmigiana",
              "section": "dairy",
              "recipes": [
                "tomato pasta",
                "turkey meatballs"
              ]
            },
            {
              "name": "eggs",
              "section": "dairy",
              "recipes": [
                "oatmeal chocolate chip cookies",
                "fried eggs for breakfast",
                "turkey meatballs"
              ]
            },
            {
              "name": "sausages",
              "section": "protein",
              "recipes": []
            },
            {
              "name": "dumplings",
              "section": "freezer",
              "recipes": []
            }
          ]
        }
        "###);

        let item = Item {
            name: ItemName::from("cumquats"),
            section: Some(Section::from("fresh")),
            recipes: Some(vec![RecipeName::from("cumquat chutney")]),
        };
        let recipe = RecipeName::from("cumquat chutney");
        list.add_item(item);
        list.add_recipe(recipe);
        insta::assert_json_snapshot!(list, @r###"
        {
          "checklist": [],
          "recipes": [
            "tomato pasta",
            "cumquat chutney"
          ],
          "items": [
            {
              "name": "garlic",
              "section": "fresh",
              "recipes": [
                "sheet pan salmon with broccoli",
                "crispy tofu with cashews and blistered snap peas",
                "chicken breasts with lemon",
                "hummus",
                "tomato pasta",
                "crispy sheet-pan noodles",
                "flue flighter chicken stew",
                "sheet-pan chicken with jammy tomatoes",
                "swordfish pasta"
              ]
            },
            {
              "name": "tomatoes",
              "section": "fresh",
              "recipes": [
                "tomato pasta"
              ]
            },
            {
              "name": "basil",
              "section": "fresh",
              "recipes": [
                "tomato pasta"
              ]
            },
            {
              "name": "lemons",
              "section": "fresh",
              "recipes": [
                "chicken breasts with lemon",
                "hummus",
                "sheet-pan chicken with jammy tomatoes",
                "flue flighter chicken stew"
              ]
            },
            {
              "name": "pasta",
              "section": "pantry",
              "recipes": [
                "tomato pasta",
                "swordfish pasta"
              ]
            },
            {
              "name": "olive oil",
              "section": "pantry",
              "recipes": [
                "sheet pan salmon with broccoli",
                "chicken breasts with lemon",
                "hummus",
                "tomato pasta",
                "sheet-pan chicken with jammy tomatoes",
                "turkey meatballs",
                "swordfish pasta"
              ]
            },
            {
              "name": "short grain brown rice",
              "section": "pantry",
              "recipes": [
                "sheet pan salmon with broccoli",
                "flue flighter chicken stew"
              ]
            },
            {
              "name": "parmigiana",
              "section": "dairy",
              "recipes": [
                "tomato pasta",
                "turkey meatballs"
              ]
            },
            {
              "name": "eggs",
              "section": "dairy",
              "recipes": [
                "oatmeal chocolate chip cookies",
                "fried eggs for breakfast",
                "turkey meatballs"
              ]
            },
            {
              "name": "sausages",
              "section": "protein",
              "recipes": []
            },
            {
              "name": "dumplings",
              "section": "freezer",
              "recipes": []
            },
            {
              "name": "cumquats",
              "section": "fresh",
              "recipes": [
                "cumquat chutney"
              ]
            }
          ]
        }
        "###);

        Ok(())
    }
}
