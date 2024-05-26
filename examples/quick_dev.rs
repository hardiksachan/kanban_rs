#![allow(unused)] // For example code.

use serde_json::json;

pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>; // For examples.

#[tokio::main]
async fn main() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3000")?;

    hc.do_get("/healthz").await?.print().await?;

    let req_login = hc.do_post(
        "/api/login",
        json!({
            "username": "admin",
            "password": "admin"
        }),
    );
    req_login.await?.print().await?;

    Ok(())
}
