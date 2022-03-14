use std::{error::Error, fmt, fs::File, io::BufReader, ops::Deref, path::Path};

use serde::{Deserialize, Serialize};

use crate::{input, prompt_for_y, read, ReadError::DeserializingError};
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

pub fn run_groceries() -> Result<(), Box<dyn Error>> {
    print_groceries()?;

    add_groceries()?;

    Ok(())
}

fn print_groceries() -> Result<(), Box<dyn Error>> {
    eprintln!(
        "View the groceries in our library?\n\
         --y\n\
         --any other key to continue"
    );

    while crate::prompt_for_y()? {
        eprintln!();

        let path = "groceries.json";

        let groceries = Groceries::from_path(path).map_err(|e| {
            format!(
                "Failed to read groceries file '{}':\n\
             '{}'\n",
                path, e
            )
        })?;

        for sec in groceries.sections {
            let sec_items = groceries
                .collection
                .iter()
                .filter(|x| x.section.0.contains(&sec.0))
                .collect::<Vec<&GroceriesItem>>();
            for item in sec_items {
                eprintln!("{}", item);
            }
            eprintln!();
        }

        eprintln!();

        eprintln!(
            "View the groceries in our library?\n\
             --y\n\
             --any other key to continue"
        );
    }
    Ok(())
}

