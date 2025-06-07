# Allegro-RS Development Guidelines

## Core Principles
- **Rust 2024 Edition** - Use latest stable features
- **Zero Panics** - No `unwrap()`, `expect()`, or `panic!()` in production code
- **Explicit Error Handling** - Use `Result<T, E>` with descriptive error types, no silent failures
- **Memory Efficiency** - Stream processing for large CWR files, avoid unbounded allocations
- **Safe Sync Code** - No unsafe or async code is anticipated for CWR file processing
- **SOLID Principles** - Small, focused modules with clear responsibilities

## Code Quality
- **Minimal Comments** - Only document non-obvious business rules that would surprise an experienced developer
- **Automated Checks** - Run `cargo fmt` and `cargo clippy` after every working change
- **Warning-Free Code** - Avoid compiler and clippy warnings as they create noise; if uncertain about best practices, ask for clarification
- **Type Safety** - Leverage Rust's type system for domain modeling (e.g., newtype patterns for IDs)
- **Must Use** - Annotate functions returning `Result` with `#[must_use]` to prevent ignored errors
- **String Types** - Use `&str` by default, `String` when data must outlive input, `Cow<str>` for conditional normalization
- **Trait Design** - Keep traits small and focused, prefer composition over large trait hierarchies
- **Documentation** - Include examples and error descriptions for public API functions
- **Performance** - Profile before optimizing, benchmark critical paths

## Testing
- **Test Data** - Use `.me/` folder for real CWR sample files (add to .gitignore)
- **Test Integrity** - Never modify tests to match code; clarify expected behavior first
- **Property Testing** - Consider proptest for parser edge cases
- **Integration Tests** - Test full CWR file processing workflows

## Dependencies
- **Logging** - Use `log` crate with structured logging where appropriate
- **Error Handling** - Consider `thiserror` for error types, `anyhow` for applications
- **Parsing** - CWR is a fixed-width EDI format, making string splitting the obvious approach

## Domain Context
- **CWR Stability** - CWR spec changes infrequently; optimize for correctness over flexibility
- **Compliance** - Ensure strict adherence to CWR 2.2 specification
- **Validation** - Implement comprehensive validation for all record types