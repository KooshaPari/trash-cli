# Product Requirements Document — trash-cli

**Version:** 1.0.0
**Stack:** Python 3.10+ (primary), Rust 2021 edition (core migration), uv/pip packaging
**Repo:** `KooshaPari/trash-cli`
**Spec Compliance:** FreeDesktop.org Trash Specification 1.0
**Primary commands:** `trash-put`, `trash-list`, `trash-restore`, `trash-rm`, `trash-empty`

---

## Overview

`trash-cli` is a command-line interface to the FreeDesktop.org Trash specification. Instead of
permanently deleting files, it moves files to the system trash directory used by desktop
environments (GNOME, KDE, XFCE), recording the original path, deletion date, and permissions in
`.trashinfo` metadata files. This makes accidental deletions recoverable without leaving files
permanently orphaned in a non-standard location.

The Phenotype fork adds a Rust core crate (`trash-cli-core`) that provides shared type-safe
primitives, a `FileSystem` trait for testable filesystem operations, and uniform `CommandOutcome`
modelling for all five commands. The Rust foundation enables command-by-command migration away
from the legacy Python implementation toward a high-performance, memory-safe core while
maintaining full backward compatibility with the existing CLI interface.

---

## E1: Safe File Deletion — `trash-put`

### E1.1: Move Files and Directories to Trash

As a user, I want `trash-put <path>...` to move one or more files or directories to the system
trash so that they are recoverable instead of permanently deleted.

**Acceptance Criteria:**
- `trash-put foo bar/` moves each argument to the appropriate trash `files/` directory.
- The trashed file name is unique within the trash directory (appended numeric suffix on collision).
- Both regular files and directories are accepted without requiring a `-r` flag.
- Symlinks are trashed as symlinks; the link target is not followed.
- The command exits non-zero and prints a per-file error for any argument that cannot be trashed
  but continues processing remaining arguments.

### E1.2: Generate `.trashinfo` Metadata Files

As a user, I want each trashed item to have a `.trashinfo` file recording the original absolute
path and deletion timestamp so that the item can be restored to its original location.

**Acceptance Criteria:**
- A `.trashinfo` file is created in `<trash>/info/` for every successfully trashed item.
- The file conforms to the FreeDesktop.org Trash Specification: `[Trash Info]` section with
  `Path=<percent-encoded absolute path>` and `DeletionDate=<ISO 8601 local>`.
- The `.trashinfo` file is written atomically (temp file then rename) to prevent partial metadata.
- The `.trashinfo` filename matches the trashed file's name in `<trash>/files/` including any
  uniqueness suffix.

### E1.3: Cross-Device Moves via Per-Volume Trash Directories

As a user, I want files on mounted volumes to be trashed on the same volume using per-volume
trash directories so that trashing large files on external drives does not copy data across devices.

**Acceptance Criteria:**
- When the target file resides on a volume other than the home volume, `trash-put` uses the
  per-volume trash at `<mount>/.Trash-<uid>/` or `<mount>/.Trash/<uid>/`.
- If no per-volume trash exists and cannot be created, the command falls back to the home trash
  with a warning.
- Cross-device moves do not use copy+delete; the move is atomic when source and target are on
  the same filesystem.

### E1.4: Permission and Special File Handling

As a user, I want `trash-put` to handle files with informative errors so I always know why a
file could not be trashed.

**Acceptance Criteria:**
- Permission failure produces a descriptive error message naming the specific failure, not a
  Python traceback.
- Non-existent path produces: `trash-put: cannot trash '<path>': No such file or directory`.
- Device files, sockets, and named pipes are accepted and moved like regular files.

---

## E2: Trash Inspection — `trash-list`

### E2.1: List All Trashed Items with Metadata

As a user, I want `trash-list` to show all trashed files across all accessible trash directories
with deletion date and original path so I can find a file I need to restore.

**Acceptance Criteria:**
- `trash-list` scans all FreeDesktop-compliant trash directories: `$XDG_DATA_HOME/Trash` and
  per-volume directories on all mounted filesystems.
- Output format: `<YYYY-MM-DD HH:MM:SS> <original-path>` one item per line.
- Items are sorted by deletion date (oldest first) by default.
- Malformed `.trashinfo` files are listed with `<unknown date>` placeholder without aborting.

### E2.2: Filter Listed Items by Path Prefix

As a user, I want `trash-list /home/user/projects` to show only items from a specific directory
subtree so I can find files from a known location.

**Acceptance Criteria:**
- `trash-list <prefix>` filters output to items whose decoded `Path` starts with `<prefix>`.
- The filter is a path prefix, not a glob.
- Zero matches produces no output and exits 0.

---

## E3: File Restoration — `trash-restore`

### E3.1: Interactive Restoration with Numbered Selection

As a user, I want `trash-restore` to show a numbered list of trashed items and let me choose
which to restore so I can recover specific files interactively.

**Acceptance Criteria:**
- `trash-restore` lists all trashed items with index, deletion date, and original path.
- The user enters a number, comma-separated list, or range (e.g., `0-2, 4`) to select items.
- Selected items are moved from trash `files/` back to their original path and their `.trashinfo`
  files are removed.
