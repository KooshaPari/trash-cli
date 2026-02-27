use chrono::NaiveDateTime;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fmt::{self, Display, Formatter};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use urlencoding::decode as url_decode;

#[cfg(unix)]
use std::os::unix::fs::MetadataExt;

const VERSION: &str = "0.24.5.26";
const UNKNOWN_DELETION_DATE: &str = "????-??-?? ??:??:??";
const BUG_REPORT_URL: &str = "https://github.com/andreafrancia/trash-cli/issues";

#[derive(Debug, Clone, Copy)]
enum Action {
    PrintVersion,
    DebugVolumes,
    ListVolumes,
    ListTrashDirs,
    ListTrash,
    PrintPythonExecutable,
}

#[derive(Debug, Clone, Copy)]
enum Attribute {
    DeletionDate,
    Size,
}

#[derive(Debug)]
struct ListConfig {
    action: Action,
    attribute_to_print: Attribute,
    show_files: bool,
    all_users: bool,
    trash_dirs: Vec<String>,
}

#[derive(Debug)]
enum Event {
    Found(TrashDir),
    SkipNotSticky(PathBuf),
    SkipSymlink(PathBuf),
}

#[derive(Debug)]
struct TrashDir {
    path: PathBuf,
    volume: String,
}

#[derive(Debug)]
enum ScanEvent {
    Found(TrashDir),
    SkippedNotSticky(PathBuf),
    SkippedSymlink(PathBuf),
    // Mirrors Python implementation behavior, kept for completeness.
    SkippedMissing,
}

#[derive(Debug)]
struct CliError(String);

impl Display for CliError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn print_help(prog: &str) {
    println!(
        "\
usage: {prog} [-h] [--print-completion {{bash,zsh,tcsh}}] [--version] \
[--volumes] [--trash-dirs] [--trash-dir TRASH_DIRS] [--all-users]

List trashed files

options:
  -h, --help            show this help message and exit
  --print-completion {{bash,zsh,tcsh}}
                        print shell completion script
  --version             show program's version number and exit
  --volumes             list volumes
  --trash-dirs          list trash dirs
  --trash-dir TRASH_DIRS
                        specify the trash directory to use
  --all-users           list trashcans of all the users

Report bugs to {url}
",
        prog = prog,
        url = BUG_REPORT_URL
    );
}

fn parse_args(args: &[String]) -> Result<ListConfig, CliError> {
    let mut config = ListConfig {
        action: Action::ListTrash,
        attribute_to_print: Attribute::DeletionDate,
        show_files: false,
        all_users: false,
        trash_dirs: Vec::new(),
    };

    let mut i = 0;
    while i < args.len() {
        let arg = &args[i];
        match arg.as_str() {
            "--help" | "-h" => {
                return Err(CliError("help".to_string()));
            }
            "--version" => config.action = Action::PrintVersion,
            "--debug-volumes" => config.action = Action::DebugVolumes,
            "--volumes" => config.action = Action::ListVolumes,
            "--trash-dirs" => config.action = Action::ListTrashDirs,
            "--size" => config.attribute_to_print = Attribute::Size,
            "--files" => config.show_files = true,
            "--all-users" => config.all_users = true,
            "--python" => config.action = Action::PrintPythonExecutable,
            "--print-completion" => {
                let shell = if i + 1 < args.len() {
                    i += 1;
                    args[i].as_str()
                } else {
                    return Err(CliError("missing completion shell".to_string()));
                };
                if shell == "bash" || shell == "zsh" || shell == "tcsh" {
                    println!("Please install shtab firstly!");
                    return Err(CliError("complete".to_string()));
                }
                return Err(CliError(format!("unsupported shell for completion: {}", shell)));
            }
            _ if arg.starts_with("--print-completion=") => {
                let shell = &arg["--print-completion=".len()..];
                if shell == "bash" || shell == "zsh" || shell == "tcsh" {
                    println!("Please install shtab firstly!");
                    return Err(CliError("complete".to_string()));
                }
                return Err(CliError(format!("unsupported shell for completion: {}", shell)));
            }
            _ if arg == "--trash-dir" => {
                if i + 1 >= args.len() {
                    return Err(CliError("missing value for --trash-dir".to_string()));
                }
                config.trash_dirs.push(args[i + 1].clone());
                i += 1;
            }
            _ if arg.starts_with("--trash-dir=") => {
                config.trash_dirs.push(arg["--trash-dir=".len()..].to_string());
            }
            _ if arg.starts_with('-') => {
                return Err(CliError(format!("unrecognized arguments: {}", arg)));
            }
            _ => {
                return Err(CliError(format!("unrecognized arguments: {}", arg)));
            }
        }
        i += 1;
    }

    Ok(config)
}

