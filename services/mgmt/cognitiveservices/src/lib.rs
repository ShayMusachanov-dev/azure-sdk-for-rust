#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![doc = "generated by AutoRust"]
#[cfg(feature = "package-2022-03")]
pub mod package_2022_03;
#[cfg(all(feature = "package-2022-03", not(feature = "no-default-tag")))]
pub use package_2022_03::{models, operations, operations::Client, operations::ClientBuilder};
#[cfg(feature = "package-2021-10")]
pub mod package_2021_10;
#[cfg(all(feature = "package-2021-10", not(feature = "no-default-tag")))]
pub use package_2021_10::{models, operations, operations::Client, operations::ClientBuilder};
#[cfg(feature = "package-2021-04")]
pub mod package_2021_04;
#[cfg(all(feature = "package-2021-04", not(feature = "no-default-tag")))]
pub use package_2021_04::{models, operations, operations::Client, operations::ClientBuilder};
#[cfg(feature = "package-2017-04")]
pub mod package_2017_04;
#[cfg(all(feature = "package-2017-04", not(feature = "no-default-tag")))]
pub use package_2017_04::{models, operations, operations::Client, operations::ClientBuilder};
#[cfg(feature = "package-2016-02-preview")]
pub mod package_2016_02_preview;
#[cfg(all(feature = "package-2016-02-preview", not(feature = "no-default-tag")))]
pub use package_2016_02_preview::{models, operations, operations::Client, operations::ClientBuilder};
