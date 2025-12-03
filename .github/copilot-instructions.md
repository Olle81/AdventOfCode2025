# Rust Development Workspace Instructions

This workspace is configured for Rust development with the following setup:

## Project Structure
- `src/main.rs` - Main application entry point
- `Cargo.toml` - Project configuration and dependencies
- `.vscode/` - VS Code specific settings and tasks

## Development Workflow
- Use `cargo build` to compile the project
- Use `cargo run` to build and execute the program
- Use `cargo test` to run unit tests
- Use `cargo check` for faster compilation checking

## Code Style
- Follow Rust naming conventions (snake_case for functions/variables, PascalCase for types)
- Use `cargo fmt` for code formatting
- Use `cargo clippy` for linting and suggestions
- Prefer explicit error handling with `Result<T, E>`

## Dependencies
- Add dependencies in `Cargo.toml` under `[dependencies]`
- Use `cargo add <crate>` to add new dependencies
- Prefer well-maintained crates from crates.io

## Testing
- Write unit tests in the same file using `#[cfg(test)]` modules
- Write integration tests in `tests/` directory
- Use `#[test]` attribute for test functions