pub fn run() -> Result<(), crate::ReadError> {
    let matches = crate::cli::cli().get_matches();

    match matches.subcommand() {
        Some(("recipes", sync_matches)) => Ok(crate::run_recipes::run(sync_matches)?),
        Some(("groceries", _sync_matches)) => Ok(crate::run_groceries::run()?),
        Some(("list", _sync_matches)) => Ok(crate::run_shopping_list::run()?),
        _ => unreachable!(),
    }
}
