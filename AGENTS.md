# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is **ri** - a Rust implementation of the popular `@antfu/ni` package manager CLI. It provides intelligent package manager detection and consistent commands across different JavaScript ecosystems (npm, yarn, pnpm, bun).

## Build and Development Commands

```bash
# Build the project
cargo build

# Build release version
cargo build --release

# Run tests
cargo test

# Install locally for development
cargo install --path .

# Install from git (production)
cargo install --git https://github.com/your-username/ri.git
```

## Architecture

The project is structured around four main modules:

### Core Modules

- **`detection/`** - Package manager detection logic that identifies npm, yarn, pnpm, or bun based on lock files
- **`commands/`** - High-level command execution functions (install, run, execute, update, uninstall)
- **`interactive/`** - User interface components using the `inquire` crate for interactive selection
- **`config/`** - Configuration management (currently minimal)

### Binary Structure

The project produces 5 separate binaries defined in `Cargo.toml`:
- `ri` (main.rs) - Install packages
- `rr` (bin/rr.rs) - Run scripts with special `-a` flag for adding scripts
- `rx` (bin/rx.rs) - Execute package binaries
- `ru` (bin/ru.rs) - Update packages
- `rd` (bin/rd.rs) - Remove packages

### Package Manager Detection

Detection priority (highest to lowest):
1. Lock files: `bun.lock`/`bun.lockb` → `pnpm-lock.yaml` → `yarn.lock` → `package-lock.json`
2. Falls back to `package.json` (defaults to bun)

### Key Dependencies

- `clap` - CLI argument parsing with derive macros
- `inquire` - Interactive prompts and selections
- `serde` + `serde_json` - JSON parsing for package.json
- `tokio` - Async runtime
- `which` - Command availability checking

## Important Implementation Details

### Interactive Mode Design
- All binaries default to interactive mode when no arguments provided
- Uses arrow key navigation for script selection in `rr`
- Gracefully handles user cancellation with `OperationCanceled`

### Script Management (`rr -a`)
- Creates package.json if missing
- Handles script overwrite confirmation
- Uses serde_json for direct JSON manipulation

### Error Handling
- Commands exit gracefully (code 0) when package.json doesn't exist for script operations
- Package manager availability checked before execution
- All external commands wrapped in Result types

### Package Command Mapping
Each PackageManager enum implements methods for:
- `command()` - Base CLI command
- `install_command()` - Install subcommand
- `run_command()` - Run subcommand
- `exec_command()` - Execute subcommand
- `update_command()` - Update subcommand

## Testing

Run tests with `cargo test`. The codebase uses standard Rust testing patterns with Result types for error handling.