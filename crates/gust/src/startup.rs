use crate::{cli, command::UserCommand, CliError};
use api::{Api, ApiError};
use tracing::instrument;

#[instrument]
pub async fn run() -> Result<(), CliError> {
    let matches = cli().get_matches();

    let api = Api::init(
        matches
            .get_one::<String>("database")
            .expect("'database' has a default setting")
            .parse()
            .map_err(ApiError::from)?,
    )
    .await?;

    let command: UserCommand = matches.try_into()?;

    let response = api.dispatch(command.into()).await?;

    println!("{response}");

    Ok(())
}
