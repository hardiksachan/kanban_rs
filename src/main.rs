mod error;

pub use error::{Error, Result};

use axum::{response::IntoResponse, routing::get, Router};

#[tokio::main]
async fn main() {
    let routes = Router::new().route("/healthz", get(healthz_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("->> LISTENING on {}\n", listener.local_addr().unwrap());
    axum::serve(listener, routes).await.unwrap();
}

async fn healthz_handler() -> impl IntoResponse {
    println!("->> {:<12} - healthz", "HANDLER");

    ()
}
