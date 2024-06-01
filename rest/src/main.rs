mod error;
mod extract;
mod log;
mod mw_auth;
mod mw_res_map;
mod routes_health;
mod routes_login;
mod routes_tickets;

use axum::{middleware, Router};
use tickets::adapters::ticket_store::InMemory;
use tickets::core::{self, ports::TicketStore};
use tower_cookies::CookieManagerLayer;
use tracing::info;

pub use error::{Error, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let tickets_store = InMemory::new().await?;

    start(tickets_store).await
}

pub const AUTH_TOKEN: &str = "auth-token";

pub async fn start(ticket_store: impl TicketStore + 'static) -> Result<()> {
    let ticket_routes = routes_tickets::routes()
        .route_layer(middleware::from_fn(mw_auth::mw_require_auth))
        .with_state(core::services::Ticket::with_store(ticket_store));

    let routes = Router::new()
        .nest("/api", routes_login::routes())
        .nest("/api", routes_health::routes())
        .nest("/api", ticket_routes)
        .layer(middleware::map_response(mw_res_map::main_response_mapper))
        .layer(middleware::from_fn(mw_auth::mw_ctx_resolver))
        .layer(CookieManagerLayer::new());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("LISTENING on {}\n", listener.local_addr().unwrap());
    axum::serve(listener, routes).await.unwrap();

    Ok(())
}