fn list_mount_points() -> Vec<String> {
    let content = match fs::read_to_string("/proc/mounts") {
        Ok(content) => content,
        Err(_) => return Vec::new(),
    };

    let mut points = Vec::new();
    for line in content.lines() {
        let mut fields = line.split_whitespace();
        let _device = fields.next();
        let mountpoint = match fields.next() {
            Some(mp) => mp,
            None => continue,
        };
        let mountpoint = unescape_mountpoint(mountpoint);
        if !points.contains(&mountpoint) {
            points.push(mountpoint);
        }
    }
    points
}

fn unescape_mountpoint(value: &str) -> String {
    let mut decoded = String::new();
    let mut chars = value.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '\\' && chars.peek().is_some() {
            let d1 = chars.next().unwrap();
            let d2 = chars.next().unwrap_or('0');
            let d3 = chars.next().unwrap_or('0');
            if let (Some(o1), Some(o2), Some(o3)) = (to_octal_digit(d1), to_octal_digit(d2), to_octal_digit(d3)) {
                let byte = (o1 << 6) | (o2 << 3) | o3;
                decoded.push(byte as char);
            } else {
                decoded.push('\\');
                decoded.push(d1);
                if d2 != '0' {
                    decoded.push(d2);
                }
                if d3 != '0' {
                    decoded.push(d3);
                }
            }
        } else {
            decoded.push(ch);
        }
    }
    decoded
}

fn to_octal_digit(ch: char) -> Option<u8> {
    match ch {
        '0'..='7' => Some(ch as u8 - b'0'),
        _ => None,
    }
}

fn list_volumes(environ: &HashMap<String, String>) -> Vec<String> {
    if let Some(volumes) = environ.get("TRASH_VOLUMES") {
        if !volumes.is_empty() {
            return volumes
                .split(':')
                .filter(|v| !v.is_empty())
                .map(str::to_string)
                .collect();
        }
    }
    list_mount_points()
}

fn list_trash_volumes(environ: &HashMap<String, String>) {
    for volume in list_volumes(environ) {
        println!("{}", volume);
    }
}

fn is_sticky_dir(path: &Path) -> bool {
    let metadata = match fs::metadata(path) {
        Ok(metadata) => metadata,
        Err(_) => return false,
    };
    if !metadata.is_dir() {
        return false;
    }
    #[cfg(unix)]
    {
        (metadata.mode() & 0o1000) == 0o1000
    }
    #[cfg(not(unix))]
    {
        false
    }
}

fn is_symlink(path: &Path) -> bool {
    match fs::symlink_metadata(path) {
        Ok(metadata) => metadata.file_type().is_symlink(),
        Err(_) => false,
    }
}

fn is_dir(path: &Path) -> bool {
    match fs::metadata(path) {
        Ok(metadata) => metadata.is_dir(),
        Err(_) => false,
    }
}

fn volume_of(path: &Path, mount_points: &HashSet<String>) -> String {
    let mut current = if path.is_absolute() {
        path.to_path_buf()
    } else {
        match env::current_dir() {
            Ok(cwd) => cwd.join(path),
            Err(_) => PathBuf::from(path),
        }
    };

    loop {
        let current_s = current.to_string_lossy().to_string();
        if mount_points.contains(&current_s) {
            return current_s;
        }
        match current.parent() {
            Some(parent) => {
                let parent_s = parent.to_string_lossy().to_string();
                if parent_s == current_s {
                    return current_s;
                }
                current = parent.to_path_buf();
            }
            None => return current_s,
        }
    }
}

