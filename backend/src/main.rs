use backend::{config::get_configuration, startup::run};
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = get_configuration().expect("Failed to read configuration.");
    let address = format!("0.0.0.0:{}", config.application_port);

    let listener = TcpListener::bind(address)?;
    run(listener)?.await
}
