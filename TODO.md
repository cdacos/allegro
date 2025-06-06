# TODO

## CWR Domain Types + Procedural Macros

### Core Concept
- **Domain types** encode single-field constraints in type system
- **Validation methods** handle cross-field business logic
- **Procedural macro** generates graceful parsing with structured warnings
- **Permissive parsing** - liberal in what it accepts, explicit about issues via warnings

### Domain Types
```rust
pub struct WorksCount(u32);           // 1-99999 range
pub enum RecordType { Agr, Ari, Alt } // Valid record types
pub enum AgreementType { OS, OA, AA } // Business meaning
pub struct Date(NaiveDate);          // Parsed YYYYMMDD format
pub enum YesNo { Yes, No }            // Y/N values
```

### Design Considerations
- **String Allocations**: Use `Cow<'a, str>` in warnings to borrow when possible
- **Domain Type Design**: 
  - Parse `Date` into `chrono::NaiveDate` rather than storing strings
  - Consider `NonZeroU32` for `WorksCount` if zero is invalid
  - Keep `YesNo` enum if Y/N distinction matters for serialization
- **Default Trait**: Each domain type needs sensible default for unparseable data

### Graceful Parsing Trait
```rust
pub trait CwrFieldParse: Sized + Default {
    fn parse_cwr_field(source: &str, field_name: &str, field_title: &str) -> (Self, Vec<CwrWarning>);
}

#[derive(Debug, Clone, PartialEq)]
pub struct CwrWarning<'a> {
    pub field_name: &'static str,    // Known at compile time
    pub field_title: &'static str,    // Known at compile time
    pub source_str: Cow<'a, str>,    // Borrowed when possible
    pub level: WarningLevel,
    pub description: String,
}

impl CwrWarning<'_> {
    pub fn is_critical(&self) -> bool {
        matches!(self.level, WarningLevel::Critical)
    }
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

### Macro Hygiene Considerations
- Use fully qualified paths in generated code
- Support custom error types per field via attributes
- Allow field-level validation functions
- Generate `FromStr` implementations where appropriate
- Consider generating builders for records with many fields

### Implementation Steps
1. Create domain types with `CwrFieldParse` trait and `Default` implementations
2. Build `#[derive(CwrRecord)]` procedural macro with proper hygiene
3. Convert AGR record to use domain types and annotations
4. Add builder pattern generation for complex records