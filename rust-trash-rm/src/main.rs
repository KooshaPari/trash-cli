use std::env;
use std::ffi::CString;
use std::collections::HashMap;
use std::fs::{self, read_dir, read_to_string};
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

type EnvVarMap = HashMap<String, String>;

fn main() {
    let mut stderr = io::stderr();
    let uid = unsafe { libc::geteuid() };
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        writeln!(
            stderr,
            "Usage:\n    trash-rm PATTERN\n\nPlease specify PATTERN.\ntrash-rm uses fnmatch.fnmatchcase to match patterns, see https://docs.python.org/3/library/fnmatch.html for more details."
        )
        .expect("unable to write usage to stderr");
        std::process::exit(8);
    }

    let pattern = args[1].as_str();
    let environ = env::vars().collect::<EnvVarMap>();
    let mut had_error = false;
    let stderr = io::stderr();
    let mut stderr = stderr.lock();
    for item in list_trashinfo_matches(collect_trash_dirs(&environ, uid as u32), pattern) {
        match item {
            Ok((_original_location, trashinfo_path, backup_path)) => {
                if let Err(err) = rm_file_if_exists(&backup_path) {
                    had_error = true;
                    writeln!(stderr, "trash-rm: failed to remove backup file {}: {}", backup_path.display(), err)
                        .expect("unable to write removal error to stderr");
                }
                if let Err(err) = rm_file2(&trashinfo_path) {
                    had_error = true;
                    writeln!(stderr, "trash-rm: failed to remove trashinfo {}: {}", trashinfo_path.display(), err)
                        .expect("unable to write removal error to stderr");
                }
            }
            Err(info_path) => {
                writeln!(
                    stderr,
                    "trash-rm: {}: unable to parse 'Path'",
                    info_path.display()
                )
                .expect("unable to write error to stderr");
            }
        }
    }

    if had_error {
        std::process::exit(1);
    }
}

fn collect_trash_dirs(environ: &EnvVarMap, uid: u32) -> Vec<(PathBuf, String)> {
    let xdg_data_home = environ.get("XDG_DATA_HOME").cloned();
    let home_dir = environ.get("HOME").cloned();
    let volumes_env = environ.get("TRASH_VOLUMES").cloned();

    let mut trash_dirs = Vec::new();

    if let Some(xdg) = xdg_data_home {
        let mut xdg_trash = PathBuf::from(xdg);
        xdg_trash.push("Trash");
        trash_dirs.push((xdg_trash, "/".to_string()));
    } else if let Some(home) = home_dir {
        let mut default_trash = PathBuf::from(home);
        default_trash.push(".local");
        default_trash.push("share");
        default_trash.push("Trash");
        trash_dirs.push((default_trash, "/".to_string()));
    }

    let volumes = if let Some(volumes) = volumes_env {
        if volumes.is_empty() {
            Vec::new()
        } else {
            volumes
                .split(':')
                .filter(|path| !path.is_empty())
                .map(PathBuf::from)
                .collect()
        }
    } else {
        list_mount_points()
    };

    for volume in volumes {
        let top_dir = volume.join(".Trash").join(uid.to_string());
        let parent = top_dir.parent().unwrap_or_else(|| Path::new("/")).to_path_buf();

        if top_dir.exists() && parent.is_dir() && is_sticky_dir(&parent) && !is_symlink(&parent) {
            trash_dirs.push((top_dir, volume.to_string_lossy().to_string()));
        }

        let alt_top_dir = volume.join(format!(".Trash-{uid}"));
        if alt_top_dir.is_dir() {
            trash_dirs.push((alt_top_dir, volume.to_string_lossy().to_string()));
        }
    }

    trash_dirs
}

fn list_mount_points() -> Vec<PathBuf> {
    read_to_string("/proc/self/mounts")
        .map(|content| {
            content
                .lines()
                .filter_map(|line| {
                    let fields: Vec<&str> = line.split_whitespace().collect();
                    (fields.len() >= 2).then(|| {
                        let mountpoint = unescape_mountpoint(fields[1]);
                        PathBuf::from(mountpoint)
                    })
                })
                .filter(|path| path.exists())
                .collect()
        })
        .unwrap_or_default()
}

