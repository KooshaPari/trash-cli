# trash-cli

Command-line interface for moving files to the system trash (recycle bin) instead of permanent deletion.

## Features

- Move files to system trash instead of `rm`
- Restore files from trash
- Empty trash
- List trash contents

## Installation

```bash
pip install -e ".[dev]"
```

## Usage

```bash
trash-put <file>     # Move file to trash
trash-list           # List files in trash
trash-restore <file> # Restore file from trash
trash-empty          # Empty the trash
```

## Development

```bash
pip install -e ".[dev]"
pytest
```

## License

MIT
