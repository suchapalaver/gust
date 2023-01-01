use crate::{ReadError, run_recipes, run_groceries, run_shopping_list};

pub fn run() -> Result<(), ReadError> {
    let matches = crate::cli::cli().get_matches();

    match matches.subcommand() {
        Some(("recipes", sync_matches)) => Ok(run_recipes::run(sync_matches)?),
        Some(("groceries", _sync_matches)) => Ok(run_groceries::run()?),
        Some(("list", _sync_matches)) => Ok(run_shopping_list::run()?),
        _ => unreachable!(),
    }
}
     