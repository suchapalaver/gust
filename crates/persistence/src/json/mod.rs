pub mod migrate;

use std::{
    collections::HashSet,
    fs::{self},
    path::{Path, PathBuf},
};

use common::{
    input::item_matches,
    item::{Item, Name},
    items::Items,
    list::List,
    recipes::{Ingredients, Recipe},
    Load,
};
use question::Answer;

use crate::store::{Storage, StoreError, StoreResponse};

pub const ITEMS_JSON_PATH: &str = "groceries.json";

pub const LIST_JSON_PATH: &str = "list.json";

#[derive(Clone)]
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
    async fn add_item(&self, item: &Name) -> Result<StoreResponse, StoreError> {
        let mut groceries = Items::from_json(&self.items)?;

        if groceries
            .get_item_matches(item.as_str())
            .any(|item| matches!(item_matches(item), Answer::YES))
        {
            eprintln!("Item already in library");
            Ok(StoreResponse::ItemAlreadyAdded(item.clone()))
        } else {
            let new_item = Item::new(item.as_str());
            groceries.add_item(new_item);
            Ok(StoreResponse::AddedItem(item.clone()))
        }
    }

    async fn add_checklist_item(&self, _item: &Name) -> Result<StoreResponse, StoreError> {
        todo!()
    }

    async fn add_list_item(&self, _item: &Name) -> Result<StoreResponse, StoreError> {
        todo!()
    }

    async fn add_list_recipe(&self, _recipe: &Recipe) -> Result<StoreResponse, StoreError> {
        todo!()
    }

    async fn add_recipe(
        &self,
        _recipe: &Recipe,
        _ingredients: &Ingredients,
    ) -> Result<StoreResponse, StoreError> {
        todo!()
    }

    async fn checklist(&self) -> Result<StoreResponse, StoreError> {
        todo!()
    }

    async fn delete_checklist_item(&self, _item: &Name) -> Result<StoreResponse, StoreError> {
        todo!()
    }

    async fn delete_recipe(&self, _recipe: &Recipe) -> Result<StoreResponse, StoreError> {
        todo!()
    }

    async fn items(&self) -> Result<StoreResponse, StoreError> {
        Ok(StoreResponse::Items(Items::from_json(&self.items)?))
    }

    async fn list(&self) -> Result<StoreResponse, StoreError> {
        Ok(StoreResponse::List(List::from_json(&self.list)?))
    }

    async fn refresh_list(&self) -> Result<StoreResponse, StoreError> {
        todo!()
    }

    async fn recipe_ingredients(&self, recipe: &Recipe) -> Result<StoreResponse, StoreError> {
        let items = Items::from_json(&self.items)?;
        let ingredients: Ingredients = items
            .recipe_ingredients(&recipe.to_string())?
            .map(|item| item.name())
            .cloned()
            .collect();

        Ok(StoreResponse::RecipeIngredients(Some(ingredients)))
    }

    async fn sections(&self) -> Result<StoreResponse, StoreError> {
        todo!()
    }

    async fn recipes(&self) -> Result<StoreResponse, StoreError> {
        let mut recipes: HashSet<Recipe> = HashSet::new();

        {
            let groceries = Items::from_json(&self.items)?;

            for item in groceries.collection() {
                if let Some(item_recipes) = item.recipes() {
                    for recipe in item_recipes.iter().cloned() {
                        recipes.insert(recipe);
                    }
                }
            }

            for recipe in groceries.recipes().cloned() {
                recipes.insert(recipe);
            }
        }

        {
            let list = List::from_json(&self.list)?;

            for recipe in list.recipes().cloned() {
                recipes.insert(recipe);
            }
        }
        Ok(StoreResponse::Recipes(recipes.into_iter().collect()))
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    use assert_fs::prelude::*;

    fn test_json_file() -> Result<assert_fs::NamedTempFile, Box<dyn std::error::Error>> {
        let file = assert_fs::NamedTempFile::new("test1.json")?;
        file.write_str(
            r#"
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
                          "fried eggs for breakfast"
                      ]
                  },
                  {
                      "name": "milk",
                      "section": "dairy",
                      "recipes": []
                  },
                  {
                      "name": "spinach",
                      "section": "fresh",
                      "recipes": [
                          "fried eggs for breakfast"
                      ]
                  },
                  {
                      "name": "beer",
                      "section": "dairy",
                      "recipes": []
                  },
                  {
                      "name": "unsalted butter",
                      "section": "dairy",
                      "recipes": [
                          "oatmeal chocolate chip cookies",
                          "fried eggs for breakfast"
                      ]
                  },
                  {
                      "name": "bread",
                      "section": "pantry",
                      "recipes": [
                          "fried eggs for breakfast"
                      ]
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
                          "oatmeal chocolate chip cookies"
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
                  }
              ],
              "recipes": [
                  "oatmeal chocolate chip cookies",
                  "fried eggs for breakfast"
              ]
          }"#,
        )?;
        Ok(file)
    }

    async fn items() -> Items {
        let file = test_json_file().unwrap();
        let store = JsonStore::new().with_items_path(file.path());
        let StoreResponse::Items(items) = store.items().await.unwrap() else {
            todo!()
        };
        items
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

    #[tokio::test]
    async fn test_save_items() -> Result<(), Box<dyn std::error::Error>> {
        let store = JsonStore::new().with_items_path(&PathBuf::from("test_groceries.json"));
        let items = Items::default();
        insta::assert_json_snapshot!(items, @r#"
    {
      "sections": [],
      "collection": [],
      "recipes": []
    }
    "#);
        store.save_items(items)?;
        let StoreResponse::Items(items) = store.items().await.unwrap() else {
            todo!()
        };
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

    #[tokio::test]
    async fn test_delete_recipe() -> Result<(), Box<dyn std::error::Error>> {
        let mut items = items().await;
        insta::assert_json_snapshot!(items.recipes().collect::<Vec<&Recipe>>(), @r###"
        [
          "oatmeal chocolate chip cookies",
          "fried eggs for breakfast"
        ]
        "###);
        items.delete_recipe("oatmeal chocolate chip cookies")?;
        insta::assert_json_snapshot!(items.recipes().collect::<Vec<&Recipe>>(), @r###"
        [
          "fried eggs for breakfast"
        ]
        "###);
        Ok(())
    }

    #[tokio::test]
    async fn test_delete_item() -> Result<(), Box<dyn std::error::Error>> {
        let mut items = items().await;
        insta::assert_json_snapshot!(items.collection().collect::<Vec<&Item>>(), @r###"
        [
          {
            "name": "eggs",
            "section": "dairy",
            "recipes": [
              "oatmeal chocolate chip cookies",
              "fried eggs for breakfast"
            ]
          },
          {
            "name": "milk",
            "section": "dairy",
            "recipes": []
          },
          {
            "name": "spinach",
            "section": "fresh",
            "recipes": [
              "fried eggs for breakfast"
            ]
          },
          {
            "name": "beer",
            "section": "dairy",
            "recipes": []
          },
          {
            "name": "unsalted butter",
            "section": "dairy",
            "recipes": [
              "oatmeal chocolate chip cookies",
              "fried eggs for breakfast"
            ]
          },
          {
            "name": "bread",
            "section": "pantry",
            "recipes": [
              "fried eggs for breakfast"
            ]
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
              "oatmeal chocolate chip cookies"
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
          }
        ]
        "###);
        items.delete_item("eggs")?;
        insta::assert_json_snapshot!(items.collection().collect::<Vec<&Item>>(), @r###"
        [
          {
            "name": "milk",
            "section": "dairy",
            "recipes": []
          },
          {
            "name": "spinach",
            "section": "fresh",
            "recipes": [
              "fried eggs for breakfast"
            ]
          },
          {
            "name": "beer",
            "section": "dairy",
            "recipes": []
          },
          {
            "name": "unsalted butter",
            "section": "dairy",
            "recipes": [
              "oatmeal chocolate chip cookies",
              "fried eggs for breakfast"
            ]
          },
          {
            "name": "bread",
            "section": "pantry",
            "recipes": [
              "fried eggs for breakfast"
            ]
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
              "oatmeal chocolate chip cookies"
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
          }
        ]
        "###);
        Ok(())
    }

    #[tokio::test]
    async fn test_groceries() -> Result<(), Box<dyn std::error::Error>> {
        let mut items = items().await;
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
                "fried eggs for breakfast"
              ]
            },
            {
              "name": "milk",
              "section": "dairy",
              "recipes": []
            },
            {
              "name": "spinach",
              "section": "fresh",
              "recipes": [
                "fried eggs for breakfast"
              ]
            },
            {
              "name": "beer",
              "section": "dairy",
              "recipes": []
            },
            {
              "name": "unsalted butter",
              "section": "dairy",
              "recipes": [
                "oatmeal chocolate chip cookies",
                "fried eggs for breakfast"
              ]
            },
            {
              "name": "bread",
              "section": "pantry",
              "recipes": [
                "fried eggs for breakfast"
              ]
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
                "oatmeal chocolate chip cookies"
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
            }
          ],
          "recipes": [
            "oatmeal chocolate chip cookies",
            "fried eggs for breakfast"
          ]
        }
        "###);

        let recipe = "cumquat chutney";

        let item = Item::new("cumquats")
            .with_section("fresh")
            .with_recipes(&[Recipe::from(recipe)]);

        let ingredients = "kumquats, carrots, dried apricots, dried cranberries, chili, onion, garlic, cider vinegar, granulated sugar, honey, kosher salt, cardamom, cloves, coriander, ginger, black peppercorns";

        items.add_item(item);
        items.add_recipe(recipe, ingredients)?;

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
                "fried eggs for breakfast"
              ]
            },
            {
              "name": "milk",
              "section": "dairy",
              "recipes": []
            },
            {
              "name": "spinach",
              "section": "fresh",
              "recipes": [
                "fried eggs for breakfast"
              ]
            },
            {
              "name": "beer",
              "section": "dairy",
              "recipes": []
            },
            {
              "name": "unsalted butter",
              "section": "dairy",
              "recipes": [
                "oatmeal chocolate chip cookies",
                "fried eggs for breakfast"
              ]
            },
            {
              "name": "bread",
              "section": "pantry",
              "recipes": [
                "fried eggs for breakfast"
              ]
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
                "oatmeal chocolate chip cookies"
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
              "name": "cumquats",
              "section": "fresh",
              "recipes": [
                "cumquat chutney"
              ]
            }
          ],
          "recipes": [
            "oatmeal chocolate chip cookies",
            "fried eggs for breakfast",
            "cumquat chutney"
          ]
        }
        "###);

        Ok(())
    }

    #[tokio::test]
    async fn test_delete_item_from_list() -> Result<(), Box<dyn std::error::Error>> {
        let file = create_test_checklist_json_file().unwrap();
        let store = JsonStore::new().with_list_path(file.path());

        let StoreResponse::List(mut shopping_list) = store.list().await.unwrap() else {
            todo!()
        };

        let item = Item::new("kumquats").with_section("fresh");

        shopping_list.add_item(item);
        insta::assert_json_snapshot!(shopping_list.items(), @r###"
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
        insta::assert_json_snapshot!(shopping_list.items(), @r###"
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

    async fn checklist() -> List {
        let file = create_test_checklist_json_file().unwrap();
        let store = JsonStore::new().with_list_path(file.path());
        let StoreResponse::List(list) = store.list().await.unwrap() else {
            todo!()
        };
        list
    }

    #[tokio::test]
    async fn test_delete_checklist_item() -> Result<(), Box<dyn std::error::Error>> {
        let mut shopping_list = checklist().await;
        let item = Item::new("kumquats").with_section("fresh");
        shopping_list.add_checklist_item(item);
        insta::assert_json_snapshot!(shopping_list.checklist(), @r###"
        [
          {
            "name": "kumquats",
            "section": "fresh",
            "recipes": null
          }
        ]
        "###);
        shopping_list.delete_checklist_item("kumquats")?;
        insta::assert_json_snapshot!(shopping_list.checklist(), @"[]");
        Ok(())
    }

    #[tokio::test]
    async fn test_delete_recipe_from_list() -> Result<(), Box<dyn std::error::Error>> {
        let mut shopping_list = checklist().await;
        insta::assert_json_snapshot!(shopping_list.recipes().collect::<Vec<&Recipe>>(), @r#"
        [
          "tomato pasta"
        ]
        "#);
        shopping_list.delete_recipe("tomato pasta")?;
        insta::assert_json_snapshot!(shopping_list.recipes().collect::<Vec<&Recipe>>(), @"[]");
        Ok(())
    }

    #[tokio::test]
    async fn json_from_file() -> Result<(), Box<dyn std::error::Error>> {
        let list = checklist().await;

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

    #[tokio::test]
    async fn test_add_groceries_item_and_add_recipe() -> Result<(), Box<dyn std::error::Error>> {
        let mut list = checklist().await;
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

        let recipe = Recipe::from("cumquat chutney");

        let item = Item::new("cumquats")
            .with_section("fresh")
            .with_recipes(&[recipe.clone()]);

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
