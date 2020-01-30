# Changelog

## [Unreleased](https://github.com/spenserblack/check-latest-rs/compare/v0.3.0...master)

## [0.3.0] 2020/01/30
### Changed
- `Result` type to be from [`anyhow`](https://crates.io/crates/anyhow) crate
- Max minor version and max patch to exclude yanked versions

### Fixed
- Unnecessary formatting on `user_agent`

## [0.2.1] 2020/01/29
### Fixed
- Some uses of `max_version!` macro failing to compile

## [0.2.0] 2020/01/28
### Added
- `async` feature
- `Result` type alias

### Changed
- Blocking requests to be enabled by `blocking` feature
- `Error` type to be a struct containing a `message`

### Fixed
- Documentation for `max_minor_version!` and `max_patch!` having examples for `max_version!`

## 0.1.0 2020/01/25
### Initial Version :tada:

[0.3.0]: https://github.com/spenserblack/check-latest-rs/compare/v0.2.1...v0.3.0
[0.2.1]: https://github.com/spenserblack/check-latest-rs/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/spenserblack/check-latest-rs/compare/v0.1.0...v0.2.0