fn current_uid() -> String {
    if let Ok(uid) = env::var("UID") {
        if !uid.is_empty() {
            return uid;
        }
    }
    "0".to_string()
}

fn home_trash_dirs_from_environ(environ: &HashMap<String, String>) -> Vec<String> {
    if let Some(xdg_data_home) = environ.get("XDG_DATA_HOME") {
        return vec![format!("{}/Trash", xdg_data_home)];
    }
    if let Some(home) = environ.get("HOME") {
        return vec![format!("{}/.local/share/Trash", home)];
    }
    Vec::new()
}

fn list_users() -> Vec<(String, String)> {
    let passwd = match fs::read_to_string("/etc/passwd") {
        Ok(content) => content,
        Err(_) => return Vec::new(),
    };
    let mut users = Vec::new();
    for line in passwd.lines() {
        let fields: Vec<&str> = line.split(':').collect();
        if fields.len() < 7 {
            continue;
        }
        let uid = fields[2];
        let home = fields[5];
        users.push((uid.to_string(), home.to_string()));
    }
    users
}

fn scan_trash_dirs_for_current_user(
    environ: &HashMap<String, String>,
    volumes: &[String],
    mount_points: &HashSet<String>,
) -> Vec<Event> {
    let mut events = Vec::new();

    for path in home_trash_dirs_from_environ(environ) {
        let path = PathBuf::from(path);
        let uid = current_uid();
        events.push(Event::Found(TrashDir {
            path,
            volume: volume_of(&path, mount_points),
        }));
    }

    for volume in volumes {
        scan_top_trash_dir(volume, &current_uid(), mount_points, &mut events);
    }
    events
}

fn scan_top_trash_dir(
    volume: &str,
    uid: &str,
    mount_points: &HashSet<String>,
    out: &mut Vec<Event>,
) {
    let top = Path::new(volume).join(".Trash").join(uid);
    match validate_top_trash_dir(&top) {
        ScanEvent::Found(_) => out.push(Event::Found(TrashDir {
            path: top,
            volume: volume.to_string(),
        })),
        ScanEvent::SkippedNotSticky(path) => out.push(Event::SkipNotSticky(path)),
        ScanEvent::SkippedSymlink(path) => out.push(Event::SkipSymlink(path)),
        ScanEvent::SkippedMissing => {}
    }

    let alt = Path::new(volume).join(format!(".Trash-{}", uid));
    if is_dir(&alt) {
        out.push(Event::Found(TrashDir {
            path: alt,
            volume: volume.to_string(),
        }));
    }
}

fn validate_top_trash_dir(path: &Path) -> ScanEvent {
    if !path.exists() {
        return ScanEvent::SkippedMissing;
    }
    let parent = match path.parent() {
        Some(p) => p,
        None => return ScanEvent::SkippedMissing,
    };
    if !is_sticky_dir(parent) {
        return ScanEvent::SkippedNotSticky(path.to_path_buf());
    }
    if is_symlink(parent) {
        return ScanEvent::SkippedSymlink(path.to_path_buf());
    }
    ScanEvent::Found(TrashDir {
        path: path.to_path_buf(),
        volume: String::new(),
    })
}

fn scan_all_users_volumes(
    volumes: &[String],
    mount_points: &HashSet<String>,
) -> Vec<Event> {
    let mut events = Vec::new();
    for (_uid, home) in list_users() {
        let _ = _uid;
        let path = Path::new(&home).join(".local/share/Trash");
        events.push(Event::Found(TrashDir {
            path,
            volume: volume_of(&PathBuf::from(home).join(".local/share/Trash"), mount_points),
        }));
        for volume in volumes {
            scan_top_trash_dir(volume, &_uid, mount_points, &mut events);
        }
    }
    events
}

