use backend::{config::get_configuration, startup::run};
use std::net::TcpListener;
use env_logger::Env;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    
    let config = get_configuration().expect("Failed to read configuration.");
    let address = format!("0.0.0.0:{}", config.application_port);

    let listener = TcpListener::bind(address)?;
    run(listener)?.await
}
