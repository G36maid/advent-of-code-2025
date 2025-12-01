# Advent of Code Rust Agent Guidelines

## Commands
- **Run Solution**: `cargo solve <day>` (e.g., `cargo solve 01`). Use `--release` for optimized builds.
- **Scaffold Day**: `cargo scaffold <day>` to generate `src/bin/<day>.rs` and input files.
- **Test**: `cargo test` runs all tests. To test a specific day: `cargo test --bin <day>`.
- **Lint & Format**: Always run `cargo clippy` and `cargo fmt` before finishing a task.

## Code Style & Conventions
- **Formatting**: Strictly follow `rustfmt` defaults.
- **Naming**: `snake_case` for functions/variables, `PascalCase` for structs/enums/traits.
- **Error Handling**: Prefer `Result` propagation (`?`) over `unwrap()`. Handle errors gracefully.
- **Structure**: Solutions reside in `src/bin/`. Shared logic goes in `src/lib.rs` or modules.
- **Imports**: Group standard library, external crates, and internal modules separately.
- **Performance**: This is AoC; prioritize correctness first, then optimization (zero-cost abstractions).
