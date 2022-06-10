use crate::{GroceriesItem, ReadError, Recipe};
use serde::{Deserialize, Serialize};
use std::path::Path;

// used to serialize and deserialize the
// most recently saved list or to create a
// new grocery list that can be saved as JSON
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShoppingList {
    pub checklist: Vec<GroceriesItem>,
    pub recipes: Vec<Recipe>,
    pub groceries: Vec<GroceriesItem>,
}

impl Default for ShoppingList {
    fn default() -> Self {
        Self::new()
    }
}

impl ShoppingList {
    pub fn new() -> Self {
        Self::new_initialized()
    }

    fn new_initialized() -> Self {
        ShoppingList {
            checklist: vec![],
            recipes: vec![],
            groceries: vec![],
        }
    }

    pub fn from_path<P: AsRef<Path> + Copy>(path: P) -> Result<ShoppingList, ReadError> {
        let reader = crate::helpers::read(path)?;

        Ok(serde_json::from_reader(reader)?)
    }

    pub fn print(&self) {
        if !self.checklist.is_empty() {
            println!("Check if we need:");

            self.checklist.iter().for_each(|item| {
                println!("\t{}", item.name.0.to_lowercase());
            });
        }
        if !self.recipes.is_empty() {
            println!("recipes:");

            self.recipes.iter().for_each(|recipe| {
                println!("\t{}", recipe);
            });
        }
        if !self.groceries.is_empty() {
            println!("groceries:");

            self.groceries.iter().for_each(|item| {
                println!("\t{}", item.name.0.to_lowercase());
            });
        }
    }

    pub fn add_groceries_item(&mut self, item: GroceriesItem) {
        self.groceries.push(item)
    }

    pub fn add_recipe(&mut self, recipe: Recipe) {
        self.recipes.push(recipe)
    }

    pub fn to_json_string(&self) -> Result<String, ReadError> {
        Ok(serde_json::to_string(&self)?)
    }

