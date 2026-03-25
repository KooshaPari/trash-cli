# trash-cli: Code Entity Map

## Python CLI

| Entity | Path | Maps To |
|--------|------|---------|
| trash-put command | `trashcli/put/` | FR-PUT-001, FR-PUT-002, FR-PUT-003 |
| trash-list command | `trashcli/list/` | FR-LIST-001, FR-LIST-002 |
| trash-restore command | `trashcli/restore/` | FR-RESTORE-001, FR-RESTORE-002 |
| trash-rm command | `trashcli/rm/` | FR-RM-001, FR-RM-002 |
| trash-empty command | `trashcli/empty/` | FR-EMPTY-001, FR-EMPTY-002 |
| Filesystem abstraction | `trashcli/fs.py` | FR-PUT-004 |
| File system reader | `trashcli/file_system_reader.py` | FR-PUT-001, FR-LIST-001 |
| Trash dirs scanner | `trashcli/trash_dirs_scanner.py` | FR-COMPAT-001 |
| fstab parser | `trashcli/fstab/` | FR-COMPAT-002 |
| .trashinfo parser | `trashcli/parse_trashinfo/` | FR-PUT-003, FR-LIST-001 |
| Shell completion | `trashcli/shell_completion.py` | - |
| Compatibility layer | `trashcli/compat.py` | FR-COMPAT-001 |
| Error types | `trashcli/lib/` | All |

## Rust Core

| Entity | Path | Maps To |
|--------|------|---------|
| Rust crate root | `rust-trash-rm/src/lib.rs` | FR-RUST-001 |
| Rust binary | `rust-trash-rm/src/main.rs` | FR-RUST-001 |
| Filesystem ops | `rust-trash-rm/src/fs.rs` | FR-RUST-001 |
| Models | `rust-trash-rm/src/models.rs` | FR-RUST-001 |
| Error types | `rust-trash-rm/src/errors.rs` | FR-RUST-001 |

## Contracts

| Entity | Path | Maps To |
|--------|------|---------|
| JSON Schema contracts | `contracts/` | FR-RUST-002 |
| JSON Schema definitions | `schemas/` | FR-RUST-002 |
