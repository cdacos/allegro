# Allegro - CWR File Parser

Looking for a project to revisit and improve my Rust skills, I thought a CWR parser would be fun!

CWR is the standard file format used by the music industry to exchange musical work data between publishers, performance rights organizations ("societies"), and other entities.

The specification is [CISAC](https://www.cisac.org) standard and v2 continues to be very popular.

- [Specification for CWR v2.1](https://members.cisac.org/CisacPortal/cisacDownloadFollow.do?docId=37079)
- [Specification for CWR v2.2](https://members.cisac.org/CisacPortal/cisacDownloadFollow.do?docId=41804)

There's also a [Common Works Registration User Manual](https://members.cisac.org/CisacPortal/cisacDownloadFollow.do?docId=22272).

This is currently an evolving project that provides CWR file parsing for versions 2.0, 2.1, and 2.2.

If you work in music publishing and this is useful, let me know!

## Current Features

- **Basic CWR Parsing**: Parses all 33+ standard record types for CWR versions 2.0, 2.1, and 2.2
- **Type Safety**: Domain types and procedural macros for consistent parsing
- **Warning System**: Basic field-level warnings for malformed data
- **Performance**: Processes ~500K records/second with full parsing and database insertion (on my Macbook Pro M4!)
 
### Architecture

The main library (`allegro-cwr`) streams CWR lines into typed records, and vice versa. The parser is deliberately agnostic in terms of final usage.

We have two handler projects that demonstrate usage of the library:

- **`allegro-cwr-json`**: Convert the CWR format into a stream of JSON records (CLI wrapper: `cwr-json`)
- **`allegro-cwr-sqlite`**: Load the CWR data into a SQLite database (with tables for each record, such as `cwr_hdr`) (CLI wrapper: `cwr-sqlite`)

## TODO

- **Business Rule Validation**: Comprehensive cross-field and inter-record validation
- **Serialisation to CWR**: The current focus has been on reading CWRs; writing and handling ACKs to come

## Usage

```bash
# Build the project
cargo build --release

# Parse to SQLite database
target/release/cwr-sqlite input_file.cwr

# Output JSON to stdout
target/release/cwr-json input_file.cwr

# Specify output database file (overwrites if exists)
target/release/cwr-sqlite -o output.db input_file.cwr

# Force specific CWR version
target/release/cwr-sqlite --cwr 2.1 input_file.cwr
target/release/cwr-json --cwr 2.1 input_file.cwr

# View logging output
RUST_LOG=info target/release/cwr-sqlite input_file.cwr
RUST_LOG=info target/release/cwr-json input_file.cwr

# Show help
target/release/cwr-sqlite --help
target/release/cwr-json --help
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