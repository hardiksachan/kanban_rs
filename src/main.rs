mod ctx;
mod error;
mod model;
mod web;

pub use error::{Error, Result};

use axum::{
    middleware, response::{IntoResponse, Response}, routing::get, Json, Router
};
use serde_json::json;
use tower_cookies::CookieManagerLayer;
use uuid::Uuid;

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
    let uuid = Uuid::new_v4();

    let service_error = res.extensions().get::<Error>();
    let client_status_error = service_error.map(|se| se.client_status_and_error());

    let error_response = client_status_error
        .as_ref()
        .map(|(status_code, client_error)| {
            let client_error_body = json!({
                "error": {
                    "type": client_error.as_ref(),
                    "req_uuid": uuid.to_string(),
            }
            });

            println!("    ->> client_error_body: {client_error_body}");

            (*status_code, Json(client_error_body)).into_response()
        });

    println!("    ->> server log line - {uuid} - Error: {service_error:?}");

    println!();
    error_response.unwrap_or(res)
}

async fn healthz_handler() -> impl IntoResponse {
    println!("->> {:<12} - healthz", "HANDLER");

    ()
}