- Out-of-range or non-numeric input prints a descriptive error without raising an unhandled exception.

### E3.2: Conflict Detection and Overwrite Control on Restore

As a user, I want `trash-restore` to detect when the original path already exists and abort so
I do not accidentally overwrite existing files.

**Acceptance Criteria:**
- When the original path is occupied, the default behaviour is to abort with:
  `trash-restore: cannot restore '<path>': destination already exists`.
- `--overwrite` flag causes the occupying file to be replaced with the trashed version.
- The restore is atomic (rename, not copy-then-delete) when source and destination are on the
  same filesystem.

### E3.3: Restore a Specific Working Directory's Items

As a user, I want `trash-restore` run inside a directory to list only items originally from
that directory tree so I can recover a project without wading through unrelated trash.

**Acceptance Criteria:**
- When run without arguments, `trash-restore` filters items to those whose original path is
  under the current working directory.
- An explicit path argument overrides the CWD filter.

---

## E4: Trash Management — `trash-rm` and `trash-empty`

### E4.1: Remove Individual Files from Trash — `trash-rm`

As a user, I want `trash-rm <pattern>` to permanently delete files matching a glob pattern from
trash so I can reclaim disk space selectively.

**Acceptance Criteria:**
- `trash-rm <pattern>` matches the basename of each item's original path against the glob.
- Matched items' `files/` entries and corresponding `.trashinfo` files are permanently deleted.
- The command reports the count of permanently deleted items.
- `trash-rm --strict` exits non-zero if no items match.

### E4.2: Empty the Trash — `trash-empty`

As a user, I want `trash-empty` to permanently delete all trashed items so I can reclaim disk
space when I know I will not need to restore anything.

**Acceptance Criteria:**
- `trash-empty` (no arguments) permanently deletes all items in all accessible trash directories.
- `trash-empty <days>` permanently deletes only items deleted more than `<days>` days ago.
- Orphaned files in `files/` without a corresponding `.trashinfo` are also removed.
- Orphaned `.trashinfo` files without a corresponding `files/` entry are also removed.
- `trash-empty --force` skips any interactive confirmation prompt.

---

## E5: Rust Core Migration (`trash-cli-core` crate)

### E5.1: Shared Rust Crate for Command Primitives

As a platform engineer, I want a `trash-cli-core` Rust crate providing shared types, a
testable `FileSystem` trait, and uniform `CommandOutcome` modelling so that future command
migrations to Rust share a common foundation.

**Acceptance Criteria:**
- The crate compiles with `cargo build` without errors on Linux.
- `CommandKind` enum covers all five commands: `Put`, `List`, `Empty`, `Restore`, `Remove`.
- `TrashedItem` captures original path, trashed path, `.trashinfo` path, trash directory, optional
  size in bytes, and optional deletion timestamp.
- `TrashDirectory` captures trash root path, `files/` directory, `info/` directory, and optional
  mount point.
- `CommandOutcome` models `Completed`, `Skipped` (with `SkipReason`), and `Failed` variants.
- `FileSystem` trait abstracts all file I/O so command implementations can be unit tested without
  accessing the real filesystem.
- `RealFileSystem` implements `FileSystem` using `std::fs`.

### E5.2: Contract-Defined Python-Rust Interface

As a developer, I want the Python-to-Rust bridge to be defined by JSON Schema contracts so
that both sides of the interface have machine-verifiable compatibility guarantees.

**Acceptance Criteria:**
- JSON Schema contracts for each command's request and response types live in `contracts/`.
- The Python test suite validates actual bridge inputs and outputs against these schemas.
- Schema mismatch fails CI before code is merged.

---

## E6: Test Infrastructure and Compatibility

### E6.1: Comprehensive Command Test Suite

As a maintainer, I want tests for all five commands covering happy path, error cases, and
cross-device scenarios so that regressions are caught before release.

**Acceptance Criteria:**
- Tests exist for all five commands: happy path, cross-device moves, missing-file errors, and
  malformed `.trashinfo` recovery.
- Integration tests use a pytest fixture with an isolated temporary filesystem.
- `cargo test` passes for the `trash-cli-core` crate.

### E6.2: FreeDesktop.org Specification Compliance

As a desktop environment user, I want trash-cli to be fully compatible with GNOME, KDE, and
XFCE trash directories so files trashed from the CLI are visible in file managers.

**Acceptance Criteria:**
- `trash-list` reads `.trashinfo` files written by GNOME Nautilus and KDE Dolphin without errors.
- `trash-put` produces `.trashinfo` files that GNOME Nautilus and KDE Dolphin can restore.
- All trash paths follow the FreeDesktop spec: `<trash>/files/` and `<trash>/info/`.
- Path encoding uses percent-encoding for non-ASCII characters per the spec.

---

## Non-Goals

- No GUI or interactive TUI; pure command-line tool.
- No secure file shredding (see `docs/about-shredding.txt`).
- Does not alias or replace `rm` (see README.rst FAQ).
- Does not support Windows or macOS native trash APIs; targets FreeDesktop.org systems only.
- The Rust migration is incremental; Python remains the primary runtime until each command's
  Rust implementation is feature-complete and tested.
