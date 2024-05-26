use axum::{
    response::IntoResponse,
    routing::get,
    Router,
};

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

#[tokio::main]
async fn main() -> Result<()> {
    let routes = Router::new().route("/healthz", get(handler_hello));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("->> LISTENING on {}\n", listener.local_addr()?);
    axum::serve(listener, routes).await?;

    Ok(())
}

async fn handler_hello() -> impl IntoResponse {
    println!("->> {:<12} - healthz", "HANDLER");

    ()
}
