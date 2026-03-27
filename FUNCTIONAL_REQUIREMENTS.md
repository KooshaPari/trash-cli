# Functional Requirements — trash-cli

FR IDs follow the pattern `FR-{CAT}-{NNN}`.

---

## FR-PUT — Safe Deletion (`trash-put`)

| ID | SHALL Statement | Traces To | Status |
|----|-----------------|-----------|--------|
| FR-PUT-001 | `trash-put <path>...` SHALL move each argument to the `files/` subdirectory of the appropriate trash directory, determined per the FreeDesktop.org Trash Specification | E1.1 | Implemented |
| FR-PUT-002 | `trash-put` SHALL accept directories as arguments and move them recursively without requiring a `-r` flag | E1.1 | Implemented |
| FR-PUT-003 | `trash-put` SHALL create a `.trashinfo` metadata file in `<trash>/info/` for every successfully trashed item containing `[Trash Info]`, `Path=<percent-encoded path>`, and `DeletionDate=<ISO 8601 local datetime>` | E1.2 | Implemented |
| FR-PUT-004 | `trash-put` SHALL write `.trashinfo` files atomically (write to temp file, then rename) so that a crash mid-write does not produce a partial metadata file | E1.2 | Implemented |
| FR-PUT-005 | `trash-put` SHALL resolve collisions in the trash `files/` directory by appending a numeric suffix to the trashed filename and SHALL use the same suffix for the corresponding `.trashinfo` file | E1.1 | Implemented |
| FR-PUT-006 | `trash-put` SHALL use a per-volume trash directory (`<mount>/.Trash-<uid>/` or `<mount>/.Trash/<uid>/`) when the target file resides on a filesystem different from the home volume | E1.3 | Implemented |
| FR-PUT-007 | `trash-put` SHALL perform a same-filesystem rename (not copy+delete) when moving to a per-volume trash on the same device as the source | E1.3 | Implemented |
| FR-PUT-008 | When the per-volume trash cannot be created or written, `trash-put` SHALL fall back to the home trash with a warning printed to stderr | E1.3 | Implemented |
| FR-PUT-009 | `trash-put` SHALL process all path arguments and SHALL NOT abort on the first failure; per-argument errors SHALL be printed to stderr, and the exit code SHALL be non-zero if any argument failed | E1.1 | Implemented |
| FR-PUT-010 | `trash-put` SHALL produce an error message `trash-put: cannot trash '<path>': No such file or directory` for non-existent paths, not a Python traceback | E1.4 | Implemented |
| FR-PUT-011 | `trash-put` SHALL accept device files, sockets, named pipes, and symlinks as valid arguments and SHALL move them to trash without following symlink targets | E1.4 | Implemented |

---

## FR-LIST — Trash Inspection (`trash-list`)

| ID | SHALL Statement | Traces To | Status |
|----|-----------------|-----------|--------|
| FR-LIST-001 | `trash-list` SHALL scan all FreeDesktop-compliant trash directories accessible to the current user: `$XDG_DATA_HOME/Trash` (defaulting to `~/.local/share/Trash`) and per-volume trash directories on all mounted filesystems | E2.1 | Implemented |
| FR-LIST-002 | `trash-list` SHALL output one line per trashed item in the format `<YYYY-MM-DD HH:MM:SS> <original-path>` | E2.1 | Implemented |
| FR-LIST-003 | `trash-list` SHALL sort items by deletion date, oldest first, by default | E2.1 | Implemented |
| FR-LIST-004 | `trash-list` SHALL parse malformed or missing `DeletionDate` fields in `.trashinfo` files without aborting; items with unparseable dates SHALL be listed with `<unknown date>` as the timestamp | E2.1 | Implemented |
| FR-LIST-005 | `trash-list <prefix>` SHALL filter output to items whose percent-decoded original path starts with `<prefix>` as a path prefix (not a glob pattern) | E2.2 | Implemented |
| FR-LIST-006 | `trash-list <prefix>` with no matching items SHALL produce no output and exit 0 | E2.2 | Implemented |

---

## FR-RESTORE — File Restoration (`trash-restore`)

| ID | SHALL Statement | Traces To | Status |
|----|-----------------|-----------|--------|
| FR-RESTORE-001 | `trash-restore` (no arguments) SHALL display a numbered list of all trashed items with index, deletion date, and original path | E3.1 | Implemented |
| FR-RESTORE-002 | `trash-restore` SHALL accept a numeric index, comma-separated list of indices, or a range (e.g., `0-2, 4`) as the restore selection | E3.1 | Implemented |
| FR-RESTORE-003 | `trash-restore` SHALL move selected items from `<trash>/files/` back to their original path and SHALL delete the corresponding `.trashinfo` file on successful restore | E3.1 | Implemented |
| FR-RESTORE-004 | `trash-restore` SHALL perform the restore as an atomic rename when source and destination are on the same filesystem | E3.2 | Implemented |
| FR-RESTORE-005 | When the original path of an item to be restored is already occupied by an existing file, `trash-restore` SHALL abort the restore of that item and print: `trash-restore: cannot restore '<path>': destination already exists` | E3.2 | Implemented |
| FR-RESTORE-006 | `trash-restore --overwrite` SHALL replace the occupying file at the original path with the trashed version | E3.2 | Implemented |
| FR-RESTORE-007 | When run inside a directory without arguments, `trash-restore` SHALL pre-filter the item list to items whose original path is under the current working directory | E3.3 | Implemented |
| FR-RESTORE-008 | Out-of-range or non-numeric selection input SHALL produce a descriptive error; the command SHALL NOT raise an unhandled exception on invalid input | E3.1 | Implemented |

