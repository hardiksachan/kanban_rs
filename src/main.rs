mod ctx;
mod error;
mod log;
mod model;
mod web;

pub use error::{Error, Result};

#[tokio::main]
async fn main() -> Result<()> {
    web::start().await
}
