# check-latest
[![Crates.io](https://img.shields.io/crates/v/check-latest)](https://crates.io/crates/check-latest/)
[![Docs.rs](https://docs.rs/check-latest/badge.svg)](https://docs.rs/check-latest/)
![Crates.io](https://img.shields.io/crates/d/check-latest)
[![Build Status](https://travis-ci.com/spenserblack/check-latest-rs.svg?branch=master)](https://travis-ci.com/spenserblack/check-latest-rs)

Check if your rust executable is the latest available version on [Crates.io]

# The Basics

```rust
if let Ok(Some(version)) = max_version!() {
    println!("We've released a new version: {}!", version);
}
```

# Notes

Using this library in your

| Rust Library | Rust Binary |
| :----------: | :---------: |
|      🤢      |     👍      |

If you use this library for your binary, you should probably make this an optional feature.
Simply checking for the latest version on [Crates.io] brings over *a lot* of dependencies
in order to send a request to the API and parse the response. Some users may want to turn
off this feature for a smaller binary. Some may simply prefer not to be told to install an update.

You can make this feature optional by adding this to your `Cargo.toml`.
```toml
[dependencies]
check-latest = { version = "*", optional = true }
```
You can then selectively compile the parts of your binary that check for later releases with this attribute
to the parts that should be compiled if this feature is enabled.
```rust
#[cfg(feature = "check-latest")]
```

[Crates.io]: https://crates.io/
