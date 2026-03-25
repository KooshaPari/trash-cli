# trash-cli: FR Implementation Tracker

| FR ID | Description | Status | Code Location |
|-------|-------------|--------|---------------|
| FR-PUT-001 | Move files to XDG trash with .trashinfo | Implemented | `trashcli/put/` |
| FR-PUT-002 | Cross-device trash handling | Implemented | `trashcli/fstab/`, `trashcli/trash_dirs_scanner.py` |
| FR-PUT-003 | Record original path and timestamp | Implemented | `trashcli/put/` |
| FR-PUT-004 | Handle symlinks, dirs, special files | Implemented | `trashcli/fs.py` |
| FR-LIST-001 | List trashed files | Implemented | `trashcli/list/` |
| FR-LIST-002 | Filter by path prefix | Implemented | `trashcli/list/` |
| FR-RESTORE-001 | Restore to original location | Implemented | `trashcli/restore/` |
| FR-RESTORE-002 | Conflict detection on restore | Implemented | `trashcli/restore/` |
| FR-RM-001 | Permanently delete from trash | Implemented | `trashcli/rm/` |
| FR-RM-002 | Pattern matching for batch removal | Implemented | `trashcli/rm/` |
| FR-EMPTY-001 | Empty all trash | Implemented | `trashcli/empty/` |
| FR-EMPTY-002 | Age-based filtering | Implemented | `trashcli/empty/` |
| FR-RUST-001 | Rust filesystem operations | In Progress | `rust-trash-rm/src/` |
| FR-RUST-002 | JSON Schema contracts | In Progress | `contracts/` |
| FR-COMPAT-001 | Scan all FreeDesktop trash dirs | Implemented | `trashcli/trash_dirs_scanner.py` |
| FR-COMPAT-002 | Parse /etc/fstab | Implemented | `trashcli/fstab/` |
