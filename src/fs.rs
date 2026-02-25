use crate::errors::CoreError;
use std::fs::{self, DirEntry, Metadata};
use std::io;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

/// Filesystem abstraction boundary for command implementations.
///
/// Keeping this trait narrow makes it easy to write deterministic tests and
/// allows alternative backends (e.g. in-memory fs) if command crates need it.
pub trait FileSystem: Send + Sync {
    /// Returns the current time in wall-clock format.
    fn now(&self) -> SystemTime;

    /// Returns true when path exists (symlink-aware).
    fn exists(&self, path: &Path) -> bool;

    /// Reads file metadata.
    fn metadata(&self, path: &Path) -> crate::Result<Metadata>;

    /// Reads symlink metadata.
    fn symlink_metadata(&self, path: &Path) -> crate::Result<Metadata>;

    /// Creates a directory and all missing parent directories.
    fn create_dir_all(&self, path: &Path) -> crate::Result<()>;

    /// Creates a directory.
    fn create_dir(&self, path: &Path) -> crate::Result<()>;

    /// Writes raw bytes atomically (truncate + replace).
    fn write(&self, path: &Path, data: &[u8]) -> crate::Result<()>;

    /// Writes UTF-8 text.
    fn write_to_string(&self, path: &Path, content: &str) -> crate::Result<()>;

    /// Reads UTF-8 text.
    fn read_to_string(&self, path: &Path) -> crate::Result<String>;

    /// Removes a file.
    fn remove_file(&self, path: &Path) -> crate::Result<()>;

    /// Renames/moves a path.
    fn rename(&self, from: &Path, to: &Path) -> crate::Result<()>;

    /// Lists directory children as concrete paths.
    fn list_dir(&self, path: &Path) -> crate::Result<Vec<PathBuf>>;

    /// Removes an empty directory.
    fn remove_dir(&self, path: &Path) -> crate::Result<()>;
}

/// Default filesystem implementation backed by `std::fs`.
#[derive(Debug, Default, Clone, Copy)]
pub struct RealFileSystem;

impl FileSystem for RealFileSystem {
    fn now(&self) -> SystemTime {
        SystemTime::now()
    }

    fn exists(&self, path: &Path) -> bool {
        path.exists()
    }

    fn metadata(&self, path: &Path) -> crate::Result<Metadata> {
        fs::metadata(path).map_err(|err| CoreError::io(path, err))
    }

    fn symlink_metadata(&self, path: &Path) -> crate::Result<Metadata> {
        fs::symlink_metadata(path).map_err(|err| CoreError::io(path, err))
    }

    fn create_dir_all(&self, path: &Path) -> crate::Result<()> {
        fs::create_dir_all(path).map_err(|err| CoreError::io(path, err))
    }

    fn create_dir(&self, path: &Path) -> crate::Result<()> {
        fs::create_dir(path).map_err(|err| CoreError::io(path, err))
    }

    fn write(&self, path: &Path, data: &[u8]) -> crate::Result<()> {
        fs::write(path, data).map_err(|err| CoreError::io(path, err))
    }

    fn write_to_string(&self, path: &Path, content: &str) -> crate::Result<()> {
        fs::write(path, content).map_err(|err| CoreError::io(path, err))
    }

    fn read_to_string(&self, path: &Path) -> crate::Result<String> {
        fs::read_to_string(path).map_err(|err| CoreError::io(path, err))
    }

    fn remove_file(&self, path: &Path) -> crate::Result<()> {
        fs::remove_file(path).map_err(|err| CoreError::io(path, err))
    }

    fn rename(&self, from: &Path, to: &Path) -> crate::Result<()> {
        fs::rename(from, to).map_err(|err| CoreError::io(from, err))
    }

    fn list_dir(&self, path: &Path) -> crate::Result<Vec<PathBuf>> {
        fs::read_dir(path)
            .map_err(|err| CoreError::io(path, err))?
            .map(|entry| entry.map(|v| v.path()))
            .collect::<Result<Vec<PathBuf>, io::Error>>()
            .map_err(|err| CoreError::io(path, err))
    }

    fn remove_dir(&self, path: &Path) -> crate::Result<()> {
        fs::remove_dir(path).map_err(|err| CoreError::io(path, err))
    }
}

/// Convenience helper for command implementations that repeatedly need the first
/// file-system entry when iterating directories.
pub fn first_entry_name(entries: &[DirEntry]) -> Option<String> {
    entries.first().and_then(|entry| {
        entry
            .file_name()
            .to_str()
            .map(|name| name.to_ascii_lowercase())
    })
}
