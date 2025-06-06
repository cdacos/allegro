# Allegro - CWR File Parser and Database Tool

**⚠️ Work in Progress - Beta Software ⚠️**

A Rust application for parsing and processing CWR (Common Works Registration) files. CWR is the standard file format used by the music industry to exchange musical work data between publishers, performance rights organizations, and other entities.

This is currently a **beta/WIP project** that provides basic CWR file parsing for versions 2.0, 2.1, and 2.2. It handles all 33+ standard CWR record types and includes field parsing with a warning system, but **lacks comprehensive business rule validation and is not ready for production use**.


## Current Features

- **Basic CWR Parsing**: Parses all 33+ standard record types for CWR versions 2.0, 2.1, and 2.2
- **Field Extraction**: Extracts individual fields according to CWR specification positions
- **Multiple Output Formats**: SQLite database storage and structured JSON output
- **Type Safety**: Domain types and procedural macros for consistent parsing
- **Warning System**: Basic field-level warnings for malformed data
- **Auto Record Type Detection**: Validates that record types match their content
- **Efficient Processing**: Streaming parser with batch database operations handles large files well

## Missing/TODO

- **Business Rule Validation**: Comprehensive cross-field and inter-record validation
- **Complete Field Validation**: Many field constraints and validations not yet implemented  
- **Error Recovery**: Limited handling of severely malformed files
- **Documentation**: API documentation and validation rule documentation incomplete

## Usage

```bash
# Build the project
cargo build --release

# Parse to SQLite database (default format)
target/release/allegro input_file.cwr

# Output JSON to stdout
target/release/allegro --format json input_file.cwr

# Specify output database file (overwrites if exists)
target/release/allegro -o output.db input_file.cwr

# Force specific CWR version
target/release/allegro --version 2.1 input_file.cwr

# View logging output
RUST_LOG=info target/release/allegro input_file.cwr

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

The project is organized into focused crates:

- **`allegro`** - CLI binary for processing CWR files
- **`allegro_cwr`** - Core CWR parsing library with domain types and validation
- **`allegro_cwr_derive`** - Procedural macros for automatic record parsing generation
- **`allegro_cwr_sqlite`** - SQLite database operations and schema management
- **`allegro_cwr_json`** - JSON output formatting with structured context

## Implementation

The parsing system uses modern Rust procedural macros with the `#[derive(CwrRecord)]` attribute to automatically generate parsing logic for each record type. Record definitions are clean and declarative:

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, CwrRecord)]
#[cwr(test_data = "AGR00000001000000011234567890123...")]
pub struct AgrRecord {
    #[cwr(title = "Always 'AGR'", start = 0, len = 3)]
    pub record_type: String,
    
    #[cwr(title = "Transaction sequence number", start = 3, len = 8)]
    pub transaction_sequence_num: String,
    
    #[cwr(title = "Agreement start date YYYYMMDD", start = 49, len = 8)]
    pub agreement_start_date: Date,
    
    // ... other fields
}
```

The derive macro automatically generates `from_cwr_line()` parsing methods with field validation and warning collection.