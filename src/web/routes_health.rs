use axum::{response::IntoResponse, routing::get, Router};

pub fn routes() -> Router {
    Router::new().route("/healtz", get(healthz_handler))
}

async fn healthz_handler() -> impl IntoResponse {
    println!("->> {:<12} - healthz", "HANDLER");

    ()
}
