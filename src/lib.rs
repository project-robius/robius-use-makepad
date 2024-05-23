//! Dummy crate that auto-configures all other `robius-*` crates 
//! in your dependency tree to work with Makepad.
//!
//! # Usage of this crate
//! If you're building a Makepad app that uses any `robius-*` crates,
//! simply add this to your `Cargo.toml` to ensure that those `robius-*` crates
//! are configured correctly to expect being used from within a Makepad app.
//!
//! That's all you need to do!
//!
//! ## More details
//! This crate exists as a convenient way to auto-configure all other `robius-*` crates
//! in your dependency tree to work with Makepad.
//! It doesn't have any code -- all it does is enable the `makepad` feature
//! in the [`robius-android-env`] crate, which is an "internal" crate that
//! many other `robius-*` crates depend on.
//!
//! [`robius-android-env`]: https://crates.io/crates/robius-android-env
//!