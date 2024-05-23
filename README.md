# robius-use-makepad

[![Latest Version](https://img.shields.io/crates/v/robius-use-makepad.svg)](https://crates.io/crates/robius-use-makepad)
[![Docs](https://docs.rs/robius-use-makepad/badge.svg)](https://docs.rs/robius-use-makepad/latest/robius_use_makepad/)
[![Project Robius Matrix Chat](https://img.shields.io/matrix/robius-general%3Amatrix.org?server_fqdn=matrix.org&style=flat&logo=matrix&label=Project%20Robius%20Matrix%20Chat&color=B7410E)](https://matrix.to/#/#robius:matrix.org)

This crate simply auto-configures all other `robius-*` crates 
in your dependency tree to work with Makepad.

## Usage of this crate
If you're building a Makepad app that uses any `robius-*` crates,
simply add this to your `Cargo.toml` to ensure that those `robius-*` crates
are configured correctly to expect being used from within a Makepad app.

That's all you need to do!

### More details
This crate exists as a convenient way to auto-configure all other `robius-*` crates
in your dependency tree to work with Makepad.
It doesn't have any code -- all it does is enable the `makepad` feature
in the [`robius-android-env`] crate, which is an "internal" crate that
many other `robius-*` crates depend on.

Simply depending on this crate will auto-configure all Robius crates to work with Makepad.

[`robius-android-env`]: https://crates.io/crates/robius-android-env
