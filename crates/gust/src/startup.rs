use crate::{cli, command::GustCommand, CliError};
use api::{Api, ApiError};
use tracing::instrument;

#[instrument]
pub async fn run() -> Result<(), CliError> {
    let matches = cli().get_matches();

    let api = Api::init(
        matches
            .get_one::<String>("store")
            .expect("'store' has a default setting")
            .parse()
            .map_err(ApiError::from)?,
    )
    .await?;

    let command: GustCommand = matches.try_into()?;

    let response = api.dispatch(command.into()).await?;

    println!("{response}");

    Ok(())
}
