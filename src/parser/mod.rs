pub mod cargo;
pub mod metadata;

pub use cargo::CargoParser;
pub use cargo::CargoManifest;
pub use cargo::Dependency;
pub use metadata::{MetadataParser, PackageInfo};