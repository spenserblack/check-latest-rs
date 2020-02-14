# Changelog

## [Unreleased](https://github.com/spenserblack/check-latest-rs/compare/v0.3.0...master)
### Added
- `Version`
- `check_max!`
- `check_minor!`
- `check_patch!`
- `check_max_async!`
- `check_minor_async!`
- `check_patch_async!`
- `new_versions!`
- `new_versions_async!`
- `MaxAndNew` to support deprecated features

### Changed
- `Versions` to contain list of `Version`s

### Deprecated
- `async::get_versions`
- `async::get_max_version`
- `async::get_max_minor_version`
- `async::get_max_patch`
- `async::get_newest_version`
- `blocking::get_versions`
- `blocking::get_max_version`
- `blocking::get_max_minor_version`
- `blocking::get_max_patch`
- `blocking::get_newest_version`
- `max_version!`
- `max_minor_version!`
- `max_patch!`;
- `max_version_async!`
- `max_minor_version_async!`
- `max_patch_async!`;

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
