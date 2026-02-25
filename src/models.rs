use chrono::{DateTime, Utc};
use std::path::PathBuf;

/// Command family being implemented in Rust (or wrapped).
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum CommandKind {
    Put,
    List,
    Empty,
    Restore,
    Remove,
}

impl CommandKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Put => "put",
            Self::List => "list",
            Self::Empty => "empty",
            Self::Restore => "restore",
            Self::Remove => "rm",
        }
    }
}

impl std::fmt::Display for CommandKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Base command metadata shared by all migration-ready command implementations.
#[derive(Debug, Clone)]
pub struct CommandContext {
    pub command: CommandKind,
    pub args: Vec<String>,
    pub cwd: PathBuf,
    pub interactive: bool,
    pub dry_run: bool,
}

#[derive(Debug, Clone)]
pub struct CommandOutput {
    pub command: CommandKind,
    pub stdout: Vec<String>,
    pub stderr: Vec<String>,
    pub exit_code: u8,
}

impl CommandOutput {
    pub fn success(command: CommandKind, stdout: impl Into<Vec<String>>) -> Self {
        Self {
            command,
            stdout: stdout.into(),
            stderr: Vec::new(),
            exit_code: 0,
        }
    }

    pub fn with_error(command: CommandKind, stderr: impl Into<String>) -> Self {
        Self {
            command,
            stdout: Vec::new(),
            stderr: vec![stderr.into()],
            exit_code: 1,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TrashDirectory {
    pub path: PathBuf,
    pub files_dir: PathBuf,
    pub info_dir: PathBuf,
    pub mount_point: Option<PathBuf>,
}

impl TrashDirectory {
    pub fn new(path: PathBuf, files_dir: PathBuf, info_dir: PathBuf) -> Self {
        Self {
            path,
            files_dir,
            info_dir,
            mount_point: None,
        }
    }

    pub fn with_mount_point(mut self, mount_point: PathBuf) -> Self {
        self.mount_point = Some(mount_point);
        self
    }
}

#[derive(Debug, Clone)]
pub struct TrashedItem {
    pub original_path: PathBuf,
    pub trashed_path: PathBuf,
    pub info_path: PathBuf,
    pub trash_dir: PathBuf,
    pub size_bytes: Option<u64>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl TrashedItem {
    pub fn new(original_path: PathBuf, trashed_path: PathBuf, info_path: PathBuf, trash_dir: PathBuf) -> Self {
        Self {
            original_path,
            trashed_path,
            info_path,
            trash_dir,
            size_bytes: None,
            deleted_at: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TrashCommand {
    pub kind: CommandKind,
    pub context: CommandContext,
    pub request_id: Option<String>,
}

#[derive(Debug, Clone)]
pub enum SkipReason {
    MissingPath(PathBuf),
    AlreadyTrashed(PathBuf),
    UnsupportedScheme(String),
    InternalError(String),
}

#[derive(Debug, Clone)]
pub enum CommandOutcome {
    Completed(CommandOutput),
    Skipped {
        command: CommandKind,
        path: PathBuf,
        reason: SkipReason,
    },
    Failed {
        command: CommandKind,
        reason: String,
    },
}

#[derive(Debug, Clone, Copy)]
pub enum ExitStatusLike {
    Ok,
    Warning,
    Error,
}

impl ExitStatusLike {
    pub fn as_code(self) -> u8 {
        match self {
            Self::Ok => 0,
            Self::Warning => 2,
            Self::Error => 1,
        }
    }
}
