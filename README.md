# Allegro - CWR File Parser

Looking for a project to revisit and improve my Rust skills, I thought a CWR parser would be fun!

CWR is the standard file format used by the music industry to exchange musical work data between publishers, performance rights organizations ("societies"), and other entities.

The specification is a [CISAC](https://www.cisac.org) standard and v2 continues to be very popular.

- [Specification for CWR v2.1](https://members.cisac.org/CisacPortal/cisacDownloadFollow.do?docId=37079)
- [Specification for CWR v2.2](https://members.cisac.org/CisacPortal/cisacDownloadFollow.do?docId=41804)

There's also a [Common Works Registration User Manual](https://members.cisac.org/CisacPortal/cisacDownloadFollow.do?docId=22272).

This is currently an evolving project that provides CWR file parsing for versions 2.0, 2.1, and 2.2.

If you work in music publishing and this is useful, let me know!

## Current Features

- **Bidirectional CWR Conversion**: Full parsing and writing of all 33+ standard record types for CWR versions 2.0, 2.1, and 2.2
- **Type Safety**: Domain types and procedural macros for consistent parsing and serialization
- **Warning System**: Basic field-level warnings for malformed data
- **Performance**: Processes ~500K records/second with full parsing and database insertion (on my Macbook Pro M4!)
- **Round-trip Fidelity**: Edit CWR data in JSON or SQLite format and export back to valid CWR files
 
### Architecture

The main library (`allegro-cwr`) streams CWR lines into typed records, and vice versa. The parser is deliberately agnostic in terms of final usage.

We have two handler projects that demonstrate bidirectional usage of the library:

- **`allegro-cwr-json`**: Bidirectional conversion between CWR ↔ JSON formats (CLI wrapper: `cwr-json`)
- **`allegro-cwr-sqlite`**: Bidirectional conversion between CWR ↔ SQLite database (with tables for each record, such as `cwr_hdr`) (CLI wrapper: `cwr-sqlite`)

## TODO

- **Business Rule Validation**: Comprehensive cross-field and inter-record validation
- **ACK File Handling**: Support for acknowledgment file processing and generation

## Usage

```bash
# Build the project
cargo build --release

# CWR ↔ SQLite conversion (auto-detects format)
target/release/cwr-sqlite input_file.cwr         # CWR → SQLite
target/release/cwr-sqlite database.db            # SQLite → CWR

# CWR ↔ JSON conversion (auto-detects format)  
target/release/cwr-json input_file.cwr           # CWR → JSON
target/release/cwr-json data.json                # JSON → CWR

# Specify output files
target/release/cwr-sqlite -o output.db input_file.cwr    # CWR → SQLite
target/release/cwr-sqlite -o output.cwr database.db      # SQLite → CWR
target/release/cwr-json -o output.json input_file.cwr    # CWR → JSON
target/release/cwr-json -o output.cwr data.json          # JSON → CWR

# SQLite: specify file ID for multi-file databases
target/release/cwr-sqlite --file-id 2 -o output.cwr database.db

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

### Editing Workflow Examples

```bash
# Edit CWR data via SQLite
target/release/cwr-sqlite input.cwr              # Import to SQLite
sqlite3 input.cwr.db "UPDATE cwr_hdr SET sender_name = 'NEW PUBLISHER';"
target/release/cwr-sqlite -o edited.cwr input.cwr.db  # Export modified CWR

# Edit CWR data via JSON
target/release/cwr-json input.cwr                # Import to JSON  
# Edit the JSON file with your preferred editor
target/release/cwr-json -o edited.cwr input.json # Export modified CWR
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