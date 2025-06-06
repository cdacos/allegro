# TODO

## CWR Domain Types + Procedural Macros

### Core Concept
- **Domain types** encode single-field constraints in type system
- **Validation methods** handle cross-field business logic
- **Procedural macro** generates graceful parsing with structured warnings

### Domain Types
```rust
pub struct WorksCount(u32);           // 1-99999 range
pub enum RecordType { Agr, Ari, Alt } // Valid record types
pub enum AgreementType { OS, OA, AA } // Business meaning
pub struct Date(String);              // YYYYMMDD format
pub enum YesNo { Yes, No }            // Y/N values
```

### Graceful Parsing Trait
```rust
pub trait CwrFieldParse: Sized {
    fn parse_cwr_field(source: &str, field_name: &str, field_title: &str) -> (Self, Vec<CwrWarning>);
}

#[derive(Debug, Clone, PartialEq)]
pub struct CwrWarning {
    pub field_name: String,
    pub field_title: String, 
    pub source_str: String,
    pub level: WarningLevel,
    pub description: String,
}
```

### Annotated Struct
```rust
#[derive(Debug, Clone, PartialEq, CwrRecord)]
#[cwr(test_data = "AGR00000001000000011234567890123AA20231201Y00001")]
pub struct AgrRecord {
    #[cwr(title = "Record Type", start = 0, len = 3)]
    pub record_type: RecordType,
    
    #[cwr(title = "Agreement Type", start = 47, len = 2)]  
    pub agreement_type: AgreementType,
    
    #[cwr(title = "Number of Works", start = 99, len = 5)]
    pub number_of_works: WorksCount,
}
```

### Implementation Steps
1. Create domain types with `CwrFieldParse` trait
2. Build `#[derive(CwrRecord)]` procedural macro
3. Convert AGR record to use domain types and annotations