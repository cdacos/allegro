# CWR File Parser and Validator

A Rust workspace for parsing CWR (Common Works Registration) files into SQLite databases.

## Usage

```bash
# Build the project
cargo build --release

# Basic usage (auto-numbered if DB exists)
target/release/allegro input_file.cwr

# Specify output database file (overwrites if exists)
target/release/allegro -o output.db input_file.cwr
target/release/allegro --output /path/to/database.db input_file.cwr

# Show help
target/release/allegro --help
```

## Development

```bash
# Format code
cargo fmt

# Run clippy
cargo clippy

# Run tests
cargo test
```

## Architecture

- `allegro_cwr_sqlite` - SQLite database operations for CWR data
- `allegro_cwr` - CWR file parsing library  
- `allegro` - CLI binary for processing CWR files