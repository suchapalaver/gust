use std::{
    fmt,
    fs::{self},
    ops::Deref,
    path::Path,
    str::FromStr,
};

use serde::{Deserialize, Serialize};

use crate::ReadError;
/*
///  let apples = GroceriesItem {
///     name: GroceriesItemName("apples".to_string()),
///     section: GroceriesItemSection("fresh".to_string()),
///     is_recipe_ingredient: true,
///     recipes: vec![Recipe("cheese and apple snack".to_string())],
///  };
///
///  let cheddar = GroceriesItem {
///     name: GroceriesItemName("cheddar cheese".to_string()),
///     section: GroceriesItemSection("dairy".to_string()),
///     is_recipe_ingredient: true,
///     recipes: vec![Recipe("cheese and apple snack".to_string()), Recipe("tomato pasta".to_string())],
///     };
///
/// let olive_oil = GroceriesItem {
///     name: GroceriesItemName("olive oil".to_string()),
///     section: GroceriesItemSection("pantry".to_string()),
///     is_recipe_ingredient: true,
///     recipes: vec![Recipe("TOO MANY TO NAME!".to_string())],
/// };
///
/// let groceries = Groceries {
///     collection: vec![apples, cheddar, olive_oil],
/// };
///
/// deref is implicitly called so groceries.iter()
/// represents groceries.collection.iter()
///
/// println!("groceries:");
///
/// for item in groceries.iter() {
///     println!("{}", item);
/// }
///
/// for item in groceries.iter() {
///    for r in item.iter() {
///         println!("{}", r);
///     }
/// }
///
/// let apple_cheese_ingredients: Vec<_> = groceries
///     .iter()
///     .filter(|x| {
///         x.recipes
///             .contains(&Recipe("cheese and apple snack".to_string()))
///     })
///     .collect();
///
/// println!();
/// println!("experiment:");
/// for ingredient in apple_cheese_ingredients {
///     let recipes = ingredient.iter().map(|x| x.0.clone()).collect::<Vec<_>>();
///     let recipes_str = recipes.join(", ");
///     println!("ingredient: {}\trecipes: {}", ingredient, recipes_str);
/// }
 */

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Groceries {
    pub sections: Vec<GroceriesItemSection>,
    pub collection: Vec<GroceriesItem>,
    pub recipes: Vec<Recipe>,
    //pub list_recipes: Vec<Recipe>,
}
/*
impl Deref for Groceries {
    type Target = Vec<GroceriesItem>;

    fn deref(&self) -> &Self::Target {
        &self.collection
    }
}

impl DerefMut for Groceries {
    //type Target = Vec<GroceriesItem>;

    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.collection
    }
}
*/
impl Groceries {
    pub fn from_path<P: AsRef<Path> + Copy>(path: P) -> Result<Groceries, ReadError> {
        Ok(serde_json::from_reader(crate::read(path)?)?)
    }

    pub fn add_item(&mut self, item: GroceriesItem) {
        self.collection.push(item);
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), ReadError> {
        let s = serde_json::to_string(&self)?;
        // let _ = fs::write(path, s)?;
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
            eprintln!("- {}", recipe);
        }
    }

    pub fn add_recipe(
        &mut self,
        recipe: Recipe,
        ingredients: Ingredients,
    ) -> Result<(), ReadError> {
        let recipe_name = recipe.clone();

        // 1st add new items to groceries
        for ingredient in ingredients.iter() {
            if self.collection.iter().all(|g| &g.name != ingredient) {
                let mut section_input_ok = false;
                let mut section_input = String::new();
                while !section_input_ok {
                    eprintln!(
                        "which section is {} in?\n\
                        *1* fresh
                        *2* pantry 
                        *3* protein 
                        *4* dairy 
                        *5* freezer",
                        ingredient
                    );

                    let input = crate::get_user_input()?;

                    section_input = match &input {
                        _ if input == "1" => {
                            section_input_ok = true;
                            "fresh".to_string()
                        }
                        _ if input == "2" => {
                            section_input_ok = true;
                            "pantry".to_string()
                        }
                        _ if input == "3" => {
                            section_input_ok = true;
                            "protein".to_string()
                        }
                        _ if input == "4" => {
                            section_input_ok = true;
                            "dairy".to_string()
                        }
                        _ if input == "5" => {
                            section_input_ok = true;
                            "freezer".to_string()
                        }
                        _ => {
                            eprintln!("re-enter section information");
                            continue;
                        }
                    };
                }
                let section = GroceriesItemSection(section_input);

                let new_item = GroceriesItem::new_initialized(ingredient.clone(), section);

                self.collection.push(new_item);
            }
        }
        // 2nd update recipe info for groceriesitems
        self.update_recipe_info(recipe, ingredients);

        // 3rd add recipe to
        self.recipes.push(recipe_name);

        Ok(())
    }

    fn update_recipe_info(&mut self, recipe: Recipe, ingredients: Ingredients) {
        self.collection
            .iter_mut()
            .filter(|x| ingredients.contains(&x.name))
            .for_each(|mut x| {
                if !x.is_recipe_ingredient {
                    x.is_recipe_ingredient = true;
                }
                x.recipes.push(recipe.clone());
            })
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
    pub fn new() -> Result<Option<Self>, ReadError> {
        // get item info
        eprintln!(
            "Enter the item\n\
            e.g. 'bread'"
        );
        let name = crate::get_user_input()?;
        eprintln!(
            "Enter the section (fresh, pantry, protein, dairy, freezer)\n\
            e.g. 'fresh'"
        );
        let section = crate::get_user_input()?;

        let groceries = Groceries::from_path("groceries.json")?;

        // check if there are no matches
        if groceries.collection.iter().all(|item| !item.matches(&name)) {
            // if no matches add the item to groceries
            Ok(Some(Self::new_initialized(
                GroceriesItemName(name),
                GroceriesItemSection(section),
            )))
        } else {
            // check any matches for a genuine match,
            // e.g. 'instant ramen noodles' is a genuine match for 'ramen noodles'
            // (in our case, at least)
            let mut found_no_matches = true;
            for item in groceries.collection.iter() {
                if item.matches(&name) {
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
                    GroceriesItemName(name),
                    GroceriesItemSection(section),
                )))
            } else {
                Ok(None)
            }
        }
    }

    fn new_initialized(name: GroceriesItemName, section: GroceriesItemSection) -> Self {
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
        // Input a list of ingredients and return it ...
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

/*
// Gets a new recipes ingredients from user
// and returns it as a Recipe
fn get_new_recipe() -> Result<Recipe, Box<dyn Error>> {

    let mut items = Vec::new();

    items = input_item(items)?;

    Ok(Recipe(String::from(name)))
}
*/

/*
// Input a list of ingredients and return it ...
pub fn input_ingredients() -> Result<Ingredients, Box<dyn Error>> {
    let input: String = input()?;

    let ingredients_list:
    Ingredients = Ingredients(input
         .split(',')
         .map(|item| GroceriesItemName(item.trim().to_lowercase()))
         .collect()
    );

    Ok(ingredients_list)
}
*/
