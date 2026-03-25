# trash-cli

Command-line interface for moving files to the system trash.

## Stack

- Language: Python
- Build: pyproject.toml / setup.py
- Tests: pytest, conftest.py

## Development

```bash
pip install -e ".[dev]"
pytest
```

## Phenotype Org Rules

- UTF-8 encoding only in all text files. No Windows-1252 smart quotes or special characters.
- Worktree discipline: canonical repo stays on `main`; feature work in worktrees.
- CI completeness: fix all CI failures on PRs, including pre-existing ones.
- Never commit agent directories (`.claude/`, `.codex/`, `.gemini/`, `.cursor/`).
