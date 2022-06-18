use crate::{LookupError, ReadError};
use serde::{Deserialize, Serialize};
use std::{fmt, fs, ops::Deref, path::Path, str::FromStr};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Groceries {
    pub sections: Vec<GroceriesItemSection>,
    pub collection: Vec<GroceriesItem>,
    pub recipes: Vec<Recipe>,
    //pub recipes_on_shoppinglist: Vec<Recipe>,
}

impl Groceries {
    pub fn new_initialized(path: &str) -> Result<Self, ReadError> {
        let g = Groceries {
            sections: vec![],
            collection: vec![],
            recipes: vec![],
        };
        let s = serde_json::to_string(&g)?;
        fs::write(path, s)?;
        Ok(g)
    }

    pub fn from_path<P: AsRef<Path> + Copy>(path: P) -> Result<Groceries, ReadError> {
        Ok(serde_json::from_reader(crate::read(path)?)?)
    }

    pub fn add_item(&mut self, item: GroceriesItem) {
        self.collection.push(item);
    }

    pub fn delete_item(&mut self, name: &str) -> Result<(), LookupError> {
        if let Ok(i) = self
            .collection
            .iter()
            .position(|x| x.name == GroceriesItemName(name.to_string()))
            .ok_or(LookupError::ItemNotFound)
        {
            self.collection.remove(i);
        }
        Ok(())
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), ReadError> {
        let s = serde_json::to_string(&self)?;
        Ok(fs::write(path, s)?)
    }

    pub fn print_groceries(&self) {
        for sec in &self.sections {
            let sec_items = self
                .collection
                .iter()
                .filter(|x| x.section.0.contains(&sec.0))
                .collect::<Vec<&GroceriesItem>>();
            for item in sec_items {
                eprintln!("{}", item);
            }
            eprintln!();
        }
    }

    pub fn print_recipes(&self) {
        for recipe in self.recipes.iter() {
            eprintln!("{}", recipe);
        }
    }

    pub fn add_recipe(&mut self, recipe: Recipe, ingredients: Ingredients) {
        self.collection
            .iter_mut()
            .filter(|x| ingredients.contains(&x.name))
            .for_each(|mut x| {
                if !x.is_recipe_ingredient {
                    x.is_recipe_ingredient = true;
                }
                x.recipes.push(recipe.clone());
            });
        self.recipes.push(recipe);
    }

