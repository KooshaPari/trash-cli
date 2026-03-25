# trash-cli: Architecture Decision Records

## ADR-001: FreeDesktop.org Trash Specification Compliance

- **Status:** Accepted
- **Context:** Need a standard trash mechanism that works across Linux desktop environments.
- **Decision:** Implement the FreeDesktop.org Trash specification (v1.0).
- **Rationale:** Industry standard; interoperable with GNOME, KDE, XFCE file managers.

## ADR-002: Python CLI with Rust Core

- **Status:** Accepted
- **Context:** Original codebase is pure Python; some filesystem operations benefit from Rust performance.
- **Decision:** Keep Python CLI layer; add `rust-trash-rm` Rust crate for performance-critical deletion.
- **Rationale:** Gradual migration path; Python handles CLI/UX, Rust handles heavy I/O.

## ADR-003: Per-Volume Trash Directories

- **Status:** Accepted
- **Context:** Moving files across filesystem boundaries is expensive (copy + delete vs. rename).
- **Decision:** Use `$MOUNT/.Trash-$UID/` for files on non-home volumes per FreeDesktop spec.
- **Rationale:** Avoids slow cross-device copies; keeps trash operations O(1) via rename.

## ADR-004: JSON Schema Contracts for Python-Rust Bridge

- **Status:** Accepted
- **Context:** Python and Rust components need a stable interface contract.
- **Decision:** Define contracts in `contracts/` as JSON Schema; validate at boundaries.
- **Rationale:** Language-agnostic, testable, versionable interface definition.

## ADR-005: Shell Completion Support

- **Status:** Accepted
- **Context:** CLI usability benefits from tab completion.
- **Decision:** Generate shell completions for bash, zsh, fish via `shell_completion.py`.
- **Rationale:** Standard CLI UX expectation.
