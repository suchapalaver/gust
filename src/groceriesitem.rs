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
