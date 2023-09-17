//! # `cufarul::repo`
//!
//! `cufarul::repo` implements the from-disk repository loading and syncing.
//! It is only responsible for repository level deserialization and thread-safe
//! update of the content. The individual document deserialization is not part
//! of this module.
//!
//! The entry point to the repository is throght the `CufarulRepository` struct.
mod core;
mod cufarul;
mod error;
mod index;
mod spec;

pub use self::core::{Cufarul, Repository};
pub use self::cufarul::CufarulRepository;
pub use self::error::{Error, Result};
pub use self::spec::RepositorySpec;

const REPOSITORY_CONFIG_FILE: &str = ".cufarul";
const REPOSITORY_SUPPORTED_VERSION: u8 = 0;
