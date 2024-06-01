use axum::{response::IntoResponse, routing::post, Json, Router};
use serde::Deserialize;
use serde_json::json;
use tower_cookies::{Cookie, Cookies};
use tracing::{info, instrument};

use crate::{web, Error};

pub fn routes() -> Router {
    Router::new().route("/login", post(api_login))
}

#[instrument(skip_all)]
async fn api_login(cookies: Cookies, Json(payload): Json<LoginPayload>) -> impl IntoResponse {
    // TODO: mock auth
    if payload.username != "admin" || payload.password != "admin" {
        info!("invalid credentials");
        return Err(Error::LoginFail);
    }

    cookies.add(Cookie::new(web::AUTH_TOKEN, "user-1.exp.sign"));

    info!("login successful");

    Ok(Json(json!({
        "result": {
        "success": true
    }})))
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}
