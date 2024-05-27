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
    //req_login.await?.print().await?;

    let req_create_ticket = hc.do_post(
        "/api/tickets",
        json!({
            "title": "Ticket A"
        }),
    );
    req_create_ticket.await?.print().await?;

    hc.do_delete("/api/tickets/1").await?.print().await?;
    hc.do_get("/api/tickets").await?.print().await?;

    Ok(())
}