    pub fn save(&self) -> Result<(), ReadError> {
        let json = self.to_json_string()?;
        crate::helpers::write("list.json", json)
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    use assert_fs::prelude::*;

    fn create_test_json_file() -> Result<assert_fs::NamedTempFile, Box<dyn std::error::Error>> {
        let file = assert_fs::NamedTempFile::new("test.json")?;
        file.write_str(
            r#"
            {"checklist":[],"recipes":["tomato pasta"],"groceries":[{"name":"garlic","section":"fresh","is_recipe_ingredient":true,"recipes":["Sheet Pan Salmon with Broccoli","crispy tofu with cashews and blistered snap peas","chicken breasts with lemon","hummus","tomato pasta","crispy sheet-pan noodles","flue flighter chicken stew","sheet-pan chicken with jammy tomatoes","swordfish pasta"]},{"name":"tomatoes","section":"fresh","is_recipe_ingredient":true,"recipes":["tomato pasta"]},{"name":"basil","section":"fresh","is_recipe_ingredient":true,"recipes":["tomato pasta"]},{"name":"lemons","section":"fresh","is_recipe_ingredient":true,"recipes":["chicken breasts with lemon","hummus","sheet-pan chicken with jammy tomatoes","flue flighter chicken stew"]},{"name":"pasta","section":"pantry","is_recipe_ingredient":true,"recipes":["tomato pasta","swordfish pasta"]},{"name":"olive oil","section":"pantry","is_recipe_ingredient":true,"recipes":["Sheet Pan Salmon with Broccoli","chicken breasts with lemon","hummus","tomato pasta","sheet-pan chicken with jammy tomatoes","turkey meatballs","swordfish pasta"]},{"name":"short grain brown rice","section":"pantry","is_recipe_ingredient":true,"recipes":["Sheet Pan Salmon with Broccoli","flue flighter chicken stew"]},{"name":"parmigiana","section":"dairy","is_recipe_ingredient":true,"recipes":["tomato pasta","turkey meatballs"]},{"name":"eggs","section":"dairy","is_recipe_ingredient":true,"recipes":["oatmeal chocolate chip cookies","fried eggs for breakfast","turkey meatballs"]},{"name":"sausages","section":"protein","is_recipe_ingredient":true,"recipes":[]},{"name":"dumplings","section":"freezer","is_recipe_ingredient":false,"recipes":[]}]}
            "#
        )?;
        Ok(file)
    }

    #[test]
    fn json_from_file() -> Result<(), Box<dyn std::error::Error>> {
        let file = create_test_json_file()?;
        let list = ShoppingList::from_path(file.path())?;

        insta::assert_json_snapshot!(list, @r###"
        {
          "checklist": [],
          "recipes": [
            "tomato pasta"
          ],
          "groceries": [
            {
              "name": "garlic",
              "section": "fresh",
              "is_recipe_ingredient": true,
              "recipes": [
                "Sheet Pan Salmon with Broccoli",
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
              "is_recipe_ingredient": true,
              "recipes": [
                "tomato pasta"
              ]
            },
            {
              "name": "basil",
              "section": "fresh",
              "is_recipe_ingredient": true,
              "recipes": [
                "tomato pasta"
              ]
            },
            {
              "name": "lemons",
              "section": "fresh",
              "is_recipe_ingredient": true,
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
              "is_recipe_ingredient": true,
              "recipes": [
                "tomato pasta",
                "swordfish pasta"
              ]
            },
            {
              "name": "olive oil",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "Sheet Pan Salmon with Broccoli",
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
              "is_recipe_ingredient": true,
              "recipes": [
                "Sheet Pan Salmon with Broccoli",
                "flue flighter chicken stew"
              ]
            },
            {
              "name": "parmigiana",
              "section": "dairy",
              "is_recipe_ingredient": true,
              "recipes": [
                "tomato pasta",
                "turkey meatballs"
              ]
            },
            {
              "name": "eggs",
              "section": "dairy",
              "is_recipe_ingredient": true,
              "recipes": [
                "oatmeal chocolate chip cookies",
                "fried eggs for breakfast",
                "turkey meatballs"
              ]
            },
            {
              "name": "sausages",
              "section": "protein",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "dumplings",
              "section": "freezer",
              "is_recipe_ingredient": false,
              "recipes": []
            }
          ]
        }
        "###);
        Ok(())
    }

    #[test]
    fn test_add_groceries_item_and_add_recipe() -> Result<(), Box<dyn std::error::Error>> {
        let file = create_test_json_file()?;
        let mut list = ShoppingList::from_path(file.path())?;
        insta::assert_json_snapshot!(list, @r###"
        {
          "checklist": [],
          "recipes": [
            "tomato pasta"
          ],
          "groceries": [
            {
              "name": "garlic",
              "section": "fresh",
              "is_recipe_ingredient": true,
              "recipes": [
                "Sheet Pan Salmon with Broccoli",
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
              "is_recipe_ingredient": true,
              "recipes": [
                "tomato pasta"
              ]
            },
            {
              "name": "basil",
              "section": "fresh",
              "is_recipe_ingredient": true,
              "recipes": [
                "tomato pasta"
              ]
            },
            {
              "name": "lemons",
              "section": "fresh",
              "is_recipe_ingredient": true,
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
              "is_recipe_ingredient": true,
              "recipes": [
                "tomato pasta",
                "swordfish pasta"
              ]
            },
            {
              "name": "olive oil",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "Sheet Pan Salmon with Broccoli",
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
              "is_recipe_ingredient": true,
              "recipes": [
                "Sheet Pan Salmon with Broccoli",
                "flue flighter chicken stew"
              ]
            },
            {
              "name": "parmigiana",
              "section": "dairy",
              "is_recipe_ingredient": true,
              "recipes": [
                "tomato pasta",
                "turkey meatballs"
              ]
            },
            {
              "name": "eggs",
              "section": "dairy",
              "is_recipe_ingredient": true,
              "recipes": [
                "oatmeal chocolate chip cookies",
                "fried eggs for breakfast",
                "turkey meatballs"
              ]
            },
            {
              "name": "sausages",
              "section": "protein",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "dumplings",
              "section": "freezer",
              "is_recipe_ingredient": false,
              "recipes": []
            }
          ]
        }
        "###);

        let item = GroceriesItem {
            name: crate::GroceriesItemName("cumquats".to_string()),
            section: crate::GroceriesItemSection("fresh".to_string()),
            is_recipe_ingredient: true,
            recipes: vec![Recipe("cumquat chutney".to_string())],
        };
        let recipe = Recipe("cumquat chutney".to_string());
        list.add_groceries_item(item);
        list.add_recipe(recipe);
        insta::assert_json_snapshot!(list, @r###"
        {
          "checklist": [],
          "recipes": [
            "tomato pasta",
            "cumquat chutney"
          ],
          "groceries": [
            {
              "name": "garlic",
              "section": "fresh",
              "is_recipe_ingredient": true,
              "recipes": [
                "Sheet Pan Salmon with Broccoli",
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
              "is_recipe_ingredient": true,
              "recipes": [
                "tomato pasta"
              ]
            },
            {
              "name": "basil",
              "section": "fresh",
              "is_recipe_ingredient": true,
              "recipes": [
                "tomato pasta"
              ]
            },
            {
              "name": "lemons",
              "section": "fresh",
              "is_recipe_ingredient": true,
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
              "is_recipe_ingredient": true,
              "recipes": [
                "tomato pasta",
                "swordfish pasta"
              ]
            },
            {
              "name": "olive oil",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "Sheet Pan Salmon with Broccoli",
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
              "is_recipe_ingredient": true,
              "recipes": [
                "Sheet Pan Salmon with Broccoli",
                "flue flighter chicken stew"
              ]
            },
            {
              "name": "parmigiana",
              "section": "dairy",
              "is_recipe_ingredient": true,
              "recipes": [
                "tomato pasta",
                "turkey meatballs"
              ]
            },
            {
              "name": "eggs",
              "section": "dairy",
              "is_recipe_ingredient": true,
              "recipes": [
                "oatmeal chocolate chip cookies",
                "fried eggs for breakfast",
                "turkey meatballs"
              ]
            },
            {
              "name": "sausages",
              "section": "protein",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "dumplings",
              "section": "freezer",
              "is_recipe_ingredient": false,
              "recipes": []
            },
            {
              "name": "cumquats",
              "section": "fresh",
              "is_recipe_ingredient": true,
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
