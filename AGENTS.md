# AGENTS.md

## Project Overview
This is a Rust CLI tool named "furnace" for scanning Rust projects, creating snapshots of code structures (functions, structs, traits), performing basic linting on variable names, and outputting results as JSON. It uses Cargo as the build system.

## Essential Commands
- **Build**: `cargo build`
- **Run**: `cargo run -- <RUST_PROJECT_PATH>` (takes a path to a Rust project as argument)
- **Run (JSON)**: `cargo run -- <RUST_PROJECT_PATH> --format json`
- **Test**: `cargo test`
- **Lint**: The tool performs its own linting on the scanned code.

## Code Organization and Structure
- Source files are in `src/` directory.
- `src/main.rs`: Entry point, handles CLI args with clap, walks directory with walkdir, runs linting, outputs results.
- `src/lib.rs`: Library entry point, exposes modules.
- `src/visitor.rs`: Contains `SnapshotVisitor` which uses `syn` to parse Rust code and extract structure.
- `src/types.rs`: Defines structs for snapshots (RustFileSnapshot, FunctionSnapshot, StructSnapshot, TraitSnapshot).
- `src/linting.rs`: Contains linting logic (naming conventions, complexity checks).

## Naming Conventions and Style Patterns
- File names: Lowercase with underscores (e.g., types.rs, linting.rs).
- Struct names: CamelCase (e.g., RustFileSnapshot).
- Function names: Snake case (e.g., lint_snapshots).
- Variable names: Snake case.
- Indentation: 4 spaces.
- Standard Rust style with derive macros for Debug, Clone, Serialize.

## Testing Approach and Patterns
- Unit tests are located in `src/visitor.rs` to verify AST extraction logic.
- Run `cargo test` to execute them.

## Important Gotchas
- Snapshot creation uses `syn` for robust parsing.
- Linting checks for:
    - Variable names (snake_case, discouraged names like "foo").
    - Function names (snake_case).
    - Struct names (PascalCase).
    - Function argument count (> 5).
    - Struct field count (> 10).
- JSON output is available for integration with other tools.