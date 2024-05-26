use axum::{response::Html, routing::get, Router};

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

#[tokio::main]
async fn main() -> Result<()> {
    let routes_hello = Router::new().route(
        "/hello",
        get(|| async { Html("Hello, world!") })
    );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("->> LISTENING on {}\n", listener.local_addr()?);
    axum::serve(listener, routes_hello).await?;

    Ok(())
}