fn select_trash_dirs(
    all_users: bool,
    user_specified_dirs: &[String],
    environ: &HashMap<String, String>,
    mount_points: &HashSet<String>,
) -> Vec<Event> {
    let volumes = list_volumes(environ);
    let mut events = Vec::new();

    if all_users {
        events.extend(scan_all_users_volumes(&volumes, mount_points));
        return events;
    }

    if user_specified_dirs.is_empty() {
        events.extend(scan_trash_dirs_for_current_user(environ, &volumes, mount_points));
    } else {
        for item in user_specified_dirs {
            events.push(Event::Found(TrashDir {
                path: PathBuf::from(item),
                volume: volume_of(Path::new(item), mount_points),
            }));
        }
    }

    events
}

fn file_size(path: &Path) -> Result<u64, String> {
    match fs::metadata(path) {
        Ok(stat) => Ok(stat.len()),
        Err(err) => {
            let is_symlink = match fs::symlink_metadata(path) {
                Ok(meta) => meta.file_type().is_symlink(),
                Err(_) => false,
            };
            if is_symlink {
                return Ok(0);
            }
            Err(err.to_string())
        }
    }
}

fn backup_copy_path(trashinfo_path: &Path) -> PathBuf {
    let trash_dir = trashinfo_path.parent().and_then(|p| p.parent()).unwrap_or_else(|| Path::new(""));
    let basename = trashinfo_path.file_name().and_then(|v| v.to_str()).unwrap_or("");
    let backup_name = basename.strip_suffix(".trashinfo").unwrap_or(basename);
    trash_dir.join("files").join(backup_name)
}

fn extract_deletion_date(contents: &str) -> String {
    let mut date = UNKNOWN_DELETION_DATE.to_string();
    for line in contents.lines() {
        if line.starts_with("DeletionDate=") && date == UNKNOWN_DELETION_DATE {
            let value = &line["DeletionDate=".len()..];
            let parsed = NaiveDateTime::parse_from_str(value, "%Y-%m-%dT%H:%M:%S");
            match parsed {
                Ok(value) => date = value.format("%Y-%m-%d %H:%M:%S").to_string(),
                Err(_) => date = UNKNOWN_DELETION_DATE.to_string(),
            }
            break;
        }
    }
    date
}

fn parse_path(contents: &str) -> Result<String, String> {
    for line in contents.lines() {
        if line.starts_with("Path=") {
            let encoded = &line["Path=".len()..];
            return url_decode(encoded)
                .map(|decoded| decoded.into_owned())
                .map_err(|error| error.to_string());
        }
    }
    Err("Unable to parse Path.".to_string())
}

fn extract_attribute(path: &Path, contents: &str, attribute: Attribute) -> Result<String, String> {
    match attribute {
        Attribute::DeletionDate => Ok(extract_deletion_date(contents)),
        Attribute::Size => {
            let backup_copy = backup_copy_path(path);
            let size = file_size(&backup_copy)?;
            Ok(size.to_string())
        }
    }
}

fn format_line(attr: &str, original_location: &str) -> String {
    format!("{} {}", attr, original_location)
}

fn format_line2(attr: &str, original_location: &str, original_file: &Path) -> String {
    format!("{} {} -> {}", attr, original_location, original_file.display())
}

fn compose_original_location(volume: &str, relative_location: &str) -> String {
    if Path::new(relative_location).is_absolute() {
        return relative_location.to_string();
    }
    let volume = volume.trim_end_matches('/');
    if volume.is_empty() || volume == "/" {
        format!("/{}", relative_location.trim_start_matches('/'))
    } else {
        format!("{}/{}", volume, relative_location.trim_start_matches('/'))
    }
}

