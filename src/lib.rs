//! Shared Rust foundation for trash-cli command migration.
//! This crate intentionally stays dependency-light and focuses on stable,
//! reusable primitives that can be consumed by command-specific crates.

pub mod errors;
pub mod fs;
pub mod helpers;
pub mod models;

pub use errors::{CoreError, Result};
pub use fs::{FileSystem, RealFileSystem};
pub use helpers::{
    build_unique_basename,
    parse_trash_datetime,
    print_size,
    sanitize_user_path,
    serialize_system_time,
    TRASHINFO_EXTENSION,
    TRASHINFO_TIME_FORMAT,
};
pub use models::{
    CommandContext,
    CommandKind,
    CommandOutcome,
    CommandOutput,
    ExitStatusLike,
    SkipReason,
    TrashCommand,
    TrashDirectory,
    TrashedItem,
};

/// Re-export a small stable API surface for command crates.
pub mod prelude {
    pub use crate::{
        errors::{CoreError, Result},
        fs::{FileSystem, RealFileSystem},
        helpers::*,
        models::*,
    };
}
