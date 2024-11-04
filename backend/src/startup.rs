use crate::routes::*;
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{dev::Server, http};
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
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
                "/health_check",
                web::get().to(health_check),
            )
            .route(
                "/send",
                web::post().to(send),
            )
    })
    .listen(listener)?
    .run();
    Ok(server)
}
