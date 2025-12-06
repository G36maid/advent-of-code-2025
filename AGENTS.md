# Advent of Code Rust Agent Guidelines

## Workflow

### Problem Solving Process
1. **Read & Analyze**: Read puzzle description from `data/puzzles/<day>.md`
2. **Add Example Input**: Copy example from puzzle into `data/examples/<day>.txt`
3. **Implement Part 1**:
   - Write solution in `src/bin/<day>.rs`
   - Update test assertion with expected example output
   - Run `cargo test --bin <day>` to verify
4. **Test & Submit Part 1**:
   - Run `cargo solve <day> --release` to get answer
   - Submit with `cargo solve <day> --release --submit 1`
5. **Download Part 2**: Run `cargo download <day>` to fetch Part 2 description
6. **Implement Part 2**:
   - Add solution to `part_two()` function
   - Update test assertion
   - Run `cargo test --bin <day>` to verify
7. **Test & Submit Part 2**:
   - Run `cargo solve <day> --release` 
   - Submit with `cargo solve <day> --release --submit 2`
8. **Cleanup**: Run `cargo fmt && cargo clippy --bin <day>` to ensure code quality
9. **Commit**: Commit solution using conventional commits format
    - `<type>[optional scope]: <description>`, where types include: `fix:`, `feat:`, `build:`, `chore:`, `ci:`, `docs:`, `style:`, `refactor:`, `perf:`, `test:`, and others.

### Git Workflow
- **Commit Format**: `feat(day<NN>): solve part 1 and 2` or `feat(day<NN>): solve part 1`
- **Commit After**: Complete solutions (both parts if possible) with tests passing
- **Message Examples**:
  - `feat(day01): solve part 1 and 2`
  - `feat(day02): solve part 1`
  - `refactor(day01): optimize solution performance`
  - `docs: update AGENTS.md with workflow`

## Commands
- **Run Solution**: `cargo solve <day>` (e.g., `cargo solve 01`). Use `--release` for optimized builds.
- **Submit Solution**: `cargo solve <day> --release --submit <part>` (auto-submits via aoc-cli)
- **Scaffold Day**: `cargo scaffold <day>` to generate `src/bin/<day>.rs` and input files.
- **Download Puzzle**: `cargo download <day>` to fetch puzzle description and input.
- **Test**: `cargo test` runs all tests. To test a specific day: `cargo test --bin <day>`.
- **Lint & Format**: Always run `cargo clippy` and `cargo fmt` before finishing a task.

## Code Style & Conventions
- **Formatting**: Strictly follow `rustfmt` defaults.
- **Naming**: `snake_case` for functions/variables, `PascalCase` for structs/enums/traits.
- **Error Handling**: Prefer `Result` propagation (`?`) over `unwrap()`. Handle errors gracefully.
- **Structure**: Solutions reside in `src/bin/`. Shared logic goes in `src/lib.rs` or modules.
- **Imports**: Group standard library, external crates, and internal modules separately.
- **Performance**: This is AoC; prioritize correctness first, then optimization (zero-cost abstractions).
- **Return Types**: Use appropriate types (`u32`, `u64`, `i32`, etc.) based on problem constraints.
- **Comments**: Add comments for complex algorithms or non-obvious logic.
