pub mod adapters;
pub mod ports;
pub mod service;

mod domain;
mod error;

pub use domain::*;
pub use error::{Error, Result};
