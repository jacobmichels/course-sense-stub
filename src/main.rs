use color_eyre::{eyre::Context, Result};
use log::info;
use std::env;

use axum::{extract::Request, Router};

#[tokio::main]
async fn main() {
    let _ = dotenvy::dotenv(); // ignoring the error if the .env file is not found
    color_eyre::install().expect("failed to install color_eyre");
    env_logger::init();

    run().await.unwrap();
}

async fn run() -> Result<()> {
    let port = env::var("PORT").wrap_err("$PORT not defined")?;

    let app = Router::new().fallback(handler);

    info!("Server running on port {}", port);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .wrap_err("failed to find to port: {port}")?;
    axum::serve(listener, app)
        .await
        .wrap_err("failed to serve app")?;

    Ok(())
}

async fn handler(req: Request) -> &'static str {
    info!("Request received: {:?}", req);
    "Hi, thanks for using Course Sense. Unfortunately, it's been discontinued. Thanks for your support."
}