    pub fn delete_recipe(&mut self, name: &str) -> Result<(), LookupError> {
        if let Ok(i) = self
            .recipes
            .iter()
            .position(|Recipe(x)| x.as_str() == name)
            .ok_or(LookupError::ItemNotFound)
        {
            self.recipes.remove(i);
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GroceriesItem {
    pub name: GroceriesItemName,       // e.g. "apples"
    pub section: GroceriesItemSection, // e.g. "fresh"
    pub is_recipe_ingredient: bool,    // i.e. true
    pub recipes: Vec<Recipe>,          // list of recipes: "apple pie", "cheese plate", ...
                                       //pub on_list: bool,
                                       //pub on_checklist: bool,
}

impl GroceriesItem {
    pub fn new(name: &str, section: &str) -> Result<Option<Self>, ReadError> {
        let groceries = Groceries::from_path("groceries.json")?;

        // check if there are no matches
        if groceries.collection.iter().all(|item| !item.matches(name)) {
            // if no matches add the item to groceries
            Ok(Some(Self::new_initialized(
                GroceriesItemName(name.to_owned()),
                GroceriesItemSection(section.to_owned()),
            )))
        } else {
            // check any matches for a genuine match,
            // e.g. 'instant ramen noodles' is a genuine match for 'ramen noodles'
            // (in our case, at least)
            let mut found_no_matches = true;
            for item in groceries.collection.iter() {
                if item.matches(name) {
                    eprintln!(
                        "is *{}* a match?\n\
			                *y* for yes
			                *any other key* for no",
                        item
                    );
                    if crate::prompt_for_y()? {
                        found_no_matches = false;
                        break;
                    }
                }
            }
            if found_no_matches {
                // means we need to add the item to groceries afterall
                // after we had to check for any fake matches above
                Ok(Some(Self::new_initialized(
                    GroceriesItemName(name.to_owned()),
                    GroceriesItemSection(section.to_owned()),
                )))
            } else {
                Ok(None)
            }
        }
    }

    pub fn new_initialized(name: GroceriesItemName, section: GroceriesItemSection) -> Self {
        //let name = name_and_section.get(0).expect("no grocery name found!");
        //let section = name_and_section.get(1).expect("no grocery section found");
        GroceriesItem {
            name,
            section,
            is_recipe_ingredient: false,
            recipes: vec![],
            //on_list: false,
            //on_checklist: false,
        }
    }

    fn matches(&self, s: &str) -> bool {
        s.split(' ').all(|word| !self.name.0.contains(word))
    }
}
/*
impl Default for GroceriesItem {
    fn default() -> Self {
        Self::new()
    }
}
*/

impl fmt::Display for GroceriesItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Deref for GroceriesItem {
    type Target = Vec<Recipe>;

    fn deref(&self) -> &Self::Target {
        &self.recipes
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GroceriesItemName(pub String);

impl std::fmt::Display for GroceriesItemName {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GroceriesItemSection(pub String);

impl fmt::Display for GroceriesItemSection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Ingredients(pub Vec<GroceriesItemName>);

impl Ingredients {
    fn new() -> Ingredients {
        Ingredients(Vec::new())
    }

    fn add(&mut self, elem: GroceriesItemName) {
        self.0.push(elem);
    }

    pub fn from_input_string(s: String) -> Result<Self, ReadError> {
        Ingredients::from_str(&s)
    }
}

impl FromIterator<GroceriesItemName> for Ingredients {
    fn from_iter<I: IntoIterator<Item = GroceriesItemName>>(iter: I) -> Self {
        let mut c = Ingredients::new();

        for i in iter {
            c.add(i);
        }
        c
    }
}

impl FromStr for Ingredients {
    type Err = crate::errors::ReadError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.split(',')
            .map(|item| GroceriesItemName(item.trim().to_lowercase()))
            .collect())
    }
}

impl Deref for Ingredients {
    type Target = Vec<GroceriesItemName>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Recipe(pub String);

impl fmt::Display for Recipe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Recipe {
    pub fn new(s: String) -> Result<Self, ReadError> {
        Recipe::from_str(&s)
    }
}

impl FromStr for Recipe {
    type Err = ReadError;

    fn from_str(s: &str) -> Result<Self, ReadError> {
        Ok(Recipe(s.to_string()))
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
            {"sections":["fresh","pantry","protein","dairy","freezer"],"collection":[{"name":"eggs","section":"dairy","is_recipe_ingredient":true,"recipes":["oatmeal chocolate chip cookies","fried eggs for breakfast","turkey meatballs"]},{"name":"milk","section":"dairy","is_recipe_ingredient":true,"recipes":[]},{"name":"lemons","section":"fresh","is_recipe_ingredient":true,"recipes":["chicken breasts with lemon","hummus","sheet-pan chicken with jammy tomatoes","flue flighter chicken stew"]},{"name":"ginger","section":"fresh","is_recipe_ingredient":true,"recipes":["Sheet Pan Salmon with Broccoli"]},{"name":"spinach","section":"fresh","is_recipe_ingredient":true,"recipes":["fried eggs for breakfast","flue flighter chicken stew"]},{"name":"garlic","section":"fresh","is_recipe_ingredient":true,"recipes":["Sheet Pan Salmon with Broccoli","crispy tofu with cashews and blistered snap peas","chicken breasts with lemon","hummus","tomato pasta","crispy sheet-pan noodles","flue flighter chicken stew","sheet-pan chicken with jammy tomatoes","swordfish pasta"]},{"name":"yellow onion","section":"fresh","is_recipe_ingredient":true,"recipes":["flue flighter chicken stew"]},{"name":"fizzy water","section":"dairy","is_recipe_ingredient":false,"recipes":[]},{"name":"kale","section":"fresh","is_recipe_ingredient":true,"recipes":[]},{"name":"beer","section":"dairy","is_recipe_ingredient":false,"recipes":[]},{"name":"parsley","section":"fresh","is_recipe_ingredient":true,"recipes":["turkey meatballs","flue flighter chicken stew","sheet-pan chicken with jammy tomatoes","swordfish pasta"]},{"name":"kefir","section":"dairy","is_recipe_ingredient":false,"recipes":[]},{"name":"kimchi","section":"dairy","is_recipe_ingredient":false,"recipes":[]},{"name":"sour cream","section":"dairy","is_recipe_ingredient":true,"recipes":[]},{"name":"potatoes","section":"fresh","is_recipe_ingredient":true,"recipes":[]},{"name":"broccoli","section":"fresh","is_recipe_ingredient":true,"recipes":["Sheet Pan Salmon with Broccoli"]},{"name":"asparagus","section":"fresh","is_recipe_ingredient":true,"recipes":[]},{"name":"dill","section":"fresh","is_recipe_ingredient":true,"recipes":[]},{"name":"red onion","section":"fresh","is_recipe_ingredient":true,"recipes":[]},{"name":"unsalted butter","section":"dairy","is_recipe_ingredient":true,"recipes":["chicken breasts with lemon","oatmeal chocolate chip cookies","fried eggs for breakfast"]},{"name":"scallions","section":"fresh","is_recipe_ingredient":true,"recipes":["Sheet Pan Salmon with Broccoli","crispy tofu with cashews and blistered snap peas"]},{"name":"mozzarella","section":"dairy","is_recipe_ingredient":true,"recipes":[]},{"name":"cucumbers","section":"fresh","is_recipe_ingredient":true,"recipes":[]},{"name":"greek yogurt","section":"dairy","is_recipe_ingredient":true,"recipes":[]},{"name":"cream cheese","section":"dairy","is_recipe_ingredient":true,"recipes":[]},{"name":"sweet potato","section":"fresh","is_recipe_ingredient":false,"recipes":[]},{"name":"sausages","section":"protein","is_recipe_ingredient":true,"recipes":[]},{"name":"tofu","section":"protein","is_recipe_ingredient":true,"recipes":["crispy tofu with cashews and blistered snap peas","crispy sheet-pan noodles"]},{"name":"short grain brown rice","section":"pantry","is_recipe_ingredient":true,"recipes":["Sheet Pan Salmon with Broccoli","flue flighter chicken stew"]},{"name":"tahini","section":"pantry","is_recipe_ingredient":true,"recipes":["hummus"]},{"name":"chicken stock","section":"pantry","is_recipe_ingredient":true,"recipes":[]},{"name":"orzo","section":"pantry","is_recipe_ingredient":true,"recipes":[]},{"name":"pasta","section":"pantry","is_recipe_ingredient":true,"recipes":["tomato pasta","swordfish pasta"]},{"name":"bread","section":"pantry","is_recipe_ingredient":true,"recipes":["fried eggs for breakfast","peanut butter and jelly on toast","turkey and cheese sandwiches"]},{"name":"coffee","section":"pantry","is_recipe_ingredient":true,"recipes":[]},{"name":"cumin","section":"pantry","is_recipe_ingredient":true,"recipes":[]},{"name":"coconut milk (unsweetened)","section":"pantry","is_recipe_ingredient":true,"recipes":["crispy tofu with cashews and blistered snap peas"]},{"name":"tortilla chips","section":"pantry","is_recipe_ingredient":true,"recipes":[]},{"name":"Ritz crackers","section":"pantry","is_recipe_ingredient":true,"recipes":[]},{"name":"black beans","section":"pantry","is_recipe_ingredient":true,"recipes":[]},{"name":"mustard","section":"pantry","is_recipe_ingredient":true,"recipes":["turkey and cheese sandwiches"]},{"name":"chips","section":"pantry","is_recipe_ingredient":true,"recipes":[]},{"name":"popcorn","section":"pantry","is_recipe_ingredient":true,"recipes":[]},{"name":"olive oil","section":"pantry","is_recipe_ingredient":true,"recipes":["Sheet Pan Salmon with Broccoli","chicken breasts with lemon","hummus","tomato pasta","sheet-pan chicken with jammy tomatoes","turkey meatballs","swordfish pasta"]},{"name":"honey","section":"pantry","is_recipe_ingredient":true,"recipes":["Sheet Pan Salmon with Broccoli","crispy tofu with cashews and blistered snap peas"]},{"name":"black pepper","section":"pantry","is_recipe_ingredient":true,"recipes":["Sheet Pan Salmon with Broccoli","sheet-pan chicken with jammy tomatoes"]},{"name":"apple cider vinegar","section":"pantry","is_recipe_ingredient":true,"recipes":[]},{"name":"pickles","section":"pantry","is_recipe_ingredient":true,"recipes":[]},{"name":"jasmine rice","section":"pantry","is_recipe_ingredient":true,"recipes":[]},{"name":"rice vinegar","section":"pantry","is_recipe_ingredient":true,"recipes":["Sheet Pan Salmon with Broccoli","crispy tofu with cashews and blistered snap peas"]},{"name":"balsamic vinegar","section":"pantry","is_recipe_ingredient":true,"recipes":[]},{"name":"vegetable oil","section":"pantry","is_recipe_ingredient":true,"recipes":["crispy tofu with cashews and blistered snap peas","crispy sheet-pan noodles"]},{"name":"baking soda","section":"pantry","is_recipe_ingredient":true,"recipes":[]},{"name":"mayonnaise","section":"pantry","is_recipe_ingredient":true,"recipes":["turkey and cheese sandwiches"]},{"name":"cannellini beans","section":"pantry","is_recipe_ingredient":true,"recipes":[]},{"name":"whole-wheat tortillas","section":"pantry","is_recipe_ingredient":true,"recipes":[]},{"name":"dumplings","section":"freezer","is_recipe_ingredient":false,"recipes":[]},{"name":"edamame","section":"freezer","is_recipe_ingredient":false,"recipes":[]},{"name":"ice cream","section":"freezer","is_recipe_ingredient":false,"recipes":[]},{"name":"old fashioned rolled oats","section":"pantry","is_recipe_ingredient":true,"recipes":["oatmeal chocolate chip cookies"]},{"name":"chocolate chips","section":"pantry","is_recipe_ingredient":true,"recipes":["oatmeal chocolate chip cookies"]},{"name":"baking powder","section":"pantry","is_recipe_ingredient":true,"recipes":["oatmeal chocolate chip cookies"]},{"name":"baking soda","section":"pantry","is_recipe_ingredient":true,"recipes":["oatmeal chocolate chip cookies"]},{"name":"salt","section":"pantry","is_recipe_ingredient":true,"recipes":["Sheet Pan Salmon with Broccoli","oatmeal chocolate chip cookies","crispy sheet-pan noodles","sheet-pan chicken with jammy tomatoes"]},{"name":"white sugar","section":"pantry","is_recipe_ingredient":true,"recipes":["oatmeal chocolate chip cookies"]},{"name":"vanilla extract","section":"pantry","is_recipe_ingredient":true,"recipes":["oatmeal chocolate chip cookies"]},{"name":"whole-wheat flour","section":"pantry","is_recipe_ingredient":true,"recipes":["oatmeal chocolate chip cookies"]},{"name":"tomatoes","section":"fresh","is_recipe_ingredient":true,"recipes":["tomato pasta"]},{"name":"basil","section":"fresh","is_recipe_ingredient":true,"recipes":["tomato pasta"]},{"name":"parmigiana","section":"dairy","is_recipe_ingredient":true,"recipes":["tomato pasta","turkey meatballs"]},{"name":"1/2 & 1/2","section":"dairy","is_recipe_ingredient":true,"recipes":["fried eggs for breakfast"]},{"name":"feta","section":"dairy","is_recipe_ingredient":true,"recipes":["fried eggs for breakfast"]},{"name":"instant ramen noodles","section":"pantry","is_recipe_ingredient":true,"recipes":["crispy sheet-pan noodles"]},{"name":"sesame oil","section":"pantry","is_recipe_ingredient":true,"recipes":["Sheet Pan Salmon with Broccoli","crispy sheet-pan noodles"]},{"name":"soy sauce","section":"pantry","is_recipe_ingredient":true,"recipes":["Sheet Pan Salmon with Broccoli","crispy tofu with cashews and blistered snap peas","crispy sheet-pan noodles"]},{"name":"baby bok choy","section":"fresh","is_recipe_ingredient":true,"recipes":["crispy sheet-pan noodles"]},{"name":"cilantro","section":"fresh","is_recipe_ingredient":true,"recipes":["crispy sheet-pan noodles"]},{"name":"hoisin","section":"pantry","is_recipe_ingredient":true,"recipes":["crispy sheet-pan noodles"]},{"name":"maple syrup","section":"pantry","is_recipe_ingredient":true,"recipes":["crispy sheet-pan noodles"]},{"name":"sesame seeds","section":"pantry","is_recipe_ingredient":true,"recipes":["Sheet Pan Salmon with Broccoli","crispy sheet-pan noodles"]},{"name":"ground turkey","section":"protein","is_recipe_ingredient":true,"recipes":["turkey meatballs"]},{"name":"panko bread crumbs","section":"pantry","is_recipe_ingredient":true,"recipes":["turkey meatballs"]},{"name":"garlic powder","section":"pantry","is_recipe_ingredient":true,"recipes":["turkey meatballs"]},{"name":"skinless boneless chicken thighs","section":"protein","is_recipe_ingredient":true,"recipes":["flue flighter chicken stew","sheet-pan chicken with jammy tomatoes"]},{"name":"carrots","section":"fresh","is_recipe_ingredient":true,"recipes":["flue flighter chicken stew"]},{"name":"red pepper flakes","section":"pantry","is_recipe_ingredient":true,"recipes":["flue flighter chicken stew","crispy tofu with cashews and blistered snap peas"]},{"name":"chicken broth","section":"pantry","is_recipe_ingredient":true,"recipes":["flue flighter chicken stew","chicken breasts with lemon"]},{"name":"string beans","section":"fresh","is_recipe_ingredient":false,"recipes":[]},{"name":"peaches","section":"fresh","is_recipe_ingredient":false,"recipes":[]},{"name":"whipped cream","section":"dairy","is_recipe_ingredient":false,"recipes":[]},{"name":"kiwi fruit","section":"fresh","is_recipe_ingredient":false,"recipes":[]},{"name":"marscapone cheese","section":"dairy","is_recipe_ingredient":false,"recipes":[]},{"name":"swordfish","section":"protein","is_recipe_ingredient":true,"recipes":["swordfish pasta"]},{"name":"eggplant","section":"fresh","is_recipe_ingredient":true,"recipes":["swordfish pasta"]},{"name":"tomato puree","section":"pantry","is_recipe_ingredient":true,"recipes":["swordfish pasta"]},{"name":"pine nuts","section":"pantry","is_recipe_ingredient":true,"recipes":["swordfish pasta"]},{"name":"french bread","section":"pantry","is_recipe_ingredient":false,"recipes":[]},{"name":"cayenne pepper","section":"pantry","is_recipe_ingredient":false,"recipes":[]}],"recipes":["oatmeal chocolate chip cookies","tomato pasta","fried eggs for breakfast","crispy sheet-pan noodles","turkey meatballs","flue flighter chicken stew","sheet-pan chicken with jammy tomatoes","turkey and cheese sandwiches","peanut butter and jelly on toast","cheese and apple snack","hummus","chicken breasts with lemon","crispy tofu with cashews and blistered snap peas","swordfish pasta"]}"#)?;
        Ok(file)
    }

    #[test]
    fn test_groceries_new() -> Result<(), Box<dyn std::error::Error>> {
        let path = "test_groceries.json";
        let _g = Groceries::new_initialized(path)?;
        let g = Groceries::from_path(path)?;
        insta::assert_json_snapshot!(g, @r###"
      {
        "sections": [],
        "collection": [],
        "recipes": []
      }
      "###);
        std::fs::remove_file(path)?;
        Ok(())
    }

    #[test]
    fn test_delete_recipe() -> Result<(), Box<dyn std::error::Error>> {
        let file = create_test_json_file()?;
        let mut g = Groceries::from_path(file.path())?;
        insta::assert_json_snapshot!(g.recipes, @r###"
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
        "###);
        g.delete_recipe("oatmeal chocolate chip cookies")?;
        insta::assert_json_snapshot!(g.recipes, @r###"
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
        "###);
        Ok(())
    }

    #[test]
    fn test_delete_item() -> Result<(), Box<dyn std::error::Error>> {
        let file = create_test_json_file()?;
        let mut g = Groceries::from_path(file.path())?;
        insta::assert_json_snapshot!(g.collection, @r###"
        [
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
            "name": "milk",
            "section": "dairy",
            "is_recipe_ingredient": true,
            "recipes": []
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
            "name": "ginger",
            "section": "fresh",
            "is_recipe_ingredient": true,
            "recipes": [
              "Sheet Pan Salmon with Broccoli"
            ]
          },
          {
            "name": "spinach",
            "section": "fresh",
            "is_recipe_ingredient": true,
            "recipes": [
              "fried eggs for breakfast",
              "flue flighter chicken stew"
            ]
          },
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
            "name": "yellow onion",
            "section": "fresh",
            "is_recipe_ingredient": true,
            "recipes": [
              "flue flighter chicken stew"
            ]
          },
          {
            "name": "fizzy water",
            "section": "dairy",
            "is_recipe_ingredient": false,
            "recipes": []
          },
          {
            "name": "kale",
            "section": "fresh",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "beer",
            "section": "dairy",
            "is_recipe_ingredient": false,
            "recipes": []
          },
          {
            "name": "parsley",
            "section": "fresh",
            "is_recipe_ingredient": true,
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
            "is_recipe_ingredient": false,
            "recipes": []
          },
          {
            "name": "kimchi",
            "section": "dairy",
            "is_recipe_ingredient": false,
            "recipes": []
          },
          {
            "name": "sour cream",
            "section": "dairy",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "potatoes",
            "section": "fresh",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "broccoli",
            "section": "fresh",
            "is_recipe_ingredient": true,
            "recipes": [
              "Sheet Pan Salmon with Broccoli"
            ]
          },
          {
            "name": "asparagus",
            "section": "fresh",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "dill",
            "section": "fresh",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "red onion",
            "section": "fresh",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "unsalted butter",
            "section": "dairy",
            "is_recipe_ingredient": true,
            "recipes": [
              "chicken breasts with lemon",
              "oatmeal chocolate chip cookies",
              "fried eggs for breakfast"
            ]
          },
          {
            "name": "scallions",
            "section": "fresh",
            "is_recipe_ingredient": true,
            "recipes": [
              "Sheet Pan Salmon with Broccoli",
              "crispy tofu with cashews and blistered snap peas"
            ]
          },
          {
            "name": "mozzarella",
            "section": "dairy",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "cucumbers",
            "section": "fresh",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "greek yogurt",
            "section": "dairy",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "cream cheese",
            "section": "dairy",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "sweet potato",
            "section": "fresh",
            "is_recipe_ingredient": false,
            "recipes": []
          },
          {
            "name": "sausages",
            "section": "protein",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "tofu",
            "section": "protein",
            "is_recipe_ingredient": true,
            "recipes": [
              "crispy tofu with cashews and blistered snap peas",
              "crispy sheet-pan noodles"
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
            "name": "tahini",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "hummus"
            ]
          },
          {
            "name": "chicken stock",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "orzo",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": []
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
            "name": "bread",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "fried eggs for breakfast",
              "peanut butter and jelly on toast",
              "turkey and cheese sandwiches"
            ]
          },
          {
            "name": "coffee",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "cumin",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "coconut milk (unsweetened)",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "crispy tofu with cashews and blistered snap peas"
            ]
          },
          {
            "name": "tortilla chips",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "Ritz crackers",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "black beans",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "mustard",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "turkey and cheese sandwiches"
            ]
          },
          {
            "name": "chips",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "popcorn",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": []
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
            "name": "honey",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "Sheet Pan Salmon with Broccoli",
              "crispy tofu with cashews and blistered snap peas"
            ]
          },
          {
            "name": "black pepper",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "Sheet Pan Salmon with Broccoli",
              "sheet-pan chicken with jammy tomatoes"
            ]
          },
          {
            "name": "apple cider vinegar",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "pickles",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "jasmine rice",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "rice vinegar",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "Sheet Pan Salmon with Broccoli",
              "crispy tofu with cashews and blistered snap peas"
            ]
          },
          {
            "name": "balsamic vinegar",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "vegetable oil",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "crispy tofu with cashews and blistered snap peas",
              "crispy sheet-pan noodles"
            ]
          },
          {
            "name": "baking soda",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "mayonnaise",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "turkey and cheese sandwiches"
            ]
          },
          {
            "name": "cannellini beans",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "whole-wheat tortillas",
            "section": "pantry",
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
            "name": "edamame",
            "section": "freezer",
            "is_recipe_ingredient": false,
            "recipes": []
          },
          {
            "name": "ice cream",
            "section": "freezer",
            "is_recipe_ingredient": false,
            "recipes": []
          },
          {
            "name": "old fashioned rolled oats",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "oatmeal chocolate chip cookies"
            ]
          },
          {
            "name": "chocolate chips",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "oatmeal chocolate chip cookies"
            ]
          },
          {
            "name": "baking powder",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "oatmeal chocolate chip cookies"
            ]
          },
          {
            "name": "baking soda",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "oatmeal chocolate chip cookies"
            ]
          },
          {
            "name": "salt",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "Sheet Pan Salmon with Broccoli",
              "oatmeal chocolate chip cookies",
              "crispy sheet-pan noodles",
              "sheet-pan chicken with jammy tomatoes"
            ]
          },
          {
            "name": "white sugar",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "oatmeal chocolate chip cookies"
            ]
          },
          {
            "name": "vanilla extract",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "oatmeal chocolate chip cookies"
            ]
          },
          {
            "name": "whole-wheat flour",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "oatmeal chocolate chip cookies"
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
            "name": "parmigiana",
            "section": "dairy",
            "is_recipe_ingredient": true,
            "recipes": [
              "tomato pasta",
              "turkey meatballs"
            ]
          },
          {
            "name": "1/2 & 1/2",
            "section": "dairy",
            "is_recipe_ingredient": true,
            "recipes": [
              "fried eggs for breakfast"
            ]
          },
          {
            "name": "feta",
            "section": "dairy",
            "is_recipe_ingredient": true,
            "recipes": [
              "fried eggs for breakfast"
            ]
          },
          {
            "name": "instant ramen noodles",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "crispy sheet-pan noodles"
            ]
          },
          {
            "name": "sesame oil",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "Sheet Pan Salmon with Broccoli",
              "crispy sheet-pan noodles"
            ]
          },
          {
            "name": "soy sauce",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "Sheet Pan Salmon with Broccoli",
              "crispy tofu with cashews and blistered snap peas",
              "crispy sheet-pan noodles"
            ]
          },
          {
            "name": "baby bok choy",
            "section": "fresh",
            "is_recipe_ingredient": true,
            "recipes": [
              "crispy sheet-pan noodles"
            ]
          },
          {
            "name": "cilantro",
            "section": "fresh",
            "is_recipe_ingredient": true,
            "recipes": [
              "crispy sheet-pan noodles"
            ]
          },
          {
            "name": "hoisin",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "crispy sheet-pan noodles"
            ]
          },
          {
            "name": "maple syrup",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "crispy sheet-pan noodles"
            ]
          },
          {
            "name": "sesame seeds",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "Sheet Pan Salmon with Broccoli",
              "crispy sheet-pan noodles"
            ]
          },
          {
            "name": "ground turkey",
            "section": "protein",
            "is_recipe_ingredient": true,
            "recipes": [
              "turkey meatballs"
            ]
          },
          {
            "name": "panko bread crumbs",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "turkey meatballs"
            ]
          },
          {
            "name": "garlic powder",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "turkey meatballs"
            ]
          },
          {
            "name": "skinless boneless chicken thighs",
            "section": "protein",
            "is_recipe_ingredient": true,
            "recipes": [
              "flue flighter chicken stew",
              "sheet-pan chicken with jammy tomatoes"
            ]
          },
          {
            "name": "carrots",
            "section": "fresh",
            "is_recipe_ingredient": true,
            "recipes": [
              "flue flighter chicken stew"
            ]
          },
          {
            "name": "red pepper flakes",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "flue flighter chicken stew",
              "crispy tofu with cashews and blistered snap peas"
            ]
          },
          {
            "name": "chicken broth",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "flue flighter chicken stew",
              "chicken breasts with lemon"
            ]
          },
          {
            "name": "string beans",
            "section": "fresh",
            "is_recipe_ingredient": false,
            "recipes": []
          },
          {
            "name": "peaches",
            "section": "fresh",
            "is_recipe_ingredient": false,
            "recipes": []
          },
          {
            "name": "whipped cream",
            "section": "dairy",
            "is_recipe_ingredient": false,
            "recipes": []
          },
          {
            "name": "kiwi fruit",
            "section": "fresh",
            "is_recipe_ingredient": false,
            "recipes": []
          },
          {
            "name": "marscapone cheese",
            "section": "dairy",
            "is_recipe_ingredient": false,
            "recipes": []
          },
          {
            "name": "swordfish",
            "section": "protein",
            "is_recipe_ingredient": true,
            "recipes": [
              "swordfish pasta"
            ]
          },
          {
            "name": "eggplant",
            "section": "fresh",
            "is_recipe_ingredient": true,
            "recipes": [
              "swordfish pasta"
            ]
          },
          {
            "name": "tomato puree",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "swordfish pasta"
            ]
          },
          {
            "name": "pine nuts",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "swordfish pasta"
            ]
          },
          {
            "name": "french bread",
            "section": "pantry",
            "is_recipe_ingredient": false,
            "recipes": []
          },
          {
            "name": "cayenne pepper",
            "section": "pantry",
            "is_recipe_ingredient": false,
            "recipes": []
          }
        ]
        "###);
        g.delete_item("eggs")?;
        insta::assert_json_snapshot!(g.collection, @r###"
        [
          {
            "name": "milk",
            "section": "dairy",
            "is_recipe_ingredient": true,
            "recipes": []
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
            "name": "ginger",
            "section": "fresh",
            "is_recipe_ingredient": true,
            "recipes": [
              "Sheet Pan Salmon with Broccoli"
            ]
          },
          {
            "name": "spinach",
            "section": "fresh",
            "is_recipe_ingredient": true,
            "recipes": [
              "fried eggs for breakfast",
              "flue flighter chicken stew"
            ]
          },
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
            "name": "yellow onion",
            "section": "fresh",
            "is_recipe_ingredient": true,
            "recipes": [
              "flue flighter chicken stew"
            ]
          },
          {
            "name": "fizzy water",
            "section": "dairy",
            "is_recipe_ingredient": false,
            "recipes": []
          },
          {
            "name": "kale",
            "section": "fresh",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "beer",
            "section": "dairy",
            "is_recipe_ingredient": false,
            "recipes": []
          },
          {
            "name": "parsley",
            "section": "fresh",
            "is_recipe_ingredient": true,
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
            "is_recipe_ingredient": false,
            "recipes": []
          },
          {
            "name": "kimchi",
            "section": "dairy",
            "is_recipe_ingredient": false,
            "recipes": []
          },
          {
            "name": "sour cream",
            "section": "dairy",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "potatoes",
            "section": "fresh",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "broccoli",
            "section": "fresh",
            "is_recipe_ingredient": true,
            "recipes": [
              "Sheet Pan Salmon with Broccoli"
            ]
          },
          {
            "name": "asparagus",
            "section": "fresh",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "dill",
            "section": "fresh",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "red onion",
            "section": "fresh",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "unsalted butter",
            "section": "dairy",
            "is_recipe_ingredient": true,
            "recipes": [
              "chicken breasts with lemon",
              "oatmeal chocolate chip cookies",
              "fried eggs for breakfast"
            ]
          },
          {
            "name": "scallions",
            "section": "fresh",
            "is_recipe_ingredient": true,
            "recipes": [
              "Sheet Pan Salmon with Broccoli",
              "crispy tofu with cashews and blistered snap peas"
            ]
          },
          {
            "name": "mozzarella",
            "section": "dairy",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "cucumbers",
            "section": "fresh",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "greek yogurt",
            "section": "dairy",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "cream cheese",
            "section": "dairy",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "sweet potato",
            "section": "fresh",
            "is_recipe_ingredient": false,
            "recipes": []
          },
          {
            "name": "sausages",
            "section": "protein",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "tofu",
            "section": "protein",
            "is_recipe_ingredient": true,
            "recipes": [
              "crispy tofu with cashews and blistered snap peas",
              "crispy sheet-pan noodles"
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
            "name": "tahini",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "hummus"
            ]
          },
          {
            "name": "chicken stock",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "orzo",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": []
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
            "name": "bread",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "fried eggs for breakfast",
              "peanut butter and jelly on toast",
              "turkey and cheese sandwiches"
            ]
          },
          {
            "name": "coffee",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "cumin",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "coconut milk (unsweetened)",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "crispy tofu with cashews and blistered snap peas"
            ]
          },
          {
            "name": "tortilla chips",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "Ritz crackers",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "black beans",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "mustard",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "turkey and cheese sandwiches"
            ]
          },
          {
            "name": "chips",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "popcorn",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": []
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
            "name": "honey",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "Sheet Pan Salmon with Broccoli",
              "crispy tofu with cashews and blistered snap peas"
            ]
          },
          {
            "name": "black pepper",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "Sheet Pan Salmon with Broccoli",
              "sheet-pan chicken with jammy tomatoes"
            ]
          },
          {
            "name": "apple cider vinegar",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "pickles",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "jasmine rice",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "rice vinegar",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "Sheet Pan Salmon with Broccoli",
              "crispy tofu with cashews and blistered snap peas"
            ]
          },
          {
            "name": "balsamic vinegar",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "vegetable oil",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "crispy tofu with cashews and blistered snap peas",
              "crispy sheet-pan noodles"
            ]
          },
          {
            "name": "baking soda",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "mayonnaise",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "turkey and cheese sandwiches"
            ]
          },
          {
            "name": "cannellini beans",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": []
          },
          {
            "name": "whole-wheat tortillas",
            "section": "pantry",
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
            "name": "edamame",
            "section": "freezer",
            "is_recipe_ingredient": false,
            "recipes": []
          },
          {
            "name": "ice cream",
            "section": "freezer",
            "is_recipe_ingredient": false,
            "recipes": []
          },
          {
            "name": "old fashioned rolled oats",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "oatmeal chocolate chip cookies"
            ]
          },
          {
            "name": "chocolate chips",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "oatmeal chocolate chip cookies"
            ]
          },
          {
            "name": "baking powder",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "oatmeal chocolate chip cookies"
            ]
          },
          {
            "name": "baking soda",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "oatmeal chocolate chip cookies"
            ]
          },
          {
            "name": "salt",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "Sheet Pan Salmon with Broccoli",
              "oatmeal chocolate chip cookies",
              "crispy sheet-pan noodles",
              "sheet-pan chicken with jammy tomatoes"
            ]
          },
          {
            "name": "white sugar",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "oatmeal chocolate chip cookies"
            ]
          },
          {
            "name": "vanilla extract",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "oatmeal chocolate chip cookies"
            ]
          },
          {
            "name": "whole-wheat flour",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "oatmeal chocolate chip cookies"
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
            "name": "parmigiana",
            "section": "dairy",
            "is_recipe_ingredient": true,
            "recipes": [
              "tomato pasta",
              "turkey meatballs"
            ]
          },
          {
            "name": "1/2 & 1/2",
            "section": "dairy",
            "is_recipe_ingredient": true,
            "recipes": [
              "fried eggs for breakfast"
            ]
          },
          {
            "name": "feta",
            "section": "dairy",
            "is_recipe_ingredient": true,
            "recipes": [
              "fried eggs for breakfast"
            ]
          },
          {
            "name": "instant ramen noodles",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "crispy sheet-pan noodles"
            ]
          },
          {
            "name": "sesame oil",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "Sheet Pan Salmon with Broccoli",
              "crispy sheet-pan noodles"
            ]
          },
          {
            "name": "soy sauce",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "Sheet Pan Salmon with Broccoli",
              "crispy tofu with cashews and blistered snap peas",
              "crispy sheet-pan noodles"
            ]
          },
          {
            "name": "baby bok choy",
            "section": "fresh",
            "is_recipe_ingredient": true,
            "recipes": [
              "crispy sheet-pan noodles"
            ]
          },
          {
            "name": "cilantro",
            "section": "fresh",
            "is_recipe_ingredient": true,
            "recipes": [
              "crispy sheet-pan noodles"
            ]
          },
          {
            "name": "hoisin",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "crispy sheet-pan noodles"
            ]
          },
          {
            "name": "maple syrup",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "crispy sheet-pan noodles"
            ]
          },
          {
            "name": "sesame seeds",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "Sheet Pan Salmon with Broccoli",
              "crispy sheet-pan noodles"
            ]
          },
          {
            "name": "ground turkey",
            "section": "protein",
            "is_recipe_ingredient": true,
            "recipes": [
              "turkey meatballs"
            ]
          },
          {
            "name": "panko bread crumbs",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "turkey meatballs"
            ]
          },
          {
            "name": "garlic powder",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "turkey meatballs"
            ]
          },
          {
            "name": "skinless boneless chicken thighs",
            "section": "protein",
            "is_recipe_ingredient": true,
            "recipes": [
              "flue flighter chicken stew",
              "sheet-pan chicken with jammy tomatoes"
            ]
          },
          {
            "name": "carrots",
            "section": "fresh",
            "is_recipe_ingredient": true,
            "recipes": [
              "flue flighter chicken stew"
            ]
          },
          {
            "name": "red pepper flakes",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "flue flighter chicken stew",
              "crispy tofu with cashews and blistered snap peas"
            ]
          },
          {
            "name": "chicken broth",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "flue flighter chicken stew",
              "chicken breasts with lemon"
            ]
          },
          {
            "name": "string beans",
            "section": "fresh",
            "is_recipe_ingredient": false,
            "recipes": []
          },
          {
            "name": "peaches",
            "section": "fresh",
            "is_recipe_ingredient": false,
            "recipes": []
          },
          {
            "name": "whipped cream",
            "section": "dairy",
            "is_recipe_ingredient": false,
            "recipes": []
          },
          {
            "name": "kiwi fruit",
            "section": "fresh",
            "is_recipe_ingredient": false,
            "recipes": []
          },
          {
            "name": "marscapone cheese",
            "section": "dairy",
            "is_recipe_ingredient": false,
            "recipes": []
          },
          {
            "name": "swordfish",
            "section": "protein",
            "is_recipe_ingredient": true,
            "recipes": [
              "swordfish pasta"
            ]
          },
          {
            "name": "eggplant",
            "section": "fresh",
            "is_recipe_ingredient": true,
            "recipes": [
              "swordfish pasta"
            ]
          },
          {
            "name": "tomato puree",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "swordfish pasta"
            ]
          },
          {
            "name": "pine nuts",
            "section": "pantry",
            "is_recipe_ingredient": true,
            "recipes": [
              "swordfish pasta"
            ]
          },
          {
            "name": "french bread",
            "section": "pantry",
            "is_recipe_ingredient": false,
            "recipes": []
          },
          {
            "name": "cayenne pepper",
            "section": "pantry",
            "is_recipe_ingredient": false,
            "recipes": []
          }
        ]
        "###);
        Ok(())
    }

    #[test]
    fn test_groceries() -> Result<(), Box<dyn std::error::Error>> {
        let file = create_test_json_file()?;
        let mut g = Groceries::from_path(file.path())?;

        insta::assert_json_snapshot!(g, @r###"
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
              "is_recipe_ingredient": true,
              "recipes": [
                "oatmeal chocolate chip cookies",
                "fried eggs for breakfast",
                "turkey meatballs"
              ]
            },
            {
              "name": "milk",
              "section": "dairy",
              "is_recipe_ingredient": true,
              "recipes": []
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
              "name": "ginger",
              "section": "fresh",
              "is_recipe_ingredient": true,
              "recipes": [
                "Sheet Pan Salmon with Broccoli"
              ]
            },
            {
              "name": "spinach",
              "section": "fresh",
              "is_recipe_ingredient": true,
              "recipes": [
                "fried eggs for breakfast",
                "flue flighter chicken stew"
              ]
            },
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
              "name": "yellow onion",
              "section": "fresh",
              "is_recipe_ingredient": true,
              "recipes": [
                "flue flighter chicken stew"
              ]
            },
            {
              "name": "fizzy water",
              "section": "dairy",
              "is_recipe_ingredient": false,
              "recipes": []
            },
            {
              "name": "kale",
              "section": "fresh",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "beer",
              "section": "dairy",
              "is_recipe_ingredient": false,
              "recipes": []
            },
            {
              "name": "parsley",
              "section": "fresh",
              "is_recipe_ingredient": true,
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
              "is_recipe_ingredient": false,
              "recipes": []
            },
            {
              "name": "kimchi",
              "section": "dairy",
              "is_recipe_ingredient": false,
              "recipes": []
            },
            {
              "name": "sour cream",
              "section": "dairy",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "potatoes",
              "section": "fresh",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "broccoli",
              "section": "fresh",
              "is_recipe_ingredient": true,
              "recipes": [
                "Sheet Pan Salmon with Broccoli"
              ]
            },
            {
              "name": "asparagus",
              "section": "fresh",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "dill",
              "section": "fresh",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "red onion",
              "section": "fresh",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "unsalted butter",
              "section": "dairy",
              "is_recipe_ingredient": true,
              "recipes": [
                "chicken breasts with lemon",
                "oatmeal chocolate chip cookies",
                "fried eggs for breakfast"
              ]
            },
            {
              "name": "scallions",
              "section": "fresh",
              "is_recipe_ingredient": true,
              "recipes": [
                "Sheet Pan Salmon with Broccoli",
                "crispy tofu with cashews and blistered snap peas"
              ]
            },
            {
              "name": "mozzarella",
              "section": "dairy",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "cucumbers",
              "section": "fresh",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "greek yogurt",
              "section": "dairy",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "cream cheese",
              "section": "dairy",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "sweet potato",
              "section": "fresh",
              "is_recipe_ingredient": false,
              "recipes": []
            },
            {
              "name": "sausages",
              "section": "protein",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "tofu",
              "section": "protein",
              "is_recipe_ingredient": true,
              "recipes": [
                "crispy tofu with cashews and blistered snap peas",
                "crispy sheet-pan noodles"
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
              "name": "tahini",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "hummus"
              ]
            },
            {
              "name": "chicken stock",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "orzo",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": []
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
              "name": "bread",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "fried eggs for breakfast",
                "peanut butter and jelly on toast",
                "turkey and cheese sandwiches"
              ]
            },
            {
              "name": "coffee",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "cumin",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "coconut milk (unsweetened)",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "crispy tofu with cashews and blistered snap peas"
              ]
            },
            {
              "name": "tortilla chips",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "Ritz crackers",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "black beans",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "mustard",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "turkey and cheese sandwiches"
              ]
            },
            {
              "name": "chips",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "popcorn",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": []
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
              "name": "honey",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "Sheet Pan Salmon with Broccoli",
                "crispy tofu with cashews and blistered snap peas"
              ]
            },
            {
              "name": "black pepper",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "Sheet Pan Salmon with Broccoli",
                "sheet-pan chicken with jammy tomatoes"
              ]
            },
            {
              "name": "apple cider vinegar",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "pickles",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "jasmine rice",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "rice vinegar",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "Sheet Pan Salmon with Broccoli",
                "crispy tofu with cashews and blistered snap peas"
              ]
            },
            {
              "name": "balsamic vinegar",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "vegetable oil",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "crispy tofu with cashews and blistered snap peas",
                "crispy sheet-pan noodles"
              ]
            },
            {
              "name": "baking soda",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "mayonnaise",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "turkey and cheese sandwiches"
              ]
            },
            {
              "name": "cannellini beans",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "whole-wheat tortillas",
              "section": "pantry",
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
              "name": "edamame",
              "section": "freezer",
              "is_recipe_ingredient": false,
              "recipes": []
            },
            {
              "name": "ice cream",
              "section": "freezer",
              "is_recipe_ingredient": false,
              "recipes": []
            },
            {
              "name": "old fashioned rolled oats",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "oatmeal chocolate chip cookies"
              ]
            },
            {
              "name": "chocolate chips",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "oatmeal chocolate chip cookies"
              ]
            },
            {
              "name": "baking powder",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "oatmeal chocolate chip cookies"
              ]
            },
            {
              "name": "baking soda",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "oatmeal chocolate chip cookies"
              ]
            },
            {
              "name": "salt",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "Sheet Pan Salmon with Broccoli",
                "oatmeal chocolate chip cookies",
                "crispy sheet-pan noodles",
                "sheet-pan chicken with jammy tomatoes"
              ]
            },
            {
              "name": "white sugar",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "oatmeal chocolate chip cookies"
              ]
            },
            {
              "name": "vanilla extract",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "oatmeal chocolate chip cookies"
              ]
            },
            {
              "name": "whole-wheat flour",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "oatmeal chocolate chip cookies"
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
              "name": "parmigiana",
              "section": "dairy",
              "is_recipe_ingredient": true,
              "recipes": [
                "tomato pasta",
                "turkey meatballs"
              ]
            },
            {
              "name": "1/2 & 1/2",
              "section": "dairy",
              "is_recipe_ingredient": true,
              "recipes": [
                "fried eggs for breakfast"
              ]
            },
            {
              "name": "feta",
              "section": "dairy",
              "is_recipe_ingredient": true,
              "recipes": [
                "fried eggs for breakfast"
              ]
            },
            {
              "name": "instant ramen noodles",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "crispy sheet-pan noodles"
              ]
            },
            {
              "name": "sesame oil",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "Sheet Pan Salmon with Broccoli",
                "crispy sheet-pan noodles"
              ]
            },
            {
              "name": "soy sauce",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "Sheet Pan Salmon with Broccoli",
                "crispy tofu with cashews and blistered snap peas",
                "crispy sheet-pan noodles"
              ]
            },
            {
              "name": "baby bok choy",
              "section": "fresh",
              "is_recipe_ingredient": true,
              "recipes": [
                "crispy sheet-pan noodles"
              ]
            },
            {
              "name": "cilantro",
              "section": "fresh",
              "is_recipe_ingredient": true,
              "recipes": [
                "crispy sheet-pan noodles"
              ]
            },
            {
              "name": "hoisin",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "crispy sheet-pan noodles"
              ]
            },
            {
              "name": "maple syrup",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "crispy sheet-pan noodles"
              ]
            },
            {
              "name": "sesame seeds",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "Sheet Pan Salmon with Broccoli",
                "crispy sheet-pan noodles"
              ]
            },
            {
              "name": "ground turkey",
              "section": "protein",
              "is_recipe_ingredient": true,
              "recipes": [
                "turkey meatballs"
              ]
            },
            {
              "name": "panko bread crumbs",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "turkey meatballs"
              ]
            },
            {
              "name": "garlic powder",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "turkey meatballs"
              ]
            },
            {
              "name": "skinless boneless chicken thighs",
              "section": "protein",
              "is_recipe_ingredient": true,
              "recipes": [
                "flue flighter chicken stew",
                "sheet-pan chicken with jammy tomatoes"
              ]
            },
            {
              "name": "carrots",
              "section": "fresh",
              "is_recipe_ingredient": true,
              "recipes": [
                "flue flighter chicken stew"
              ]
            },
            {
              "name": "red pepper flakes",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "flue flighter chicken stew",
                "crispy tofu with cashews and blistered snap peas"
              ]
            },
            {
              "name": "chicken broth",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "flue flighter chicken stew",
                "chicken breasts with lemon"
              ]
            },
            {
              "name": "string beans",
              "section": "fresh",
              "is_recipe_ingredient": false,
              "recipes": []
            },
            {
              "name": "peaches",
              "section": "fresh",
              "is_recipe_ingredient": false,
              "recipes": []
            },
            {
              "name": "whipped cream",
              "section": "dairy",
              "is_recipe_ingredient": false,
              "recipes": []
            },
            {
              "name": "kiwi fruit",
              "section": "fresh",
              "is_recipe_ingredient": false,
              "recipes": []
            },
            {
              "name": "marscapone cheese",
              "section": "dairy",
              "is_recipe_ingredient": false,
              "recipes": []
            },
            {
              "name": "swordfish",
              "section": "protein",
              "is_recipe_ingredient": true,
              "recipes": [
                "swordfish pasta"
              ]
            },
            {
              "name": "eggplant",
              "section": "fresh",
              "is_recipe_ingredient": true,
              "recipes": [
                "swordfish pasta"
              ]
            },
            {
              "name": "tomato puree",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "swordfish pasta"
              ]
            },
            {
              "name": "pine nuts",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "swordfish pasta"
              ]
            },
            {
              "name": "french bread",
              "section": "pantry",
              "is_recipe_ingredient": false,
              "recipes": []
            },
            {
              "name": "cayenne pepper",
              "section": "pantry",
              "is_recipe_ingredient": false,
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

        let item = GroceriesItem {
            name: crate::GroceriesItemName("cumquats".to_string()),
            section: crate::GroceriesItemSection("fresh".to_string()),
            is_recipe_ingredient: true,
            recipes: vec![Recipe("cumquat chutney".to_string())],
        };
        let recipe = Recipe("cumquat chutney".to_string());

        let ingredients = Ingredients(vec![
            GroceriesItemName("kumquats".to_string()),
            GroceriesItemName("carrots".to_owned()),
            GroceriesItemName("dried apricots".to_owned()),
            GroceriesItemName("dried cranberries".to_owned()),
            GroceriesItemName("chili".to_owned()),
            GroceriesItemName("onion".to_owned()),
            GroceriesItemName("garlic".to_owned()),
            GroceriesItemName("cider vinegar".to_owned()),
            GroceriesItemName("granulated sugar".to_owned()),
            GroceriesItemName("honey".to_owned()),
            GroceriesItemName("kosher salt".to_owned()),
            GroceriesItemName("cardamom".to_owned()),
            GroceriesItemName("cloves".to_owned()),
            GroceriesItemName("coriander".to_owned()),
            GroceriesItemName("ginger".to_owned()),
            GroceriesItemName("black peppercorns".to_owned()),
        ]);

        g.add_item(item);
        g.add_recipe(recipe, ingredients);

        insta::assert_json_snapshot!(g, @r###"
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
              "is_recipe_ingredient": true,
              "recipes": [
                "oatmeal chocolate chip cookies",
                "fried eggs for breakfast",
                "turkey meatballs"
              ]
            },
            {
              "name": "milk",
              "section": "dairy",
              "is_recipe_ingredient": true,
              "recipes": []
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
              "name": "ginger",
              "section": "fresh",
              "is_recipe_ingredient": true,
              "recipes": [
                "Sheet Pan Salmon with Broccoli",
                "cumquat chutney"
              ]
            },
            {
              "name": "spinach",
              "section": "fresh",
              "is_recipe_ingredient": true,
              "recipes": [
                "fried eggs for breakfast",
                "flue flighter chicken stew"
              ]
            },
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
                "swordfish pasta",
                "cumquat chutney"
              ]
            },
            {
              "name": "yellow onion",
              "section": "fresh",
              "is_recipe_ingredient": true,
              "recipes": [
                "flue flighter chicken stew"
              ]
            },
            {
              "name": "fizzy water",
              "section": "dairy",
              "is_recipe_ingredient": false,
              "recipes": []
            },
            {
              "name": "kale",
              "section": "fresh",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "beer",
              "section": "dairy",
              "is_recipe_ingredient": false,
              "recipes": []
            },
            {
              "name": "parsley",
              "section": "fresh",
              "is_recipe_ingredient": true,
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
              "is_recipe_ingredient": false,
              "recipes": []
            },
            {
              "name": "kimchi",
              "section": "dairy",
              "is_recipe_ingredient": false,
              "recipes": []
            },
            {
              "name": "sour cream",
              "section": "dairy",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "potatoes",
              "section": "fresh",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "broccoli",
              "section": "fresh",
              "is_recipe_ingredient": true,
              "recipes": [
                "Sheet Pan Salmon with Broccoli"
              ]
            },
            {
              "name": "asparagus",
              "section": "fresh",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "dill",
              "section": "fresh",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "red onion",
              "section": "fresh",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "unsalted butter",
              "section": "dairy",
              "is_recipe_ingredient": true,
              "recipes": [
                "chicken breasts with lemon",
                "oatmeal chocolate chip cookies",
                "fried eggs for breakfast"
              ]
            },
            {
              "name": "scallions",
              "section": "fresh",
              "is_recipe_ingredient": true,
              "recipes": [
                "Sheet Pan Salmon with Broccoli",
                "crispy tofu with cashews and blistered snap peas"
              ]
            },
            {
              "name": "mozzarella",
              "section": "dairy",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "cucumbers",
              "section": "fresh",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "greek yogurt",
              "section": "dairy",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "cream cheese",
              "section": "dairy",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "sweet potato",
              "section": "fresh",
              "is_recipe_ingredient": false,
              "recipes": []
            },
            {
              "name": "sausages",
              "section": "protein",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "tofu",
              "section": "protein",
              "is_recipe_ingredient": true,
              "recipes": [
                "crispy tofu with cashews and blistered snap peas",
                "crispy sheet-pan noodles"
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
              "name": "tahini",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "hummus"
              ]
            },
            {
              "name": "chicken stock",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "orzo",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": []
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
              "name": "bread",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "fried eggs for breakfast",
                "peanut butter and jelly on toast",
                "turkey and cheese sandwiches"
              ]
            },
            {
              "name": "coffee",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "cumin",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "coconut milk (unsweetened)",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "crispy tofu with cashews and blistered snap peas"
              ]
            },
            {
              "name": "tortilla chips",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "Ritz crackers",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "black beans",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "mustard",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "turkey and cheese sandwiches"
              ]
            },
            {
              "name": "chips",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "popcorn",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": []
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
              "name": "honey",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "Sheet Pan Salmon with Broccoli",
                "crispy tofu with cashews and blistered snap peas",
                "cumquat chutney"
              ]
            },
            {
              "name": "black pepper",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "Sheet Pan Salmon with Broccoli",
                "sheet-pan chicken with jammy tomatoes"
              ]
            },
            {
              "name": "apple cider vinegar",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "pickles",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "jasmine rice",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "rice vinegar",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "Sheet Pan Salmon with Broccoli",
                "crispy tofu with cashews and blistered snap peas"
              ]
            },
            {
              "name": "balsamic vinegar",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "vegetable oil",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "crispy tofu with cashews and blistered snap peas",
                "crispy sheet-pan noodles"
              ]
            },
            {
              "name": "baking soda",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "mayonnaise",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "turkey and cheese sandwiches"
              ]
            },
            {
              "name": "cannellini beans",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": []
            },
            {
              "name": "whole-wheat tortillas",
              "section": "pantry",
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
              "name": "edamame",
              "section": "freezer",
              "is_recipe_ingredient": false,
              "recipes": []
            },
            {
              "name": "ice cream",
              "section": "freezer",
              "is_recipe_ingredient": false,
              "recipes": []
            },
            {
              "name": "old fashioned rolled oats",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "oatmeal chocolate chip cookies"
              ]
            },
            {
              "name": "chocolate chips",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "oatmeal chocolate chip cookies"
              ]
            },
            {
              "name": "baking powder",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "oatmeal chocolate chip cookies"
              ]
            },
            {
              "name": "baking soda",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "oatmeal chocolate chip cookies"
              ]
            },
            {
              "name": "salt",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "Sheet Pan Salmon with Broccoli",
                "oatmeal chocolate chip cookies",
                "crispy sheet-pan noodles",
                "sheet-pan chicken with jammy tomatoes"
              ]
            },
            {
              "name": "white sugar",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "oatmeal chocolate chip cookies"
              ]
            },
            {
              "name": "vanilla extract",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "oatmeal chocolate chip cookies"
              ]
            },
            {
              "name": "whole-wheat flour",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "oatmeal chocolate chip cookies"
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
              "name": "parmigiana",
              "section": "dairy",
              "is_recipe_ingredient": true,
              "recipes": [
                "tomato pasta",
                "turkey meatballs"
              ]
            },
            {
              "name": "1/2 & 1/2",
              "section": "dairy",
              "is_recipe_ingredient": true,
              "recipes": [
                "fried eggs for breakfast"
              ]
            },
            {
              "name": "feta",
              "section": "dairy",
              "is_recipe_ingredient": true,
              "recipes": [
                "fried eggs for breakfast"
              ]
            },
            {
              "name": "instant ramen noodles",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "crispy sheet-pan noodles"
              ]
            },
            {
              "name": "sesame oil",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "Sheet Pan Salmon with Broccoli",
                "crispy sheet-pan noodles"
              ]
            },
            {
              "name": "soy sauce",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "Sheet Pan Salmon with Broccoli",
                "crispy tofu with cashews and blistered snap peas",
                "crispy sheet-pan noodles"
              ]
            },
            {
              "name": "baby bok choy",
              "section": "fresh",
              "is_recipe_ingredient": true,
              "recipes": [
                "crispy sheet-pan noodles"
              ]
            },
            {
              "name": "cilantro",
              "section": "fresh",
              "is_recipe_ingredient": true,
              "recipes": [
                "crispy sheet-pan noodles"
              ]
            },
            {
              "name": "hoisin",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "crispy sheet-pan noodles"
              ]
            },
            {
              "name": "maple syrup",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "crispy sheet-pan noodles"
              ]
            },
            {
              "name": "sesame seeds",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "Sheet Pan Salmon with Broccoli",
                "crispy sheet-pan noodles"
              ]
            },
            {
              "name": "ground turkey",
              "section": "protein",
              "is_recipe_ingredient": true,
              "recipes": [
                "turkey meatballs"
              ]
            },
            {
              "name": "panko bread crumbs",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "turkey meatballs"
              ]
            },
            {
              "name": "garlic powder",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "turkey meatballs"
              ]
            },
            {
              "name": "skinless boneless chicken thighs",
              "section": "protein",
              "is_recipe_ingredient": true,
              "recipes": [
                "flue flighter chicken stew",
                "sheet-pan chicken with jammy tomatoes"
              ]
            },
            {
              "name": "carrots",
              "section": "fresh",
              "is_recipe_ingredient": true,
              "recipes": [
                "flue flighter chicken stew",
                "cumquat chutney"
              ]
            },
            {
              "name": "red pepper flakes",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "flue flighter chicken stew",
                "crispy tofu with cashews and blistered snap peas"
              ]
            },
            {
              "name": "chicken broth",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "flue flighter chicken stew",
                "chicken breasts with lemon"
              ]
            },
            {
              "name": "string beans",
              "section": "fresh",
              "is_recipe_ingredient": false,
              "recipes": []
            },
            {
              "name": "peaches",
              "section": "fresh",
              "is_recipe_ingredient": false,
              "recipes": []
            },
            {
              "name": "whipped cream",
              "section": "dairy",
              "is_recipe_ingredient": false,
              "recipes": []
            },
            {
              "name": "kiwi fruit",
              "section": "fresh",
              "is_recipe_ingredient": false,
              "recipes": []
            },
            {
              "name": "marscapone cheese",
              "section": "dairy",
              "is_recipe_ingredient": false,
              "recipes": []
            },
            {
              "name": "swordfish",
              "section": "protein",
              "is_recipe_ingredient": true,
              "recipes": [
                "swordfish pasta"
              ]
            },
            {
              "name": "eggplant",
              "section": "fresh",
              "is_recipe_ingredient": true,
              "recipes": [
                "swordfish pasta"
              ]
            },
            {
              "name": "tomato puree",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "swordfish pasta"
              ]
            },
            {
              "name": "pine nuts",
              "section": "pantry",
              "is_recipe_ingredient": true,
              "recipes": [
                "swordfish pasta"
              ]
            },
            {
              "name": "french bread",
              "section": "pantry",
              "is_recipe_ingredient": false,
              "recipes": []
            },
            {
              "name": "cayenne pepper",
              "section": "pantry",
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
}
