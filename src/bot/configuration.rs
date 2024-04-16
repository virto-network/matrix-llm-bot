use std::str::FromStr;

use serde_aux::field_attributes::deserialize_number_from_string;
use sqlx::sqlite::SqliteConnectOptions;

#[derive(serde::Deserialize, Debug, Clone)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
    pub matrix: MatrixSettings,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct ApplicationSettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub base_url: String,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct DatabaseSettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub database_name: String,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct MatrixSettings {
    pub username: String,
    pub password: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = base_path.join("configuration");

    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT");

    let environment_filename = format!("{}.yaml", environment.as_str());

    let settings = config::Config::builder()
        .add_source(config::File::from(
            configuration_directory.join("base.yaml"),
        ))
        .add_source(config::File::from(
            configuration_directory.join(&environment_filename),
        ))
        .add_source(
            config::Environment::with_prefix("app")
                .prefix_separator("_")
                .separator("__"),
        )
        .build()?;

    settings.try_deserialize::<Settings>()
}

pub enum Environment {
    Local,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} is not a suported environment. Use either `local` or `production`.",
                other
            )),
        }
    }
}

impl DatabaseSettings {
    pub fn with_db(&self) -> SqliteConnectOptions {

    let conn = SqliteConnectOptions::from_str("sqlite:///Users/ailenrgrimaldi/matrix-llm-bot/rooms.db").expect("No se conectó el archivo de la base de datos.");

    conn
    }

    pub fn without_db(&self) -> SqliteConnectOptions {
        SqliteConnectOptions::new()
    }
}
