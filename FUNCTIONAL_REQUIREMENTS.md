# trash-cli: Functional Requirements

## FR-PUT: Safe Deletion

| ID | Requirement | Traces To |
|----|------------|-----------|
| FR-PUT-001 | System SHALL move files to XDG trash directory with .trashinfo metadata | E1.1 |
| FR-PUT-002 | System SHALL handle cross-device moves using per-volume trash dirs | E1.2 |
| FR-PUT-003 | System SHALL record original absolute path and deletion timestamp in .trashinfo | E1.3 |
| FR-PUT-004 | System SHALL handle symlinks, directories, and special files | E1.1 |

## FR-LIST: Trash Inspection

| ID | Requirement | Traces To |
|----|------------|-----------|
| FR-LIST-001 | System SHALL list all trashed files with deletion date and original path | E2.1 |
| FR-LIST-002 | System SHALL support filtering listed items by path prefix | E2.2 |

## FR-RESTORE: File Restoration

| ID | Requirement | Traces To |
|----|------------|-----------|
| FR-RESTORE-001 | System SHALL restore files to their original location | E3.1 |
| FR-RESTORE-002 | System SHALL detect and handle conflicts when original path exists | E3.2 |

## FR-RM: Selective Trash Removal

| ID | Requirement | Traces To |
|----|------------|-----------|
| FR-RM-001 | System SHALL permanently delete specified items from trash | E4.1 |
| FR-RM-002 | System SHALL support pattern matching for batch removal | E4.1 |

## FR-EMPTY: Trash Emptying

| ID | Requirement | Traces To |
|----|------------|-----------|
| FR-EMPTY-001 | System SHALL permanently delete all items in trash | E4.2 |
| FR-EMPTY-002 | System SHALL support age-based filtering (delete items older than N days) | E4.2 |

## FR-RUST: Rust Core

| ID | Requirement | Traces To |
|----|------------|-----------|
| FR-RUST-001 | Rust crate SHALL implement filesystem operations for trash-rm | E5.1 |
| FR-RUST-002 | Python-Rust interface SHALL validate against JSON Schema contracts | E5.2 |

## FR-COMPAT: Compatibility

| ID | Requirement | Traces To |
|----|------------|-----------|
| FR-COMPAT-001 | System SHALL scan all FreeDesktop-compliant trash directories (home + volumes) | E2.1 |
| FR-COMPAT-002 | System SHALL parse /etc/fstab for volume mount points | E1.2 |
