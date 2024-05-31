mod core;
mod ctx;
mod error;
mod log;
mod trace;
mod web;

pub use error::{Error, Result};

//use opentelemetry::global;
//use opentelemetry::sdk::propagation::TraceContextPropagator;
//use trace::open_telemetry::init_trace;
//use tracing_subscriber::layer::SubscriberExt;

#[tokio::main]
async fn main() -> Result<()> {
    //global::set_text_map_propagator(TraceContextPropagator::new());
    //let tracer = init_trace().unwrap();
    //let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);
    //let subscriber = tracing_subscriber::Registry::default().with(telemetry);
    //tracing::subscriber::set_global_default(subscriber).unwrap();

    //web::start().await

    Ok(())
}
