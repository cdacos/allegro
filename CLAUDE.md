# Allegro-RS Development Guidelines

## Language & Style
- **Rust 2024 Edition** - Released Feb. 20, 2025!
- **Zero Panics** - No `unwrap()`, `expect()`, or `panic!()` in production code
- **Explicit Error Handling** - Use `Result<T, E>` with descriptive error types, no silent failures
- **Type Safety** - Leverage Rust's type system for domain modeling (e.g., newtype patterns for IDs)
- **String Types** - Use `&str` by default, `String` when data must outlive input, `Cow<str>` for conditional normalization
- **Must Use** - Annotate functions returning `Result` with `#[must_use]` to prevent ignored errors

## Code Documentation
- **Minimal Comments** - Only document non-obvious business rules that would surprise an experienced developer
- **Public API Documentation** - Include examples and error descriptions for all public functions
- **Trait Design** - Keep traits small and focused, prefer composition over large trait hierarchies

## Architecture & Performance
- **SOLID Principles** - Small, focused modules with clear responsibilities
- **Memory Efficiency** - Stream processing for large CWR files, avoid unbounded allocations
- **Safe Sync Code** - No unsafe or async code is anticipated for CWR file processing
- **Performance** - Avoid decisions that significantly degrade performance; profile critical paths when optimizing
- **CWR Domain** - CWR spec changes infrequently; optimize for correctness over flexibility, ensure strict CWR 2.2 compliance

## Dependencies & Tools
- **"Dependency Restraint** - Prefer fewer, proven dependencies over many small ones; implement simple functionality yourself rather than adding dependencies
- **Core Dependencies** - `log` for logging, `thiserror` for error types, `anyhow` for applications
- **Parsing Approach** - CWR is fixed-width EDI format, string splitting is the obvious approach
- **Automated Checks** - Run `cargo fmt` and `cargo clippy` after every working change
- **Commit Message** - Propose a single-line commit message after every working change
- **Warning-Free Code** - Avoid compiler and clippy warnings; ask for clarification if uncertain about best practices

## Testing Strategy
- **Test Data** - Use `.me/` folder for real CWR sample files (add to .gitignore)
- **Test Integrity** - Never modify tests to match code; clarify expected behavior first
- **Property Testing** - Consider proptest for parser edge cases
- **Integration Tests** - Test full CWR file processing workflows

## Review Process
- **Constructive Review** - Challenge assumptions and provide direct technical feedback; avoid excessive affirmation that prevents growth