fn list_trash_for_dir(
    trash_dir: &TrashDir,
    config: &ListConfig,
) -> Result<(), String> {
    let info_dir = trash_dir.path.join("info");
    let entries = match fs::read_dir(&info_dir) {
        Ok(entries) => entries,
        Err(_) => return Ok(()),
    };

    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        let is_trashinfo = path
            .extension()
            .and_then(|ext| ext.to_str())
            .map_or(false, |ext| ext == "trashinfo");
        if !is_trashinfo {
            continue;
        }

        match fs::read_to_string(&path) {
            Ok(contents) => {
                let relative = match parse_path(&contents) {
                    Ok(value) => value,
                    Err(_) => {
                        eprintln!("Parse Error: {}: Unable to parse Path.", path.display());
                        continue;
                    }
                };
                let attribute = match extract_attribute(&path, &contents, config.attribute_to_print) {
                    Ok(value) => value,
                    Err(error) => return Err(error),
                };
                let original = compose_original_location(&trash_dir.volume, &relative);
                if config.show_files {
                    let original_file = backup_copy_path(&path);
                    println!("{}", format_line2(&attribute, &original, &original_file));
                } else {
                    println!("{}", format_line(&attribute, &original));
                }
            }
            Err(err) => {
                eprintln!("{}", err);
            }
        }
    }
    Ok(())
}

fn list_trash_dirs(config: &ListConfig, environ: &HashMap<String, String>, mount_points: &HashSet<String>) {
    let events = select_trash_dirs(
        config.all_users,
        &config.trash_dirs,
        environ,
        mount_points,
    );
    for event in events {
        match event {
            Event::Found(trash_dir) => println!("{}", trash_dir.path.display()),
            Event::SkipNotSticky(path) => println!("parent_not_sticky: {}", path.display()),
            Event::SkipSymlink(path) => println!("parent_is_symlink: {}", path.display()),
        }
    }
}

fn list_trash(config: &ListConfig, environ: &HashMap<String, String>, mount_points: &HashSet<String>) {
    let events = select_trash_dirs(
        config.all_users,
        &config.trash_dirs,
        environ,
        mount_points,
    );

    for event in events {
        match event {
            Event::Found(trash_dir) => {
                if let Err(err) = list_trash_for_dir(&trash_dir, config) {
                    eprintln!("{}", err);
                }
            }
            Event::SkipNotSticky(path) => eprintln!(
                "TrashDir skipped because parent not sticky: {}",
                path.display()
            ),
            Event::SkipSymlink(path) => eprintln!(
                "TrashDir skipped because parent is symlink: {}",
                path.display()
            ),
        }
    }
}

fn debug_volumes() {
    let mounts = list_mount_points();
    let mut physical = mounts.clone();
    physical.sort();
    let virtual = Vec::<String>::new();
    println!("physical ->");
    println!("{:#?}", physical);
    println!("virtual ->");
    println!("{:#?}", virtual);
    if let Ok(output) = Command::new("df").args(["-P"]).output() {
        print!("{}", String::from_utf8_lossy(&output.stdout));
    }
}

fn print_version() {
    println!("trash-list {}", VERSION);
}

fn print_python_executable() {
    println!(
        "{}",
        env::var("TRASH_LIST_PYTHON_EXECUTABLE").unwrap_or_else(|_| "python".to_string())
    );
}

fn run() -> i32 {
    let args: Vec<String> = env::args().skip(1).collect();
    let program = env::args().next().unwrap_or_else(|| "trash-list".to_string());
    let environ: HashMap<String, String> = env::vars().collect();

    let config = match parse_args(&args) {
        Ok(config) => config,
        Err(error) if error.0 == "help" => {
            print_help(&program);
            return 0;
        }
        Err(error) if error.0 == "complete" => return 0,
        Err(error) => {
            eprintln!("trash-list: {}", error);
            return 2;
        }
    };

    let mount_points: HashSet<String> = list_mount_points().into_iter().collect();

    match config.action {
        Action::PrintVersion => {
            print_version();
        }
        Action::DebugVolumes => {
            debug_volumes();
        }
        Action::ListVolumes => {
            list_trash_volumes(&environ);
        }
        Action::ListTrashDirs => {
            list_trash_dirs(&config, &environ, &mount_points);
        }
        Action::ListTrash => {
            list_trash(&config, &environ, &mount_points);
        }
        Action::PrintPythonExecutable => {
            print_python_executable();
        }
    }
    0
}

fn main() {
    std::process::exit(run());
}
