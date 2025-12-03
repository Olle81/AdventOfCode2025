# Advent of Code 2025

A Rust project for solving Advent of Code 2025 puzzles.

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) (installed automatically during setup)
- VS Code with the following extensions (installed automatically):
  - **rust-analyzer**: Provides IDE-like features for Rust
  - **CodeLLDB**: Debugger for Rust applications
  - **Crates**: Helps manage Cargo dependencies
  - **Even Better TOML**: Enhanced TOML file support

### Building and Running

Use the VS Code tasks or run commands manually in the terminal:

#### Using VS Code Tasks (Recommended)

- **Build**: `Ctrl+Shift+P` → "Tasks: Run Task" → "Cargo Build"
- **Run**: `Ctrl+Shift+P` → "Tasks: Run Task" → "Cargo Run"  
- **Test**: `Ctrl+Shift+P` → "Tasks: Run Task" → "Cargo Test"
- **Check**: `Ctrl+Shift+P` → "Tasks: Run Task" → "Cargo Check"

#### Using Terminal Commands

```bash
# Build the project
cargo build

# Run the project
cargo run

# Run tests
cargo test

# Check for errors without building
cargo check

# Format code
cargo fmt

# Run clippy for linting
cargo clippy
```

### Debugging

Use `F5` to start debugging or go to the Debug panel and select one of the configured launch targets:
- "Debug executable 'advent-of-code-2025'"
- "Debug unit tests in executable 'advent-of-code-2025'"

### Adding Dependencies

Add dependencies to `Cargo.toml`:

```toml
[dependencies]
serde = "1.0"
tokio = "1.0"
```

Or use the command line:

```bash
cargo add serde
cargo add tokio
```

### Project Structure

```
├── src/
│   └── main.rs          # Main application entry point
├── Cargo.toml           # Project configuration and dependencies
├── .vscode/             # VS Code configuration
│   ├── tasks.json       # Build/run/test tasks
│   ├── launch.json      # Debug configurations
│   └── settings.json    # Editor settings for Rust
└── README.md            # This file
```

## Development Tips

- Use `rust-analyzer` for code completion, error checking, and refactoring
- Use `cargo fmt` to format your code according to Rust style guidelines
- Use `cargo clippy` for additional linting and suggestions
- The VS Code setup includes automatic formatting on save
- Inlay hints are enabled to show type information and parameter names

Happy coding! 🦀