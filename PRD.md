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
- The trashed file name is unique within the trash directory (appended numeric suffix if a
  collision exists).
- Both regular files and directories (including non-empty) are accepted without requiring a `-r`
  flag.
- Symlinks are trashed as symlinks; the link target is not followed.
- The command exits 0 when all arguments are successfully trashed.
- The command exits non-zero and prints a per-file error for any argument that cannot be trashed,
  but continues processing remaining arguments.

### E1.2: Generate `.trashinfo` Metadata Files

As a user, I want each trashed item to have a corresponding `.trashinfo` file recording the
original absolute path and deletion timestamp so that the item can be restored to its original
location.

**Acceptance Criteria:**
- A `.trashinfo` file is created in `<trash>/info/` for every successfully trashed item.
- The `.trashinfo` file conforms to the FreeDesktop.org Trash Specification format: section
  `[Trash Info]` with `Path=<percent-encoded absolute path>` and `DeletionDate=<ISO 8601 local>`.
- The `.trashinfo` file is written atomically (write to temp file then rename) to prevent
  partial metadata on crash.
- The `.trashinfo` filename matches the trashed file's name in `<trash>/files/` including any
  uniqueness suffix.

### E1.3: Cross-Device Moves via Per-Volume Trash Directories

As a user, I want files on mounted volumes to be trashed on the same volume using per-volume
trash directories so that trashing large files on external drives does not copy data across
devices.

**Acceptance Criteria:**
- When the target file resides on a volume other than the home volume, `trash-put` uses the
  per-volume trash directory at `<mount>/.Trash-<uid>/` or `<mount>/.Trash/<uid>/` according to
  the FreeDesktop specification.
- If no per-volume trash exists and cannot be created, the command falls back to the home trash
  (`$XDG_DATA_HOME/Trash` or `~/.local/share/Trash`) with a warning.
- Cross-device moves do not use a file copy followed by delete; the move is atomic when the source
  and target trash are on the same filesystem.

### E1.4: Permission and Special File Handling

As a user, I want `trash-put` to handle files I own on any filesystem (including read-only
directories) with informative errors so I always know why a file could not be trashed.

**Acceptance Criteria:**
- Attempting to trash a file in a directory where the user lacks write permission produces an
  error message naming the specific permission failure, not a generic exception traceback.
- Attempting to trash a non-existent path produces: `trash-put: cannot trash '<path>': No such file or directory`.
- Device files, sockets, and named pipes are accepted and moved like regular files.

---

## E2: Trash Inspection — `trash-list`

### E2.1: List All Trashed Items with Metadata

As a user, I want `trash-list` to show all trashed files across all trash directories I have
access to, with deletion date and original path, so I can find a file I need to restore.

**Acceptance Criteria:**
- `trash-list` scans all FreeDesktop-compliant trash directories accessible to the current user:
  `$XDG_DATA_HOME/Trash`, and per-volume directories on all mounted filesystems.
- Output format: `<YYYY-MM-DD HH:MM:SS> <original-path>` one item per line.
- Items are sorted by deletion date (oldest first) by default.
- If a `.trashinfo` file is malformed, the item is listed with the trashed filename and a
  `<unknown date>` placeholder; the command does not abort.

### E2.2: Filter Listed Items by Path Prefix

As a user, I want `trash-list /home/user/projects` to show only items whose original path
starts with the given prefix so I can find files trashed from a specific directory.

**Acceptance Criteria:**
- `trash-list <prefix>` filters output to items whose `Path` in `.trashinfo` starts with the
  given prefix (after percent-decoding).
- The filter is a path prefix, not a glob; partial directory name matches are not included.
- Zero matches produces no output and exits 0.

---

## E3: File Restoration — `trash-restore`

### E3.1: Interactive Restoration with Numbered Selection

As a user, I want `trash-restore` to show a numbered list of trashed items and let me choose
which to restore so I can recover specific files without knowing their exact trash paths.

**Acceptance Criteria:**
- `trash-restore` lists all trashed items with index, deletion date, and original path.
- The user enters a number (or comma-separated list / range) to select items to restore.
- Selected items are moved from trash `files/` back to their original path and their `.trashinfo`
  files are removed.
- Entering an out-of-range index or non-numeric input prints an error and re-prompts (or exits
  with a descriptive message).
- Pressing Ctrl-C or entering an empty selection exits without restoring.

### E3.2: Conflict Detection and Overwrite Control on Restore

As a user, I want `trash-restore` to detect when the original path already exists and either
prompt me or abort so I do not accidentally overwrite existing files.

**Acceptance Criteria:**
- When the original path is occupied, the default behaviour is to abort and print:
  `trash-restore: cannot restore '<path>': destination already exists`.
- `--overwrite` flag causes the occupying file to be replaced with the trashed version.
- The restore operation is atomic (rename, not copy-then-delete) when source and destination
  are on the same filesystem.

### E3.3: Restore a Specific Working Directory's Items

As a user, I want `trash-restore` run inside a directory to list and restore only items
originally from that directory tree, so I can recover a project without wading through
unrelated trash.

**Acceptance Criteria:**
- When run without arguments inside a directory, `trash-restore` filters the item list to items
  whose original path is under the current working directory.
- An explicit path argument overrides the CWD filter.

---

