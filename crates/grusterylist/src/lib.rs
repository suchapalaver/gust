// Try this:
// Note: Loads the contents of the module from another file
//       with the same name as the module. Read more at
//       https://doc.rust-lang.org/book/ch07-05-separating-modules-into-different-files.html
mod cli;
mod migrate_json_db;
mod run_groceries;
mod run_recipes;
pub mod startup;

// Note: Re-exports the content of the square_content module to keep paths short.
//       Read more at https://doc.rust-lang.org/reference/items/use-declarations.html#use-visibility
pub use crate::cli::*;
pub use crate::startup::*;
