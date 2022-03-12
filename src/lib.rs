// Try this:
// Note: Loads the contents of the module from another file
//       with the same name as the module. Read more at
//       https://doc.rust-lang.org/book/ch07-05-separating-modules-into-different-files.html
mod errors;
mod groceries;
mod helpers;
mod run;
mod run_recipes;
mod shoppinglist;

// Note: Re-exports the content of the square_content module to keep paths short.
//       Read more at https://doc.rust-lang.org/reference/items/use-declarations.html#use-visibility
pub use crate::errors::*;
pub use crate::groceries::*;
pub use crate::helpers::*;
pub use crate::run::*;
pub use crate::run_recipes::*;
pub use crate::shoppinglist::*;
