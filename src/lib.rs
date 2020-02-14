//! Depending on the features you have enabled/disabled, you should view the
//! documentation for `blocking` and/or `async` for basic usage.
//!
//! # Features
//! ## `blocking`
//!
//! This feature is enabled by default.
//!
//! Provides the basic usage.
//!
//! ## `async`
//!
//! Allows you to asynchronously check for available versions.
//! If enabled, it will provide async versions of the macros, which can be used
//! with `<macro_name>_async!` For example, `max_version_async!`.
//!
//! ```toml
//! [dependencies.check-latest]
//! default-features = false # If you want async, you probably don't want blocking
//! features = ["async"]
//! ```

#![deny(missing_docs)]

use chrono::{DateTime, Utc};
use semver::Version as SemVer;
use serde::Deserialize;
use std::cmp::Ordering;
use std::fmt::{self, Display};

/// A collection of `Version`s.
#[derive(Debug, Deserialize)]
pub struct Versions {
    versions: Vec<Version>,
}

/// A release to [Crates.io].
///
/// [Crates.io]: https://crates.io/
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct Version {
    #[serde(rename = "num")]
    version: SemVer,
    /// If this version was yanked
    pub yanked: bool,
    /// When this version was published
    pub created_at: DateTime<Utc>,
}

impl Versions {
    /// Gets *any* max version.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use check_latest::Versions;
    ///
    /// let newest = Versions::new("my-cool-crate", "my-cool-crate/1.0.0")
    ///     .unwrap()
    ///     .max_version();
    /// ```
    pub fn max_version(&self) -> Option<&Version> {
        self.versions
            .iter()
            .max_by(|v1, v2| v1.version.cmp(&v2.version))
    }
    /// Gets the max version that hasn't been yanked.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use check_latest::Versions;
    ///
    /// let newest = Versions::new("my-cool-crate", "my-cool-crate/1.0.0")
    ///     .unwrap()
    ///     .max_unyanked_version();
    /// ```
    pub fn max_unyanked_version(&self) -> Option<&Version> {
        self.versions
            .iter()
            .filter(|v| !v.yanked)
            .max_by(|v1, v2| v1.version.cmp(&v2.version))
    }
    /// Gets max version that has been yanked.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use check_latest::Versions;
    ///
    /// let newest = Versions::new("my-cool-crate", "my-cool-crate/1.0.0")
    ///     .unwrap()
    ///     .newest_yanked_version();
    /// ```
    pub fn max_yanked_version(&self) -> Option<&Version> {
        self.versions
            .iter()
            .filter(|v| v.yanked)
            .max_by(|v1, v2| v1.version.cmp(&v2.version))
    }
    /// Gets *any* max version with the same major version.
    ///
    /// For example, if `major` = 1, then `1.0.0 <= max_minor_version < 2.0.0`.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use check_latest::Versions;
    ///
    /// let newest = Versions::new("my-cool-crate", "my-cool-crate/1.0.0")
    ///     .unwrap()
    ///     .max_minor_version(1);
    /// ```
    pub fn max_minor_version(&self, major: u64) -> Option<&Version> {
        self.versions
            .iter()
            .filter(|v| v.major() == major)
            .max_by(|v1, v2| v1.version.cmp(&v2.version))
    }
    /// Gets the max version that hasn't been yanked with the same major
    /// version.
    ///
    /// For example, if `major` = 1, then `1.0.0 <= max_minor_version < 2.0.0`.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use check_latest::Versions;
    ///
    /// let newest = Versions::new("my-cool-crate", "my-cool-crate/1.0.0")
    ///     .unwrap()
    ///     .max_unyanked_minor_version(1);
    /// ```
    pub fn max_unyanked_minor_version(&self, major: u64) -> Option<&Version> {
        self.versions
            .iter()
            .filter(|v| !v.yanked)
            .filter(|v| v.major() == major)
            .max_by(|v1, v2| v1.version.cmp(&v2.version))
    }
    /// Gets max version that has been yanked with the same major version.
    ///
    /// For example, if `major` = 1, then `1.0.0 <= max_minor_version < 2.0.0`.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use check_latest::Versions;
    ///
    /// let newest = Versions::new("my-cool-crate", "my-cool-crate/1.0.0")
    ///     .unwrap()
    ///     .max_yanked_minor_version(1);
    /// ```
    pub fn max_yanked_minor_version(&self, major: u64) -> Option<&Version> {
        self.versions
            .iter()
            .filter(|v| v.yanked)
            .filter(|v| v.major() == major)
            .max_by(|v1, v2| v1.version.cmp(&v2.version))
    }
    /// Gets *any* max version with the same major and minor version.
    ///
    /// For example, if `major` = 1 and `minor` = 2,
    /// then `1.2.0 <= max_patch < 1.3.0`.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use check_latest::Versions;
    ///
    /// let newest = Versions::new("my-cool-crate", "my-cool-crate/1.0.0")
    ///     .unwrap()
    ///     .max_patch(1, 2);
    /// ```
    pub fn max_patch(&self, major: u64, minor: u64) -> Option<&Version> {
        self.versions
            .iter()
            .filter(|v| v.major() == major)
            .filter(|v| v.minor() == minor)
            .max_by(|v1, v2| v1.version.cmp(&v2.version))
    }
    /// Gets the max version that hasn't been yanked with the same major
    /// and minor version.
    ///
    /// For example, if `major` = 1 and `minor` = 2,
    /// then `1.2.0 <= max_patch < 1.3.0`.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use check_latest::Versions;
    ///
    /// let newest = Versions::new("my-cool-crate", "my-cool-crate/1.0.0")
    ///     .unwrap()
    ///     .max_unyanked_patch(1, 2);
    /// ```
    pub fn max_unyanked_patch(&self, major: u64, minor: u64) -> Option<&Version> {
        self.versions
            .iter()
            .filter(|v| !v.yanked)
            .filter(|v| v.major() == major)
            .filter(|v| v.minor() == minor)
            .max_by(|v1, v2| v1.version.cmp(&v2.version))
    }
    /// Gets max version that has been yanked with the same major and minor
    /// version.
    ///
    /// For example, if `major` = 1 and `minor` = 2,
    /// then `1.2.0 <= max_patch < 1.3.0`.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use check_latest::Versions;
    ///
    /// let newest = Versions::new("my-cool-crate", "my-cool-crate/1.0.0")
    ///     .unwrap()
    ///     .max_yanked_patch(1, 2);
    /// ```
    pub fn max_yanked_patch(&self, major: u64, minor: u64) -> Option<&Version> {
        self.versions
            .iter()
            .filter(|v| v.yanked)
            .filter(|v| v.major() == major)
            .filter(|v| v.minor() == minor)
            .max_by(|v1, v2| v1.version.cmp(&v2.version))
    }
    /// Gets *any* newest version.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use check_latest::Versions;
    ///
    /// let newest = Versions::new("my-cool-crate", "my-cool-crate/1.0.0")
    ///     .unwrap()
    ///     .newest_version();
    /// ```
    pub fn newest_version(&self) -> Option<&Version> {
        self.versions
            .iter()
            .max_by(|v1, v2| v1.created_at.cmp(&v2.created_at))
    }
    /// Gets the newest version that hasn't been yanked.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use check_latest::Versions;
    ///
    /// let newest = Versions::new("my-cool-crate", "my-cool-crate/1.0.0")
    ///     .unwrap()
    ///     .newest_unyanked_version();
    /// ```
    pub fn newest_unyanked_version(&self) -> Option<&Version> {
        self.versions
            .iter()
            .filter(|v| !v.yanked)
            .max_by(|v1, v2| v1.created_at.cmp(&v2.created_at))
    }
    /// Gets newest version that has been yanked.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use check_latest::Versions;
    ///
    /// let newest = Versions::new("my-cool-crate", "my-cool-crate/1.0.0")
    ///     .unwrap()
    ///     .newest_yanked_version();
    /// ```
    pub fn newest_yanked_version(&self) -> Option<&Version> {
        self.versions
            .iter()
            .filter(|v| v.yanked)
            .max_by(|v1, v2| v1.created_at.cmp(&v2.created_at))
    }
    /// Gets the full list of versions that were found.
    pub fn versions(&self) -> &Vec<Version> {
        &self.versions
    }
    /// Gets a mutable list of versions that were found.
    pub fn versions_mut(&mut self) -> &mut Vec<Version> {
        &mut self.versions
    }
    /// Takes ownership of `self` and returns owned versions list.
    pub fn versions_owned(self) -> Vec<Version> {
        self.versions
    }
}