fn add_groceries() -> Result<(), Box<dyn Error>> {
    eprintln!(
        "Add groceries to our library?\n\
         --y\n\
         --any other key to exit"
    );

    while prompt_for_y()? {
        let path = "groceries.json";

        let groceries = Groceries::from_path(path).map_err(|e| {
            format!(
                "Failed to read groceries file '{}':\n\
		 '{}'\n",
                path, e
            )
        })?;

        let groceries = input_item(groceries)?;

        let json = serde_json::to_string(&groceries)?;

        crate::helpers::write(path, json)?;

        eprintln!(
            "Add more groceries to our library?\n\
	     --y\n\
	     --any other key to exit"
        );
    }
    Ok(())
}

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
    pub fn new() -> Self {
        Self::new_initialized()
    }

    pub fn new_initialized() -> Self {
        let collection: Vec<GroceriesItem> = Vec::new();

        let recipes: Vec<Recipe> = Vec::new();

        let sections: Vec<GroceriesItemSection> = Vec::new();

        //let list_recipes: Vec<Recipe> = Vec::new();

        Self {
            sections,
            collection,
            recipes,
            //list_recipes,
        } // using Self, can I make this a generic trait?
    }

    pub fn from_path<P: AsRef<Path> + Copy>(path: P) -> Result<Groceries, Box<dyn Error>> {
        let reader: BufReader<File> = read(path)?;

        let data: Groceries = serde_json::from_reader(reader).map_err(DeserializingError)?;

        Ok(data)
    }

    pub fn add_item(mut self, name: String, section: String) -> Result<Groceries, Box<dyn Error>> {
        let name = GroceriesItemName(name);
        let section = GroceriesItemSection(section);
        let item = GroceriesItem::new(name, section)?;
        self.collection.push(item);
        Ok(self)
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
    pub fn new(
        name: GroceriesItemName,
        section: GroceriesItemSection,
    ) -> Result<Self, Box<dyn Error>> {
        // this fn def is duplicate
        Self::new_initialized(name, section)
    }

    pub fn new_initialized(
        name: GroceriesItemName,
        section: GroceriesItemSection,
    ) -> Result<Self, Box<dyn Error>> {
        //let name = name_and_section.get(0).expect("no grocery name found!");
        //let section = name_and_section.get(1).expect("no grocery section found");
        Ok(GroceriesItem {
            name,
            section,
            is_recipe_ingredient: false,
            recipes: vec![],
            //on_list: false,
            //on_checklist: false,
        })
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
    // Input a list of ingredients and return it ...
    pub fn new(input: String) -> Result<Ingredients, Box<dyn Error>> {
        let ingredients_list: Ingredients = Ingredients(
            input
                .split(',')
                .map(|item| GroceriesItemName(item.trim().to_lowercase()))
                .collect(),
        );

        Ok(ingredients_list)
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

pub fn add_recipe(name: String, ingredients: String) -> Result<(), Box<dyn Error>> {
    let recipe_name = Recipe(name);

    let recipe_ingredients: Ingredients = Ingredients::new(ingredients)?;

    let mut groceries = Groceries::from_path("groceries.json")?;

    // 1st add new items to groceries
    for ingredient in recipe_ingredients.iter() {
        if groceries.collection.iter().all(|g| &g.name != ingredient) {
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

		let input = input()?;

		section_input = match input {
		    _ if input == "1".to_string() => {section_input_ok = true; "fresh".to_string()},
		    _ if input == "2".to_string() => {section_input_ok = true; "pantry".to_string()},
		    _ if input == "3".to_string() => {section_input_ok = true; "protein".to_string()},
		    _ if input == "4".to_string() => {section_input_ok = true; "dairy".to_string()},
		    _ if input == "5".to_string() => {section_input_ok = true; "freezer".to_string()},
		    _                             => {eprintln!("re-enter section information"); continue},
		};
	    }
	    let section = GroceriesItemSection(section_input);
	    
	    let new_item = GroceriesItem::new(ingredient.clone(), section)?;
	    
	    groceries.collection.push(new_item);       
	}
    }
    // 2nd update recipe info for groceriesitems
    groceries
        .collection
        .iter_mut()
        .filter(|x| recipe_ingredients.contains(&x.name))
        .for_each(|x| {
            if !x.is_recipe_ingredient {
                x.is_recipe_ingredient = true;
            }
            x.recipes.push(recipe_name.clone());
        });

    // 3rd add recipe to
    //let path = "recipes.json";
    /*
    let mut recipes = Recipes::from_path(path).map_err(|e| {
    format!(
        "Failed to read recipes file '{}':\n\
         '{}'",
        path, e
    )
    })?;
    */
    groceries.recipes.push(recipe_name);

    let json = serde_json::to_string(&groceries)?;
    // Put trace here
    crate::helpers::write("groceries.json", json)?;

    Ok(())
}

pub fn print_recipes() -> Result<(), Box<dyn Error>> {
    let path = "groceries.json";

    let groceries = Groceries::from_path(path).map_err(|e| {
        format!(
            "Failed to read recipes file '{}':\n\
	     '{}'",
            path, e
        )
    })?;

    for recipe in groceries.recipes.iter() {
        eprintln!("- {}", recipe);
    }
    eprintln!();

    Ok(())
}

fn input_item(mut groceries: Groceries) -> Result<Groceries, Box<dyn Error>> {
    // get item info
    eprintln!(
        "Enter the item\n\
	 e.g. 'bread'"
    );
    let name = input()?;
    eprintln!(
        "Enter the section (fresh, pantry, protein, dairy, freezer)\n\
	 e.g. 'fresh'"
    );
    let section = input()?;
    // check if there are no matches
    if groceries
        .collection
        .iter()
        .all(|item| no_match(&name, item))
    {
        // if no matches add the item to groceries
        groceries.collection.push(GroceriesItem::new(
            GroceriesItemName(name),
            GroceriesItemSection(section),
        )?);
    } else {
        // check any matches for a genuine match,
        // e.g. 'instant ramen noodles' is a genuine match for 'ramen noodles'
        // (in our case, at least)
        let mut found_no_matches = true;
        for item in groceries.collection.iter() {
            if !no_match(&name, item) {
                eprintln!(
                    "is *{}* a match?\n\
			   *y* for yes
			   *any other key* for no",
                    item
                );
                if prompt_for_y()? {
                    found_no_matches = false;
                    break;
                }
            }
        }
        if found_no_matches {
            // means we need to add the item to groceries afterall
            // after we had to check for any fakse matches above
            groceries = groceries.add_item(name, section)?;
        }
    }
    Ok(groceries)
}

fn no_match(name: &str, item: &GroceriesItem) -> bool {
    name.split(' ').all(|word| !item.name.0.contains(word))
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