## E4: Trash Management — `trash-rm` and `trash-empty`

### E4.1: Remove Individual Files from Trash — `trash-rm`

As a user, I want `trash-rm <pattern>` to permanently delete files whose original name matches
a glob pattern so I can reclaim disk space from specific categories of trashed files without
emptying the entire trash.

**Acceptance Criteria:**
- `trash-rm <pattern>` matches the basename of each item's original path against the glob.
- Matched items' `files/` entries and corresponding `.trashinfo` files are permanently deleted.
- The command reports the count of permanently deleted items.
- Glob patterns containing shell-special characters work correctly (the user must quote them
  to protect from shell expansion, but the tool does not double-expand).
- `trash-rm` exits non-zero if no items match and `--strict` is passed.

### E4.2: Empty the Trash — `trash-empty`

As a user, I want `trash-empty` to permanently delete all trashed items so I can reclaim disk
space when I know I will not need to restore anything.

**Acceptance Criteria:**
- `trash-empty` (no arguments) permanently deletes all items in all accessible trash directories.
- `trash-empty <days>` permanently deletes only items deleted more than `<days>` days ago,
  computed from `DeletionDate` in `.trashinfo`.
- Orphaned files in `files/` without a corresponding `.trashinfo` are also removed during empty.
- Orphaned `.trashinfo` files without a corresponding `files/` entry are also removed.
- `trash-empty` asks for confirmation when `--trash-dir` is not specified and more than 100
  items would be permanently deleted, unless `--force` is passed.

---

## E5: Rust Core Migration (`trash-cli-core` crate)

### E5.1: Shared Rust Crate for Command Primitives

As a platform engineer, I want a `trash-cli-core` Rust crate providing shared types, a
testable `FileSystem` trait, and uniform `CommandOutcome` modelling so that future command
migrations to Rust share a common foundation with consistent error handling.

**Acceptance Criteria:**
- The `trash-cli-core` crate (at `src/` in this repo) compiles with `cargo build` without errors.
- `CommandKind` enum covers all five commands: `Put`, `List`, `Empty`, `Restore`, `Remove`.
- `TrashedItem` struct captures: original path, trashed path, `.trashinfo` path, trash directory,
  optional size in bytes, and optional deletion timestamp.
- `TrashDirectory` struct captures: trash root path, `files/` subdirectory, `info/` subdirectory,
  and optional mount point.
- `CommandOutcome` enum models `Completed`, `Skipped` (with `SkipReason`), and `Failed` variants.
- `FileSystem` trait abstracts all file I/O operations so command implementations can be unit
  tested without actual filesystem access.
- `RealFileSystem` implements `FileSystem` using `std::fs`.

### E5.2: Contract-Defined Python–Rust Interface

As a developer, I want the Python-to-Rust command bridge to be defined by JSON Schema contracts
so that both sides of the interface have machine-verifiable compatibility guarantees.

**Acceptance Criteria:**
- JSON Schema contracts for each command's request and response types live in `contracts/`.
- The Python test suite validates actual bridge inputs and outputs against these schemas using
  `jsonschema`.
- Any change to the Rust type definitions that would break the contract fails schema validation
  in CI before the code is merged.

---

## E6: Test Infrastructure and Compatibility

### E6.1: Comprehensive Command Test Suite

As a maintainer, I want a test suite covering all five commands with both unit tests and
integration tests against a real (ephemeral) filesystem so that regressions are caught before
release.

**Acceptance Criteria:**
- Tests exist for `trash-put`, `trash-list`, `trash-restore`, `trash-rm`, and `trash-empty`.
- Each command has at least: a happy-path test, a cross-device move test, a missing-file error
  test, and a malformed `.trashinfo` recovery test.
- Integration tests use a pytest fixture that creates an isolated temporary filesystem with
  simulated mount points.
- `cargo test` passes for the `trash-cli-core` crate on Linux and macOS.

### E6.2: FreeDesktop.org Specification Compliance

As a desktop environment user, I want trash-cli to be fully compatible with the trash
directories used by GNOME, KDE, and XFCE so that files trashed from the CLI are visible in
file managers and vice versa.

**Acceptance Criteria:**
- `trash-list` reads `.trashinfo` files written by GNOME Nautilus, KDE Dolphin, and Thunar
  without errors.
- `trash-put` produces `.trashinfo` files that GNOME Nautilus and KDE Dolphin can read and
  restore from the file manager UI.
- The trash directory structure follows the FreeDesktop spec: `<trash>/files/` and
  `<trash>/info/` subdirectories, `.trashinfo` extension, `[Trash Info]` section header.
- Path encoding in `.trashinfo` uses percent-encoding for non-ASCII characters per the spec.

---

## Non-Goals

- This tool does not implement a GUI or interactive TUI; it is a pure command-line tool.
- It does not implement secure file shredding (`shred`/`wipe`); `trash-empty` uses normal
  filesystem deletion. See `docs/about-shredding.txt` for rationale.
- It does not alias or replace the system `rm` command; see the FAQ in README.rst.
- It does not support Windows or macOS native trash APIs; it targets FreeDesktop.org
  (Linux / BSD) systems exclusively.
- The Rust migration is incremental; the Python implementation remains the primary runtime
  until each command's Rust implementation is feature-complete and tested.
