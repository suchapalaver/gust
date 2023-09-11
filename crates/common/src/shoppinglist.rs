use crate::{
    errors::ReadError,
    groceriesitem::{Item, ItemName},
    helpers::ReadWrite,
    recipes::RecipeName,
};
use serde::{Deserialize, Serialize};

pub const LIST_JSON_PATH: &str = "list.json";

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct ShoppingList {
    pub checklist: Vec<Item>,
    pub recipes: Vec<RecipeName>,
    pub items: Vec<Item>,
}

impl ReadWrite for ShoppingList {}

impl ShoppingList {
    pub fn new() -> Self {
        Self::default()
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
        if !self.items.is_empty() {
            println!("groceries:");

            self.items.iter().for_each(|item| {
                println!("\t{}", item.name.0.to_lowercase());
            });
        }
    }

    pub fn add_groceries_item(&mut self, item: Item) {
        self.items.push(item)
    }

    pub fn delete_groceries_item(&mut self, name: &str) -> Result<(), ReadError> {
        if let Ok(i) = self
            .items
            .iter()
            .position(|x| x.name == ItemName(name.to_string()))
            .ok_or(ReadError::ItemNotFound)
        {
            self.items.remove(i);
        }
        Ok(())
    }

    pub fn add_checklist_item(&mut self, item: Item) {
        self.checklist.push(item)
    }

    pub fn delete_checklist_item(&mut self, name: &str) -> Result<(), ReadError> {
        if let Ok(i) = self
            .checklist
            .iter()
            .position(|x| x.name == ItemName(name.to_string()))
            .ok_or(ReadError::ItemNotFound)
        {
            self.checklist.remove(i);
        }
        Ok(())
    }

    pub fn add_recipe(&mut self, recipe: RecipeName) {
        self.recipes.push(recipe)
    }

    pub fn delete_recipe(&mut self, name: &str) -> Result<(), ReadError> {
        if let Ok(i) = self
            .recipes
            .iter()
            .position(|x| x == &RecipeName(name.to_string()))
            .ok_or(ReadError::ItemNotFound)
        {
            self.recipes.remove(i);
        }
        Ok(())
    }

    pub fn to_json_string(&self) -> Result<String, ReadError> {
        Ok(serde_json::to_string(&self)?)
    }
}

#[cfg(test)]
pub mod test {
    use crate::groceriesitem::Section;

    use super::*;

    use assert_fs::prelude::*;

    fn create_test_json_file() -> Result<assert_fs::NamedTempFile, Box<dyn std::error::Error>> {
        let file = assert_fs::NamedTempFile::new("test.json")?;
        file.write_str(
            r#"
            {"checklist":[],"recipes":["tomato pasta"],"items":[{"name":"garlic","section":"fresh","is_ingredient":true,"recipes":["Sheet Pan Salmon with Broccoli","crispy tofu with cashews and blistered snap peas","chicken breasts with lemon","hummus","tomato pasta","crispy sheet-pan noodles","flue flighter chicken stew","sheet-pan chicken with jammy tomatoes","swordfish pasta"]},{"name":"tomatoes","section":"fresh","is_ingredient":true,"recipes":["tomato pasta"]},{"name":"basil","section":"fresh","is_ingredient":true,"recipes":["tomato pasta"]},{"name":"lemons","section":"fresh","is_ingredient":true,"recipes":["chicken breasts with lemon","hummus","sheet-pan chicken with jammy tomatoes","flue flighter chicken stew"]},{"name":"pasta","section":"pantry","is_ingredient":true,"recipes":["tomato pasta","swordfish pasta"]},{"name":"olive oil","section":"pantry","is_ingredient":true,"recipes":["Sheet Pan Salmon with Broccoli","chicken breasts with lemon","hummus","tomato pasta","sheet-pan chicken with jammy tomatoes","turkey meatballs","swordfish pasta"]},{"name":"short grain brown rice","section":"pantry","is_ingredient":true,"recipes":["Sheet Pan Salmon with Broccoli","flue flighter chicken stew"]},{"name":"parmigiana","section":"dairy","is_ingredient":true,"recipes":["tomato pasta","turkey meatballs"]},{"name":"eggs","section":"dairy","is_ingredient":true,"recipes":["oatmeal chocolate chip cookies","fried eggs for breakfast","turkey meatballs"]},{"name":"sausages","section":"protein","is_ingredient":true,"recipes":[]},{"name":"dumplings","section":"freezer","is_ingredient":false,"recipes":[]}]}
            "#
        )?;
        Ok(file)
    }

    #[test]
    fn test_delete_groceries_item() -> Result<(), Box<dyn std::error::Error>> {
        let file = create_test_json_file()?;
        let mut shopping_list = ShoppingList::from_path(file.path())?;
        let item = Item {
            name: ItemName("kumquats".to_string()),
            section: Some(Section("fresh".to_string())),
            recipes: None,
        };
        shopping_list.add_groceries_item(item);
        insta::assert_json_snapshot!(shopping_list.items, @r###"
        [
          {
            "name": "garlic",
            "section": "fresh",
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
            "recipes": [
              "Sheet Pan Salmon with Broccoli",
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
            "recipes": [
              "Sheet Pan Salmon with Broccoli",
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

    #[test]
    fn test_delete_checklist_item() -> Result<(), Box<dyn std::error::Error>> {
        let file = create_test_json_file()?;
        let mut shopping_list = ShoppingList::from_path(file.path())?;
        let item = Item {
            name: ItemName("kumquats".to_string()),
            section: Some(Section("fresh".to_string())),
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
    fn test_delete_recipe() -> Result<(), Box<dyn std::error::Error>> {
        let file = create_test_json_file()?;
        let mut shopping_list = ShoppingList::from_path(file.path())?;
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
        let file = create_test_json_file()?;
        let list = ShoppingList::from_path(file.path())?;

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
              "recipes": [
                "Sheet Pan Salmon with Broccoli",
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
        let file = create_test_json_file()?;
        let mut list = ShoppingList::from_path(file.path())?;
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
              "recipes": [
                "Sheet Pan Salmon with Broccoli",
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
            name: ItemName("cumquats".to_string()),
            section: Some(Section("fresh".to_string())),
            recipes: Some(vec![RecipeName("cumquat chutney".to_string())]),
        };
        let recipe = RecipeName("cumquat chutney".to_string());
        list.add_groceries_item(item);
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
              "recipes": [
                "Sheet Pan Salmon with Broccoli",
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
