use bot::{
    auth::login,
    configuration::get_configuration,
    listeners::init_matrix_event_subscription,
    startup::Application,
    telemetry::{get_subscriber, init_subscriber},
};
use matrix_sdk::config::SyncSettings;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = get_subscriber("bot".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration");

    let matrix_client = login(&configuration.matrix).await?;

    matrix_client.sync_once(SyncSettings::default()).await?;
    init_matrix_event_subscription(&matrix_client)
        .await
        .unwrap();

    let application = Application::build(configuration, &matrix_client).await?;
    application.run_until_stopped().await?;

    // Syncing is important to synchronize the client state with the server.
    // This method will never return unless there is an error.
    matrix_client.sync(SyncSettings::default()).await?;

    Ok(())
}
