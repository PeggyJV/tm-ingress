//! CosmosTxEndpoint
//!
//! Application based on the [Abscissa] framework.
//!
//! [Abscissa]: https://github.com/iqlusioninc/abscissa

// Tip: Deny warnings with `RUSTFLAGS="-D warnings"` environment variable in CI

#![forbid(unsafe_code)]
#![warn(
    rust_2018_idioms,
    trivial_casts,
    unused_lifetimes,
    unused_qualifications
)]

pub mod application;
mod commands;
mod config;
mod error;
mod grpc;
mod handlers;
mod prelude;
mod rpc;
// mod reflection;
