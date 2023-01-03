use crate::{migrate_json_db, run_groceries, run_recipes, run_shopping_list, ReadError, cli};

pub fn run() -> Result<(), ReadError> {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("recipes", sync_matches)) => Ok(run_recipes::run(sync_matches)?),
        Some(("groceries", _)) => Ok(run_groceries::run()?),
        Some(("list", _)) => Ok(run_shopping_list::run()?),
        Some(("show-list-sections", _)) => {
            run_shopping_list::sections();
            Ok(())
        }
        Some(("migrate-json-groceries-to-db", _)) => Ok(migrate_json_db::migrate_groceries()?),
        Some(("show-items-in-db", _)) => {
            run_groceries::show_items();
            Ok(())
        }
        _ => unreachable!(),
    }
}
