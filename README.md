# Allegro - CWR File Parser

Looking for a project to revisit and improve my Rust skills, I thought a CWR parser would be fun!

CWR is the standard file format used by the music industry to exchange musical work data between publishers, performance rights organizations ("societies"), and other entities.

The specification is a [CISAC](https://www.cisac.org) standard and v2 continues to be very popular.

- [Specification for CWR v2.1](https://members.cisac.org/CisacPortal/cisacDownloadFollow.do?docId=37079)
- [Specification for CWR v2.2](https://members.cisac.org/CisacPortal/cisacDownloadFollow.do?docId=41804)

There's also a [Common Works Registration User Manual](https://members.cisac.org/CisacPortal/cisacDownloadFollow.do?docId=22272).

This is currently an evolving project that provides CWR file parsing for versions 2.0, 2.1, and 2.2.

If you work in music publishing and this is of interest, let me know!

## Current Features

- **Bidirectional CWR Conversion**: Full parsing and writing of all 33+ standard record types for CWR versions 2.0, 2.1, and 2.2
- **Type Safety**: Domain types and procedural macros for consistent parsing and serialization
- **Warning System**: Basic field-level warnings for malformed data
- **Performance**: Processes ~500K records/second with full parsing and database insertion (on my Macbook Pro M4!)
- **Round-trip Fidelity**: Edit CWR data in JSON or SQLite format and export back to valid CWR files with identical formatting
 
### Architecture

The main library (`allegro-cwr`) streams CWR lines into typed records, and vice versa. The parser is deliberately agnostic in terms of final usage.

We have four handler projects that demonstrate different uses of the library:

- **`allegro-cwr-json`**: Bidirectional conversion between CWR ↔ JSON formats (CLI wrapper: `cwr-json`)
- **`allegro-cwr-sqlite`**: Bidirectional conversion between CWR ↔ SQLite database (with tables for each record, such as `cwr_hdr`) (CLI wrapper: `cwr-sqlite`)
- **`allegro-cwr-obfuscate`**: Privacy-preserving obfuscation of sensitive CWR data while maintaining referential integrity (CLI wrapper: `cwr-obfuscate`)
- **`allegro-cwr-validate`**: Round-trip integrity validation to ensure CWR files can be parsed and re-serialized identically (CLI wrapper: `cwr-validate`)

## TODO

- **Business Rule Validation**: Comprehensive cross-field and inter-record validation
- **ACK File Handling**: Support for acknowledgment file processing and generation

## Usage

```bash
# Build the project
cargo build --release
```

### File Naming Behavior

All CLI tools follow consistent file naming conventions:

**Single File Processing:**
- **With `-o`**: Overwrites the specified output file
- **Without `-o`**: Uses tool-specific defaults (stdout for JSON, auto-generated for SQLite/obfuscate)

**Multiple File Processing:**  
- **With `-o`**: Creates numbered output files (`output.1`, `output.2`, etc.)
- **Without `-o`**: Generates default output files with appropriate extensions

**Input Format Detection:**
- **cwr-json** and **cwr-sqlite**: Auto-detect format and convert bidirectionally (CWR ↔ JSON/SQLite)
- **cwr-obfuscate**: Auto-detects and validates CWR format (only CWR files supported)
- **cwr-validate**: Validates CWR format and round-trip integrity

**Default Output Behavior:**
- **Text formats** (JSON, CWR): Single files without `-o` write to stdout (suitable for piping)
- **Binary formats** (SQLite): Single files without `-o` create output files (databases can't be piped to stdout)

### Basic Usage

```bash
# CWR ↔ SQLite conversion (auto-detects format)
target/release/cwr-sqlite input_file.cwr         # → input_file.cwr.db
target/release/cwr-sqlite database.db            # → stdout (CWR)

# CWR ↔ JSON conversion (auto-detects format)  
target/release/cwr-json input_file.cwr           # → stdout (JSON)
target/release/cwr-json data.json                # → stdout (CWR)

# CWR obfuscation (privacy-preserving test data)
target/release/cwr-obfuscate input_file.cwr      # → stdout (obfuscated CWR)

# CWR validation (round-trip integrity checking)
target/release/cwr-validate input_file.cwr       # Validate round-trip integrity
```

### Output File Control

```bash
# Single file with custom output (overwrites existing files)
target/release/cwr-sqlite -o output.db input_file.cwr   # → output.db
target/release/cwr-json -o output.json input_file.cwr   # → output.json
target/release/cwr-obfuscate -o safe.cwr input_file.cwr # → safe.cwr

# Multiple files with custom output (numbered files)
target/release/cwr-sqlite -o db *.cwr            # → db.1, db.2, db.3, ...
target/release/cwr-json -o result.json *.cwr     # → result.json.1, result.json.2, ...
target/release/cwr-obfuscate -o clean.cwr *.cwr  # → clean.cwr.1, clean.cwr.2, ...

# Multiple files without -o (auto-generated names)
target/release/cwr-sqlite file1.cwr file2.cwr    # → file1.cwr.db, file2.cwr.db
target/release/cwr-json file1.cwr file2.cwr      # → file1.cwr.json, file2.cwr.json
target/release/cwr-obfuscate file1.cwr file2.cwr # → file1.cwr.obfuscated, file2.cwr.obfuscated
```

### Advanced Options

```bash
# SQLite: specify file ID for multi-file databases
target/release/cwr-sqlite --file-id 2 -o output.cwr database.db

# Force specific CWR version
target/release/cwr-sqlite --cwr 2.1 input_file.cwr
target/release/cwr-json --cwr 2.1 input_file.cwr
target/release/cwr-obfuscate --cwr 2.1 input_file.cwr
target/release/cwr-validate --cwr 2.1 input_file.cwr

# View logging output
RUST_LOG=info target/release/cwr-sqlite input_file.cwr
RUST_LOG=info target/release/cwr-json input_file.cwr
RUST_LOG=info target/release/cwr-obfuscate input_file.cwr
RUST_LOG=info target/release/cwr-validate input_file.cwr

# Show help
target/release/cwr-sqlite --help
target/release/cwr-json --help
target/release/cwr-obfuscate --help
target/release/cwr-validate --help
```

### Workflow Examples

```bash
# Edit CWR data via SQLite
target/release/cwr-sqlite input.cwr              # → input.cwr.db
sqlite3 input.cwr.db "UPDATE cwr_hdr SET sender_name = 'NEW PUBLISHER';"
target/release/cwr-sqlite -o edited.cwr input.cwr.db  # → edited.cwr

# Edit CWR data via JSON
target/release/cwr-json -o input.json input.cwr  # → input.json
# Edit the JSON file with your preferred editor
target/release/cwr-json -o edited.cwr input.json # → edited.cwr

# Process multiple files efficiently
target/release/cwr-sqlite *.cwr                  # → file1.cwr.db, file2.cwr.db, ...
target/release/cwr-json *.cwr                    # → file1.cwr.json, file2.cwr.json, ...
target/release/cwr-obfuscate *.cwr               # → file1.cwr.obfuscated, file2.cwr.obfuscated, ...

# Create privacy-safe test data
target/release/cwr-obfuscate production.cwr      # → production.cwr.obfuscated
# Share obfuscated file safely - all names, titles, IPIs are fake but consistent
target/release/cwr-sqlite production.cwr.obfuscated  # → production.cwr.obfuscated.db

# Validate round-trip integrity (for testing)
target/release/cwr-sqlite input.cwr              # → input.cwr.db
target/release/cwr-sqlite -o roundtrip.cwr input.cwr.db  # → roundtrip.cwr
diff input.cwr roundtrip.cwr                     # Should be identical
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

- **`allegro_cwr`** - Core CWR parsing library with domain types and validation
- **`allegro_cwr_derive`** - Procedural macros for automatic record parsing generation
- **`allegro_cwr_sqlite`** - SQLite database operations, schema management, and `cwr-sqlite` CLI binary
- **`allegro_cwr_json`** - JSON output formatting with structured context and `cwr-json` CLI binary
- **`allegro_cwr_obfuscate`** - Privacy-preserving obfuscation with consistent mapping and `cwr-obfuscate` CLI binary
- **`allegro_cwr_validate`** - Round-trip integrity validation and CWR compliance checking with `cwr-validate` CLI binary

## Implementation

The parsing system uses Rust procedural macros with the `#[derive(CwrRecord)]` attribute to automatically generate parsing logic for each record type. Record definitions are clean and declarative:

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

The derive macro automatically generates `from_cwr_line()` and `to_cwr_line()` parsing methods with field validation and warning collection.