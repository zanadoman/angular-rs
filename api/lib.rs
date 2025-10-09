#![feature(bool_to_result)]
#![forbid(unsafe_code)]
#![deny(warnings)]
#![warn(clippy::nursery, clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]

pub use self::router::new;
use self::{
    app_state::AppState,
    authenticator::{Authenticator, Credentials},
    handlers::HandlerError,
    models::FieldError,
};

mod app_state;
mod authenticator;
mod handlers;
mod models;
mod router;
