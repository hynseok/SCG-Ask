use crate::config::get_configuration;
use crate::routes::*;
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{dev::Server, http};
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let config = get_configuration().expect("Failed to read configuration.");
    let server = HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![
                        http::header::AUTHORIZATION,
                        http::header::ACCEPT,
                        http::header::CONTENT_TYPE,
                    ])
                    .max_age(3600),
            )
            .wrap(Logger::default())
            .route(
                &format!("{}/health_check", config.api_version),
                web::get().to(health_check),
            )
            .route(
                &format!("{}/send", config.api_version),
                web::post().to(send),
            )
    })
    .listen(listener)?
    .run();
    Ok(server)
}
