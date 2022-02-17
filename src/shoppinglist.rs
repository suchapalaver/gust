use serde::{Serialize, Deserialize};

use crate::{GroceriesItem, Recipe};

// used to serialize and deserialize the
// most recently saved list or to create a
// new grocery list that can be saved as JSON
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShoppingList {
    pub checklist: Vec<GroceriesItem>,
    pub recipes: Vec<Recipe>,
    pub groceries: Vec<GroceriesItem>,
}

impl ShoppingList {
    pub fn new() -> Self {
        // this fn def is duplicate
        Self::new_initialized()
    }

    pub fn new_initialized() -> Self {
        ShoppingList {
            // this fn def is unique to this struct
            checklist: vec![], // or default
            recipes: vec![],   // or default()
            groceries: vec![],
        }
    }
}

/*
// You need a checklist on every shopping list
// to check if we need certain items we're not sure about
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Checklist(pub Vec<GroceriesItem>);

impl Checklist {
fn new() -> Self {
	    Checklist::new_initialized()
	}

	pub fn new_initialized() -> Checklist {
	    Checklist(Vec::new())
	}

	pub fn as_slice(&self) -> &[GroceriesItem] {
&self.0
	}
}
*/
