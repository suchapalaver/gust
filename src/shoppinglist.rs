use crate::{GroceriesItem, GroceriesItemName, ReadError, Recipe, helpers};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct ShoppingList {
    pub checklist: Vec<GroceriesItem>,
    pub recipes: Vec<Recipe>,
    pub groceries: Vec<GroceriesItem>,
}

impl ShoppingList {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_path<P: AsRef<Path> + Copy>(path: P) -> Result<ShoppingList, ReadError> {
        let reader = helpers::read(path)?;

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

    pub fn delete_groceries_item(&mut self, name: &str) -> Result<(), ReadError> {
        if let Ok(i) = self
            .groceries
            .iter()
            .position(|x| x.name == GroceriesItemName(name.to_string()))
            .ok_or(ReadError::ItemNotFound)
        {
            self.groceries.remove(i);
        }
        Ok(())
    }

    pub fn add_checklist_item(&mut self, item: GroceriesItem) {
        self.checklist.push(item)
    }

    pub fn delete_checklist_item(&mut self, name: &str) -> Result<(), ReadError> {
        if let Ok(i) = self
            .checklist
            .iter()
            .position(|x| x.name == GroceriesItemName(name.to_string()))
            .ok_or(ReadError::ItemNotFound)
        {
            self.checklist.remove(i);
        }
        Ok(())
    }

    pub fn add_recipe(&mut self, recipe: Recipe) {
        self.recipes.push(recipe)
    }

    pub fn delete_recipe(&mut self, name: &str) -> Result<(), ReadError> {
        if let Ok(i) = self
            .recipes
            .iter()
            .position(|x| x == &Recipe(name.to_string()))
            .ok_or(ReadError::ItemNotFound)
        {
            self.recipes.remove(i);
        }
        Ok(())
    }

    pub fn to_json_string(&self) -> Result<String, ReadError> {
        Ok(serde_json::to_string(&self)?)
    }

    pub fn save(&self) -> Result<(), ReadError> {
        let json = self.to_json_string()?;
        helpers::write("list.json", &json)
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    use crate::GroceriesItemSection;
    use assert_fs::prelude::*;

    // test suite helper function
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
    fn test_delete_groceries_item() -> Result<(), Box<dyn std::error::Error>> {
        let file = create_test_json_file()?;
        let mut shopping_list = ShoppingList::from_path(file.path())?;
        let item = GroceriesItem {
            name: GroceriesItemName("kumquats".to_string()),
            section: GroceriesItemSection("fresh".to_string()),
            is_recipe_ingredient: false,
            recipes: vec![],
        };
        shopping_list.add_groceries_item(item);
        insta::assert_json_snapshot!(shopping_list.groceries, @r###"
        [
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
            "name": "kumquats",
            "section": "fresh",
            "is_recipe_ingredient": false,
            "recipes": []
          }
        ]
        "###);
        shopping_list.delete_groceries_item("kumquats")?;
        insta::assert_json_snapshot!(shopping_list.groceries, @r###"
        [
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
        "###);
        Ok(())
    }

    #[test]
    fn test_delete_checklist_item() -> Result<(), Box<dyn std::error::Error>> {
        let file = create_test_json_file()?;
        let mut shopping_list = ShoppingList::from_path(file.path())?;
        let item = GroceriesItem {
            name: GroceriesItemName("kumquats".to_string()),
            section: GroceriesItemSection("fresh".to_string()),
            is_recipe_ingredient: false,
            recipes: vec![],
        };
        shopping_list.add_checklist_item(item);
        insta::assert_json_snapshot!(shopping_list.checklist, @r###"
        [
          {
            "name": "kumquats",
            "section": "fresh",
            "is_recipe_ingredient": false,
            "recipes": []
          }
        ]
        "###);
        shopping_list.delete_checklist_item("kumquats")?;
        insta::assert_json_snapshot!(shopping_list.checklist, @"[]");
        Ok(())
    }

    #[test]
    fn test_delete_recipe() -> Result<(), Box<dyn std::error::Error>> {
        let file = create_test_json_file()?;
        let mut shopping_list = ShoppingList::from_path(file.path())?;
        insta::assert_json_snapshot!(shopping_list.recipes, @r###"
        [
          "tomato pasta"
        ]
        "###);
        shopping_list.delete_recipe("tomato pasta")?;
        insta::assert_json_snapshot!(shopping_list.recipes, @"[]");
        Ok(())
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
