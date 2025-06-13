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
- **ASCII** - CWR spec only supports ASCII, so all reading and writing need to use this encoding!

## Dependencies & Tools
- **ALWAYS CLEAN UP** - All `cargo tests` should pass (unless they are new and the features have not been implemented), then fix any `cargo clippy` issues, then `cargo fmt`
- **"Dependency Restraint** - Prefer fewer, proven dependencies over many small ones; implement simple functionality yourself rather than adding dependencies
- **Core Dependencies** - `log` for logging, `thiserror` for error types, `anyhow` for applications
- **Parsing Approach** - CWR is fixed-width EDI format, string splitting is the obvious approach
- **Commit Message** - Propose a single-line commit message after every working change
- **Warning-Free Code** - Avoid compiler and clippy warnings; ask for clarification if uncertain about best practices
- **Use tools and scripts!** - Use tools (`rg`, `sed`, etc) and scripts (`python3`) to perform mass updates wherever possible
- **DO NOT ADD rusqlite** - If you add rusqlite as a dependency to allegro_cwr you will make me cry.

## Testing Strategy
- **Test Data** - Use `.me/` folder for real CWR sample files (add to .gitignore)
- **Test Integrity** - Never modify tests to match code; clarify expected behavior first
- **Property Testing** - Consider proptest for parser edge cases
- **Integration Tests** - Test full CWR file processing workflows

## Review Process
- **Constructive Review** - Challenge assumptions and provide direct technical feedback; avoid excessive affirmation that prevents growth