use matrix_sdk::ruma::UserId;
use matrix_sdk::Client as MatrixClient;

use crate::configuration::MatrixSettings;

pub async fn login(
    configuration: &MatrixSettings,
) -> anyhow::Result<matrix_sdk::Client, anyhow::Error> {
    let bot_user = UserId::parse(&configuration.username)?;
    let matrix_client = MatrixClient::builder()
        .server_name(bot_user.server_name())
        .build()
        .await?;

    // First we need to log in.
    matrix_client
        .login_username(&bot_user, &configuration.password)
        .send()
        .await?;

    Ok(matrix_client)
}
