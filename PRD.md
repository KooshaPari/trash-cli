# trash-cli: Product Requirements Document

**Version:** 1.0 | **Status:** Draft | **Date:** 2026-03-25

## Product Vision

trash-cli is a safe file deletion CLI that moves files to the system trash (FreeDesktop.org Trash specification) instead of permanently deleting them. It provides `trash-put`, `trash-list`, `trash-restore`, `trash-rm`, and `trash-empty` commands.

## Epics

### E1: Safe Deletion (trash-put)

| ID | Story | Acceptance Criteria |
|----|-------|-------------------|
| E1.1 | Move files/directories to trash | Files are moved to XDG trash dir with .trashinfo metadata |
| E1.2 | Handle cross-device moves | Files on different filesystems use per-volume trash dirs |
| E1.3 | Preserve original path metadata | .trashinfo records original absolute path and deletion date |

### E2: Trash Inspection (trash-list)

| ID | Story | Acceptance Criteria |
|----|-------|-------------------|
| E2.1 | List trashed files with metadata | Show deletion date, original path, size |
| E2.2 | Filter by path pattern | `trash-list /home/user/docs` shows only matching items |

### E3: Restoration (trash-restore)

| ID | Story | Acceptance Criteria |
|----|-------|-------------------|
| E3.1 | Restore files to original location | File returns to original path; .trashinfo is removed |
| E3.2 | Handle conflicts on restore | Prompt or error if original path is occupied |

### E4: Trash Management

| ID | Story | Acceptance Criteria |
|----|-------|-------------------|
| E4.1 | Remove specific items from trash (trash-rm) | Permanently delete selected trashed items |
| E4.2 | Empty entire trash (trash-empty) | Permanently delete all trashed items, optionally by age |

### E5: Rust Core Migration

| ID | Story | Acceptance Criteria |
|----|-------|-------------------|
| E5.1 | Shared Rust foundation for performance-critical paths | Rust crate handles filesystem operations |
| E5.2 | Contract-based Python-Rust bridge | JSON Schema contracts define the interface |
