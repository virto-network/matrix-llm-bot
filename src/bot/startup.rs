use std::net::TcpListener;

use actix_cors::Cors;
use actix_web::{dev::Server, http, web, App, HttpServer, Result};
use matrix_sdk::Client as MatrixClient;
use tracing_actix_web::TracingLogger;

use crate::{
    configuration::Settings,
    routes::{cast_vote, create, create_initiative, get_by_alias, get_by_id, get_initiative_by_id, health_check, upload},
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
        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        let listener = TcpListener::bind(&address)?;
        let port = listener
            .local_addr()
            .expect("Should get the local_addr")
            .port();

        let origin = configuration.cors.origin;

        let server = run(listener, origin, matrix_client.clone())?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

fn run(
    listener: TcpListener,
    origin: Vec<String>,
    matrix_client: MatrixClient,
) -> anyhow::Result<Server, anyhow::Error> {
    let matrix: web::Data<MatrixClient> = web::Data::new(matrix_client.clone());

    let server = HttpServer::new(move || {
        let cors = origin
            .iter()
            .fold(Cors::default(), |acc, origin| acc.allowed_origin(&origin));

        let cors = cors
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(TracingLogger::default())
            .wrap(cors)
            .route("/health_check", web::get().to(health_check))
            .route("/room/upload", web::post().to(upload))
            .route("/room/alias/{id}", web::get().to(get_by_alias))
            .route("/room/id/{id}", web::get().to(get_by_id))
            .route("/room/create", web::post().to(create))
            .route("/initiative/create", web::post().to(create_initiative))
            .route("/initiative/vote", web::post().to(cast_vote))
            .route("/initiative/{id}", web::get().to(get_initiative_by_id))
            .app_data(matrix.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
