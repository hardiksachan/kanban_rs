use axum::{response::IntoResponse, routing::post, Json, Router};
use serde::Deserialize;
use serde_json::json;
use tower_cookies::{Cookie, Cookies};

use crate::{web, Error};

pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}

async fn api_login(
    cookies: Cookies,
    Json(payload): Json<LoginPayload>,
) -> impl IntoResponse {
    println!("->> {:<12} - api_login", "HANDLER");

    // TODO: mock auth
    if payload.username != "admin" || payload.password != "admin" {
        return Err(Error::LoginFail);
    }

    cookies.add(Cookie::new(web::AUTH_TOKEN, "user1.exp.sign"));

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
