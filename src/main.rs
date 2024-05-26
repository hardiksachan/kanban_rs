mod error;
mod model;
mod web;

pub use error::{Error, Result};

use axum::{
    middleware,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use tower_cookies::CookieManagerLayer;

use crate::model::ModelController;

#[tokio::main]
async fn main() -> Result<()> {
    let mc = ModelController::new().await?;

    let routes = Router::new()
        .merge(web::routes_login::routes())
        .nest("/api", web::routes_tickets::routes(mc))
        .route("/healthz", get(healthz_handler))
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("->> LISTENING on {}\n", listener.local_addr().unwrap());
    axum::serve(listener, routes).await.unwrap();

    Ok(())
}

async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");

    println!();
    res
}

async fn healthz_handler() -> impl IntoResponse {
    println!("->> {:<12} - healthz", "HANDLER");

    ()
}
