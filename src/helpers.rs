//! Shared utility helpers for migration-aware command behavior.

use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

/// File extension used by trash info files.
pub const TRASHINFO_EXTENSION: &str = ".trashinfo";

/// Deletion date format commonly used by Trash info metadata.
pub const TRASHINFO_TIME_FORMAT: &str = "%Y-%m-%dT%H:%M:%S";

/// Returns a user-safe, trimmed path string that can be used in logs and messages.
pub fn sanitize_user_path(path: &Path) -> String {
    path.display().to_string().trim().to_string()
}

/// Builds a deterministic, namespaced filename for trash files.
pub fn build_unique_basename(file_name: &str, suffix: u64) -> String {
    let base = Path::new(file_name)
        .file_name()
        .and_then(|v| v.to_str())
        .unwrap_or("item");
    format!("{base}.{suffix}")
}

/// Parses an ISO-like deletion date string into a UTC datetime.
pub fn parse_trash_datetime(value: &str) -> Option<DateTime<Utc>> {
    NaiveDateTime::parse_from_str(value, TRASHINFO_TIME_FORMAT)
        .ok()
        .map(|naive| Utc.from_utc_datetime(&naive))
        .or_else(|| DateTime::parse_from_rfc3339(value).ok().map(|dt| dt.with_timezone(&Utc)))
}

/// Serializes a UTC datetime into the repository-standard trash format.
pub fn serialize_system_time(time: SystemTime) -> String {
    let dt = DateTime::<Utc>::from(time);
    dt.format(TRASHINFO_TIME_FORMAT).to_string()
}

/// Human readable size rendering shared across commands.
pub fn print_size(bytes: u64) -> String {
    const SUFFIXES: [&str; 5] = ["B", "K", "M", "G", "T"];
    let mut value = bytes as f64;
    let mut idx = 0usize;

    while value >= 1024.0 && idx < SUFFIXES.len() - 1 {
        value /= 1024.0;
        idx += 1;
    }

    if idx == 0 {
        format!("{:.0} {}", value, SUFFIXES[idx])
    } else {
        format!("{:.1} {}", value, SUFFIXES[idx])
    }
}

/// Returns a normalized path by resolving `.` and `..` segments where possible.
pub fn canonical_or_relaxed(path: &Path) -> PathBuf {
    path.canonicalize().unwrap_or_else(|_| path.to_path_buf())
}

/// Produces a human readable timeout string from duration.
pub fn format_duration(duration: Duration) -> String {
    let secs = duration.as_secs();
    let mins = secs / 60;
    let hours = mins / 60;
    let days = hours / 24;
    let rem_secs = secs % 60;
    let rem_mins = mins % 60;
    let rem_hours = hours % 24;

    if days > 0 {
        format!("{days}d {rem_hours:02}:{rem_mins:02}:{rem_secs:02}")
    } else if hours > 0 {
        format!("{hours}h {rem_mins:02}:{rem_secs:02}")
    } else if mins > 0 {
        format!("{mins}m {rem_secs:02}s")
    } else {
        format!("{secs}s")
    }
}
