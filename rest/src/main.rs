mod log;
mod web;

use tickets::adapters::ticket_store::InMemory;
pub use tickets::error::{Error, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let tickets_store = InMemory::new().await?;

    web::start(tickets_store).await
}