impl Version {
    /// Gets the SemVer MAJOR version
    pub fn major(&self) -> u64 {
        self.version.major
    }
    /// Gets the SemVer MINOR version
    pub fn minor(&self) -> u64 {
        self.version.minor
    }
    /// Gets the SemVer PATCH version
    pub fn patch(&self) -> u64 {
        self.version.patch
    }
}

impl PartialEq<SemVer> for Version {
    fn eq(&self, rhs: &SemVer) -> bool {
        self.version.eq(&rhs)
    }
}

impl PartialEq<str> for Version {
    fn eq(&self, rhs: &str) -> bool {
        match SemVer::parse(rhs) {
            Ok(version) => self.eq(&version),
            Err(_) => false,
        }
    }
}

impl PartialEq<&str> for Version {
    fn eq(&self, rhs: &&str) -> bool {
        self.eq(rhs.to_owned())
    }
}

impl PartialOrd<SemVer> for Version {
    fn partial_cmp(&self, rhs: &SemVer) -> Option<Ordering> {
        self.version.partial_cmp(rhs)
    }
}

impl PartialOrd<str> for Version {
    fn partial_cmp(&self, rhs: &str) -> Option<Ordering> {
        match SemVer::parse(rhs) {
            Ok(version) => self.partial_cmp(&version),
            Err(_) => None,
        }
    }
}

