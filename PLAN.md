# trash-cli: Implementation Plan

## Phase 1: Core Python CLI (Complete)

| Task | Description | Depends On |
|------|-------------|------------|
| P1.1 | trash-put: move files to XDG trash with .trashinfo | - |
| P1.2 | trash-list: enumerate and display trashed files | - |
| P1.3 | trash-restore: restore files to original location | P1.1 |
| P1.4 | trash-empty: permanently empty trash (with age filter) | - |
| P1.5 | trash-rm: selectively remove items from trash | - |
| P1.6 | Cross-device trash handling via fstab parsing | P1.1 |

## Phase 2: Rust Core Migration (In Progress)

| Task | Description | Depends On |
|------|-------------|------------|
| P2.1 | Rust crate scaffolding (trash-cli-core) | - |
| P2.2 | Rust filesystem operations (models, helpers, errors) | P2.1 |
| P2.3 | JSON Schema contracts for Python-Rust bridge | P2.2 |
| P2.4 | rust-trash-rm binary for performance-critical deletion | P2.2 |

## Phase 3: Quality and Packaging

| Task | Description | Depends On |
|------|-------------|------------|
| P3.1 | Comprehensive test suite (Python + Rust) | P1.1, P2.2 |
| P3.2 | Shell completion generation (bash, zsh, fish) | P1.1 |
| P3.3 | Man page generation | P1.1 |
| P3.4 | PyPI and crates.io packaging | P3.1 |