---

## FR-RM — Selective Trash Removal (`trash-rm`)

| ID | SHALL Statement | Traces To | Status |
|----|-----------------|-----------|--------|
| FR-RM-001 | `trash-rm <pattern>` SHALL permanently delete all trashed items whose original path basename matches the given glob pattern | E4.1 | Implemented |
| FR-RM-002 | `trash-rm` SHALL delete both the `files/` entry and the corresponding `.trashinfo` file for each matched item | E4.1 | Implemented |
| FR-RM-003 | `trash-rm` SHALL print the count of permanently deleted items on completion | E4.1 | Implemented |
| FR-RM-004 | `trash-rm --strict` SHALL exit non-zero when no items match the given pattern | E4.1 | Implemented |
| FR-RM-005 | `trash-rm` SHALL match patterns against the basename of the original path (not the trashed name) using Python `fnmatch` semantics | E4.1 | Implemented |

---

## FR-EMPTY — Trash Emptying (`trash-empty`)

| ID | SHALL Statement | Traces To | Status |
|----|-----------------|-----------|--------|
| FR-EMPTY-001 | `trash-empty` (no arguments) SHALL permanently delete all items in all accessible trash directories | E4.2 | Implemented |
| FR-EMPTY-002 | `trash-empty <days>` SHALL permanently delete only items whose `DeletionDate` in `.trashinfo` is more than `<days>` days before the current time | E4.2 | Implemented |
| FR-EMPTY-003 | `trash-empty` SHALL remove orphaned files in `files/` that have no corresponding `.trashinfo` | E4.2 | Implemented |
| FR-EMPTY-004 | `trash-empty` SHALL remove orphaned `.trashinfo` files that have no corresponding `files/` entry | E4.2 | Implemented |
| FR-EMPTY-005 | `trash-empty` with `--force` SHALL skip any interactive confirmation prompt regardless of the number of items | E4.2 | Implemented |

---

## FR-CORE — Rust Core Crate (`src/`, `trash-cli-core`)

| ID | SHALL Statement | Traces To | Status |
|----|-----------------|-----------|--------|
| FR-CORE-001 | The `trash-cli-core` Rust crate SHALL compile without errors or warnings under `cargo build` on Linux | E5.1 | Implemented |
| FR-CORE-002 | `CommandKind` SHALL be an enum covering `Put`, `List`, `Empty`, `Restore`, and `Remove` variants with `as_str()` and `Display` implementations | E5.1 | Implemented |
| FR-CORE-003 | `TrashedItem` SHALL carry: `original_path: PathBuf`, `trashed_path: PathBuf`, `info_path: PathBuf`, `trash_dir: PathBuf`, `size_bytes: Option<u64>`, and `deleted_at: Option<DateTime<Utc>>` | E5.1 | Implemented |
| FR-CORE-004 | `TrashDirectory` SHALL carry: `path: PathBuf`, `files_dir: PathBuf`, `info_dir: PathBuf`, and `mount_point: Option<PathBuf>` | E5.1 | Implemented |
| FR-CORE-005 | `CommandOutcome` SHALL be an enum with variants `Completed(CommandOutput)`, `Skipped { command, path, reason: SkipReason }`, and `Failed { command, reason: String }` | E5.1 | Implemented |
| FR-CORE-006 | `FileSystem` trait SHALL abstract all filesystem I/O so that unit tests can inject a fake implementation without touching the real filesystem | E5.1 | Implemented |
| FR-CORE-007 | `RealFileSystem` SHALL implement `FileSystem` using `std::fs` functions | E5.1 | Implemented |
| FR-CORE-008 | The `prelude` module SHALL re-export all public stable types so command crates can import via `use trash_cli_core::prelude::*` | E5.1 | Implemented |

---

## FR-CONTRACT — Python-Rust Interface Contracts (`contracts/`)

| ID | SHALL Statement | Traces To | Status |
|----|-----------------|-----------|--------|
| FR-CONTRACT-001 | JSON Schema contracts defining the request and response types for each command's Python-Rust bridge SHALL reside in `contracts/` | E5.2 | Implemented |
| FR-CONTRACT-002 | The Python test suite SHALL validate actual bridge inputs and outputs against the JSON Schema contracts using `jsonschema` | E5.2 | Implemented |
| FR-CONTRACT-003 | CI SHALL run contract validation against the schemas on every pull request; a schema mismatch SHALL fail the build | E5.2 | Implemented |

---

## FR-COMPAT — Specification Compatibility

| ID | SHALL Statement | Traces To | Status |
|----|-----------------|-----------|--------|
| FR-COMPAT-001 | `trash-list` SHALL read `.trashinfo` files created by GNOME Nautilus, KDE Dolphin, and Thunar without errors or data loss | E6.2 | Implemented |
| FR-COMPAT-002 | `.trashinfo` files written by `trash-put` SHALL be readable and restorable by GNOME Nautilus and KDE Dolphin | E6.2 | Implemented |
| FR-COMPAT-003 | All trash directory paths SHALL follow the FreeDesktop.org Trash Specification: `<trash>/files/` and `<trash>/info/` subdirectories | E6.2 | Implemented |
| FR-COMPAT-004 | `Path` values in `.trashinfo` files SHALL use percent-encoding for all non-ASCII characters per the FreeDesktop.org Trash Specification | E6.2 | Implemented |
| FR-COMPAT-005 | `trash-list` and `trash-put` SHALL resolve trash directories by scanning `/proc/mounts` (Linux) or `mount` output for all accessible volume mount points | E1.3 | Implemented |
