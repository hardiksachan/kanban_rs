mod mw_auth;
mod mw_res_map;
mod routes_health;
mod routes_login;
mod routes_tickets;

pub const AUTH_TOKEN: &str = "auth-token";

use crate::error::Result;
use crate::model::ModelController;
use axum::{middleware, Router};
use tower_cookies::CookieManagerLayer;
use tracing::info;

pub async fn start() -> Result<()> {
    let mc = ModelController::new().await?;

    let routes_apis = routes_tickets::routes(mc.clone())
        .route_layer(middleware::from_fn(mw_auth::mw_require_auth));

    let routes = Router::new()
        .nest("/api", routes_login::routes())
        .nest("/api", routes_health::routes())
        .nest("/api", routes_apis)
        .layer(middleware::map_response(mw_res_map::main_response_mapper))
        .layer(middleware::from_fn(mw_auth::mw_ctx_resolver))
        .layer(CookieManagerLayer::new());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("LISTENING on {}\n", listener.local_addr().unwrap());
    axum::serve(listener, routes).await.unwrap();

    Ok(())
}