impl PartialOrd<&str> for Version {
    fn partial_cmp(&self, rhs: &&str) -> Option<Ordering> {
        self.partial_cmp(rhs.to_owned())
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.version)?;
        if self.yanked {
            write!(f, " (yanked)")
        } else {
            Ok(())
        }
    }
}

impl From<Version> for SemVer {
    fn from(v: Version) -> SemVer {
        v.version
    }
}

fn build_url(crate_name: &str) -> String {
    format!(
        "https://crates.io/api/v1/crates/{crate_name}",
        crate_name = crate_name,
    )
}

/// Check for version updates with asynchronous requests.
#[cfg(feature = "async")]
pub mod r#async;

/// Check for version updates with blocking requests.
#[cfg(feature = "blocking")]
pub mod blocking;

/// Gets the name of the crate as defined in *your* `Cargo.toml`.
#[macro_export]
macro_rules! crate_name {
    () => {
        env!("CARGO_PKG_NAME")
    };
}

/// Gets the version of the crate as defined in *your* `Cargo.toml`.
///
/// Will be `&str`
#[macro_export]
macro_rules! crate_version {
    () => {
        env!("CARGO_PKG_VERSION")
    };
}

/// Gets the major version of the crate as defined in *your* `Cargo.toml`.
///
/// Will be `&str`
#[macro_export]
macro_rules! crate_major_version {
    () => {
        env!("CARGO_PKG_VERSION_MAJOR")
    };
}

/// Gets the minor version of the crate as defined in *your* `Cargo.toml`.
///
/// Will be `&str`
#[macro_export]
macro_rules! crate_minor_version {
    () => {
        env!("CARGO_PKG_VERSION_MINOR")
    };
}

/// Gets the patch version of the crate as defined in *your* `Cargo.toml`.
///
/// Will be `&str`
#[macro_export]
macro_rules! crate_patch {
    () => {
        env!("CARGO_PKG_VERSION_PATCH")
    };
}

/// Defines an appropriate user agent for making requests.
///
/// `"<your-crate-name>/<version>"`
#[macro_export]
macro_rules! user_agent {
    () => {
        concat!($crate::crate_name!(), "/", $crate::crate_version!())
    };
}

#[derive(Deserialize)]
struct CratesioResponse {
    #[serde(rename = "crate")]
    versions: MaxAndNew,
    all_versions: Vec<VersionListItem>,
}

#[derive(Deserialize)]
#[deprecated(since = "0.4")]
/// Maintains compatibility with deprecated `fn`s.
pub struct MaxAndNew {
    /// The max version according to the [Crates.io] API
    ///
    /// ```json
    /// {
    ///   "crate": {
    ///     "max_version": "<version>"
    ///   }
    /// }
    /// ```
    ///
    /// [Crates.io]: https://crates.io/
    pub max_version: SemVer,
    /// The newest version according to the [Crates.io] API
    ///
    /// ```json
    /// {
    ///   "crate": {
    ///     "newest_version": "<version>"
    ///   }
    /// }
    /// ```
    ///
    /// [Crates.io]: https://crates.io/
    pub newest_version: SemVer,
}

#[derive(Deserialize)]
struct VersionListItem {
    #[serde(rename = "num")]
    version: SemVer,
    yanked: bool,
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDateTime;
    use lazy_static::lazy_static;
    use super::*;

    lazy_static! {
        static ref DONT_CARE_DATETIME: DateTime<Utc> = {
            let naive = NaiveDateTime::from_timestamp(0, 0);
            DateTime::from_utc(naive, Utc)
        };
    }

    #[test]
    fn is_greater_semver() {
        let version = Version {
            version: SemVer::parse("1.2.3").unwrap(),
            yanked: false,
            created_at: DONT_CARE_DATETIME.clone(),
        };
        let semver = SemVer::parse("1.2.0").unwrap();
        assert!(version > semver);
    }

    #[test]
    fn is_lesser_semver() {
        let version = Version {
            version: SemVer::parse("1.2.3").unwrap(),
            yanked: false,
            created_at: DONT_CARE_DATETIME.clone(),
        };
        let semver = SemVer::parse("1.3.0").unwrap();
        assert!(version < semver);
    }

    #[test]
    fn is_greater_str() {
        let version = Version {
            version: SemVer::parse("1.2.3").unwrap(),
            yanked: false,
            created_at: DONT_CARE_DATETIME.clone(),
        };
        assert!(version > "1.2.0");
    }

    #[test]
    fn is_lesser_str() {
        let version = Version {
            version: SemVer::parse("1.2.3").unwrap(),
            yanked: false,
            created_at: DONT_CARE_DATETIME.clone(),
        };
        assert!(version < "1.3.0");
    }
}
