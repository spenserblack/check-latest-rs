# check-latest
[![Crates.io](https://img.shields.io/crates/v/check-latest)](https://crates.io/crates/check-latest/)
[![Docs.rs](https://docs.rs/check-latest/badge.svg)](https://docs.rs/check-latest/)
![Crates.io](https://img.shields.io/crates/d/check-latest)
[![Build Status](https://travis-ci.com/spenserblack/check-latest-rs.svg?branch=master)](https://travis-ci.com/spenserblack/check-latest-rs)
[![CI](https://github.com/spenserblack/check-latest-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/spenserblack/check-latest-rs/actions/workflows/ci.yml)

Check if your rust executable is the latest available version on [Crates.io]

# The Basics

```rust
use check_latest::check_max;

if let Ok(Some(version)) = check_max!() {
    println!("Version {} is now available!", version);
}
```

*Please check the examples and [the documentation](https://docs.rs/check-latest/) For more usage.*

# Features
This crate has two features: `async` and `blocking`.
By default, `blocking` is enabled and `async` is disabled. This default is compatible with the
example in the section titled **The Basics**. If you want to use asynchronous requests, you can
swap these features with the following in you `Cargo.toml`.
```toml
[dependencies.check-latest]
version = "*"
default-features = false
features = ["async"]
```
*__NOTE__ There's nothing stopping you from enabling both `async` and `blocking`, but that's
unlikely to be necessary.*

# Notes

## Making this Library Optional

If you use this library for your binary, you should probably make this an optional feature.
Simply checking for the latest version on [Crates.io] brings over *a lot* of dependencies
in order to send a request to the API and parse the response. Some users may want to turn
off this feature for a smaller binary. Some may simply prefer not to be told to install an update.

You can make this feature optional by adding this to your `Cargo.toml`.
```toml
[dependencies.check-latest]
version = "*"
optional = true
```
To selectively compile the parts of your binary that check for later releases, add this attribute
to the parts that should be compiled if this feature is enabled.
```rust
#[cfg(feature = "check-latest")]
```

[Crates.io]: https://crates.io/
