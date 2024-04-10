use std::net::TcpListener;

use actix_web::{dev::Server, web, App, HttpServer, Result};
use matrix_sdk::Client as MatrixClient;
use sqlx::{postgres::PgPoolOptions, PgPool};
use tokio::sync::mpsc;
use tracing_actix_web::TracingLogger;

use crate::{
    configuration::{DatabaseSettings, Settings},
    routes::{create, health_check, invite, next},
};

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(
        configuration: Settings,
        matrix_client: &MatrixClient,
    ) -> Result<Self, anyhow::Error> {
        let connection_pool = get_connection_pool(&configuration.database);

        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        let listener = TcpListener::bind(&address)?;
        let port = listener.local_addr().unwrap().port();

        let server = run(listener, connection_pool, matrix_client.clone())?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub fn get_connection_pool(configuration: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.with_db())
}

fn run(
    listener: TcpListener,
    db_pool: PgPool,
    matrix_client: MatrixClient,
) -> anyhow::Result<Server, anyhow::Error> {
    let (tx, rx) = mpsc::channel(32);

    let db_client = web::Data::new(db_pool);
    let matrix = web::Data::new(matrix_client.clone());
    let tx = web::Data::new(tx.clone());

    tokio::spawn(create(rx, db_client.clone(), matrix.clone()));

    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/room/next", web::get().to(next))
            .route("/room/invite", web::post().to(invite))
            .app_data(tx.clone())
            .app_data(db_client.clone())
            .app_data(matrix.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
