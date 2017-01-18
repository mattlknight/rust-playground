//! This is documentation for the `mylib::server` module.
//!
//! # Examples

pub mod errors;
pub mod traits;
pub mod types;
pub mod routes;
mod common;
mod start_server;

pub use self::common::{get_body, get_user, send_json_error, json_reply};
pub use self::start_server::start_server;
