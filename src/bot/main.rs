use bot::{
    auth::login,
    configuration::get_configuration,
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

    let _ = matrix_client.sync_once(SyncSettings::default()).await;
    
    let application = Application::build(configuration, &matrix_client).await?;
    application.run_until_stopped().await?;

    Ok(())
}
