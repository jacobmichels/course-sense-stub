use std::env;

use axum::{extract::Request, Router};

#[tokio::main]
async fn main() {
    let port = env::var("PORT").unwrap_or("3000".to_string());

    let app = Router::new().fallback(handler);

    println!("Server running on port {}", port);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handler(req: Request) -> &'static str {
    println!("Request received: {:?}", req);
    "Hi, thanks for using Course Sense. Unfortunately, it's been discontinued. Thanks for your support."
}
