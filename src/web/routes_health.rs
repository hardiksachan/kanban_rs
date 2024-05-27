use axum::{response::IntoResponse, routing::get, Router};
use tracing::{info, instrument};

pub fn routes() -> Router {
    Router::new().route("/healtz", get(healthz_handler))
}

#[instrument]
async fn healthz_handler() -> impl IntoResponse {
    info!("health: OK");
    ()
}
