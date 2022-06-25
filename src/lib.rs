// Try this:
// Note: Loads the contents of the module from another file
//       with the same name as the module. Read more at
//       https://doc.rust-lang.org/book/ch07-05-separating-modules-into-different-files.html
mod cli;
mod errors;
mod groceries;
mod groceriesitem;
mod helpers;
mod recipes;
mod run_groceries;
mod run_recipes;
mod run_shopping_list;
mod shoppinglist;
pub mod startup;

// Note: Re-exports the content of the square_content module to keep paths short.
//       Read more at https://doc.rust-lang.org/reference/items/use-declarations.html#use-visibility
pub use crate::cli::*;
pub use crate::errors::*;
pub use crate::groceries::*;
pub use crate::groceriesitem::*;
pub use crate::helpers::*;
pub use crate::recipes::*;
pub use crate::run_groceries::*;
pub use crate::run_recipes::*;
pub use crate::run_shopping_list::*;
pub use crate::shoppinglist::*;
pub use crate::startup::*;
