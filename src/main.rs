mod ctx;
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

    let routes_apis = web::routes_tickets::routes(mc.clone())
        .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));

    let routes = Router::new()
        .route("/healthz", get(healthz_handler))
        .merge(web::routes_login::routes())
        .nest("/api", routes_apis)
        .layer(middleware::map_response(main_response_mapper))
        .layer(middleware::from_fn(web::mw_auth::mw_ctx_resolver))
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