fn unescape_mountpoint(value: &str) -> String {
    value
        .replace("\\011", "\t")
        .replace("\\012", "\n")
        .replace("\\040", " ")
        .replace("\\\\", "\\")
}

fn list_trashinfo_matches(
    trash_dirs: Vec<(PathBuf, String)>,
    pattern: &str,
) -> Vec<Result<(PathBuf, PathBuf, PathBuf), PathBuf>> {
    let mut matched = Vec::new();
    for (trash_dir, volume) in trash_dirs {
        let info_dir = trash_dir.join("info");
        let entries = match read_dir(info_dir) {
            Ok(entries) => entries,
            Err(_) => continue,
        };

        for entry in entries.flatten() {
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if !name.ends_with(".trashinfo") {
                continue;
            }

            let info_path = entry.path();
            match parse_trashinfo_path(&info_path) {
                Ok(original_location) => {
                    let complete_path = join_volume_and_path(&volume, &original_location);
                    if path_matches(pattern, &complete_path) {
                        let backup_path = backup_copy_for_trashinfo(&info_path);
                        matched.push(Ok((PathBuf::from(original_location), info_path, backup_path)));
                    }
                }
                Err(_) => matched.push(Err(info_path)),
            }
        }
    }
    matched
}

fn parse_trashinfo_path(path: &Path) -> Result<String, ()> {
    let content = read_to_string(path).map_err(|_| ())?;
    for line in content.lines() {
        if let Some(rest) = line.strip_prefix("Path=") {
            return urlencoding::decode(rest).map_err(|_| ()).map(|path| path.into_owned());
        }
    }
    Err(())
}

fn join_volume_and_path(volume: &str, original_location: &str) -> String {
    if original_location.starts_with('/') {
        original_location.to_string()
    } else if volume == "/" {
        format!("/{original_location}")
    } else if volume.ends_with('/') {
        format!("{volume}{original_location}")
    } else {
        format!("{volume}/{original_location}")
    }
}

fn path_matches(pattern: &str, original_location: &str) -> bool {
    let subject = if pattern.starts_with('/') {
        original_location.to_string()
    } else {
        Path::new(original_location)
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or_default()
            .to_string()
    };

    let pattern = CString::new(pattern).ok();
    let subject = CString::new(subject).ok();
    match (pattern, subject) {
        (Some(pattern), Some(subject)) => unsafe {
            libc::fnmatch(pattern.as_ptr(), subject.as_ptr(), 0) == 0
        },
        _ => false,
    }
}

fn backup_copy_for_trashinfo(trashinfo_path: &Path) -> PathBuf {
    let basename = trashinfo_path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or_default()
        .trim_end_matches(".trashinfo");
    trashinfo_path
        .parent()
        .and_then(|parent| parent.parent())
        .unwrap_or_else(|| Path::new(""))
        .join("files")
        .join(basename)
}

fn rm_file_if_exists(path: &Path) -> io::Result<()> {
    if fs::symlink_metadata(path).is_ok() {
        rm_file2(path)?;
    }
    Ok(())
}

fn rm_file2(path: &Path) -> io::Result<()> {
    match fs::remove_file(path) {
        Ok(()) => Ok(()),
        Err(err) => match fs::remove_dir_all(path) {
            Ok(()) => Ok(()),
            Err(_) => Err(err),
        },
    }
}

fn is_symlink(path: &Path) -> bool {
    fs::symlink_metadata(path)
        .ok()
        .is_some_and(|metadata| metadata.file_type().is_symlink())
}

fn is_sticky_dir(path: &Path) -> bool {
    fs::metadata(path)
        .ok()
        .map(|metadata| metadata.is_dir() && metadata.permissions().mode() & 0o1000 == 0o1000)
        .unwrap_or(false)
}
