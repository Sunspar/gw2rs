//! Rust bindings for the ArenaNet Guild Wars 2 API.

#![deny(missing_debug_implementations)]
//#![deny(missing_docs)]

// Async HTTP Stuffs
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;
extern crate url;

// Serde
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

// Logging
#[macro_use]
extern crate log;

// Modules and exports
#[macro_use]
pub(crate) mod macros;
pub(crate) mod anet;
pub(crate) mod internal;
pub mod prelude;
