//! # The Bugsnag api
//!
//! This crate provides an interface for reporting messages to Bugsnag.
//!
//! # Example
//!
//! ```
//! let mut api = bugsnag::Bugsnag::new("api-key", env!("CARGO_MANIFEST_DIR"));
//!
//! // setting the appinfo is not required, but recommended
//! api.set_app_info(Some(env!("CARGO_PKG_VERSION")),
//!                  Some("development"),
//!                  Some("rust"));
//!
//! api.notify("Info", "This is a message from the rust bugsnag api.")
//!       .severity(bugsnag::Severity::Info);
//! ```
//!
//! For more examples on how to integrate bugsnag into a project, the examples
//! folder provides some reference implementations.

mod bugsnag_impl;
mod event;
mod exception;
mod notification;
mod stacktrace;
pub use self::bugsnag_impl::*;
mod appinfo;
mod deviceinfo;
pub mod panic;
pub mod user;
