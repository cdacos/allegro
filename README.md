# Allegro - CWR File Parser and Database Tool

A Rust application for parsing and processing CWR (Common Works Registration) files. CWR is the standard file format used by the music industry to exchange musical work data between publishers, performance rights organizations, and other entities.

This application validates CWR files according to the CWR 2.2 specification (with support for 2.0 and 2.1) and stores the parsed data in SQLite databases for querying and analysis. It handles all standard CWR record types including works registrations, publisher information, writer details, territory data, etc.

## Features

- **CWR 2.2 Support**: Parses all standard record types according to the CWR specification
- **Validation**: Validates field formats, mandatory fields, and data integrity
- **Database Storage**: Stores parsed data in structured SQLite tables for easy querying
- **Error Reporting**: Detailed validation error reporting with line numbers and descriptions
- **Multiple Output Formats**: Supports database storage, JSON streaming, and SQL output
- **Performance**: Efficiently processes large CWR files with thousands of records

## Usage

```bash
# Build the project
cargo build --release

# Basic usage (auto-numbered if DB exists)
target/release/allegro input_file.cwr

# Specify output database file (overwrites if exists)
target/release/allegro -o output.db input_file.cwr
target/release/allegro --output /path/to/database.db input_file.cwr

# View logging
RUST_LOG=info target/release/allegro -o output.db input_file.cwr

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