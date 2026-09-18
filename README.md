# ri - Rust Package Manager CLI

A Rust implementation of the popular `@antfu/ni` package manager CLI. **ri** provides intelligent package manager detection and consistent commands across different JavaScript ecosystems.

> **Note**: This tool is specifically designed for JavaScript/Node.js projects (npm, yarn, pnpm, bun) and is implemented in Rust.

## Features

- 🚀 **Intelligent Detection** - Automatically detects npm or bun based on lock files
- 🎯 **Simple Commands** - Consistent shortcuts across different package managers
- 🔍 **Interactive Mode** - Default interactive mode with arrow key navigation
- 📦 **Cross-platform** - Works on Windows, macOS, and Linux

## Installation

### Method 1: Install from Git (Recommended)

```bash
# Install globally using cargo (adds to ~/.cargo/bin/)
cargo install --locked --git https://github.com/oggnimodd/ri.git

# Or from a local clone. install.sh wraps this and re-asserts the binary
# permissions, which cargo install has been known to drop.
./install.sh
```

### Method 2: Build from Source

```bash
# Build from source
cargo build --release

# The binaries will be available in target/release/
# ri, rr, rx, ru, rd
```

After installation with `cargo install`, the commands will be available globally in your system's PATH:

```bash
# Verify installation
ri --version
rr --help
```

## Usage

### Install Packages (`ri`)

```bash
# Interactive mode - shows package selection
ri

# Install specific packages
ri react lodash

# Verbose output
ri -v react
```

### Run Scripts (`rr`)

```bash
# Interactive mode - select scripts with arrow keys
rr

# Run specific script
rr dev

# Add a new script to package.json
rr -a

# Verbose output
rr -v start
```

### Execute Commands (`rx`)

```bash
# Execute npm binaries
rx node --version
rx tsc --init

# Execute bun binaries (works in bun projects)
rx vite
```

### Update Packages (`ru`)

```bash
# Interactive mode - select packages to update
ru

# Update specific packages
ru react lodash

# Update all dependencies
ru -v
```

### Remove Packages (`rd`)

```bash
# Interactive mode - select packages to remove (shows dependencies as options)
rd

# Remove specific packages
rd lodash

# Verbose output
rd -v react
```

### Script Management (`rr -a`)

```bash
# Add a new script interactively
rr -a

# Features:
- Creates package.json if it doesn't exist
- Prompts for script name and command
- Warns if script already exists
- Overwrites with confirmation
```

## Package Manager Detection

**ri** automatically detects your package manager based on:

- `package-lock.json` → npm
- `yarn.lock` → yarn
- `pnpm-lock.yaml` → pnpm
- `bun.lock` or `bun.lockb` → bun (supports both old and new formats)
- `package.json` → npm (default)

## Supported Package Managers

| Manager | Install | Run | Execute | Update | Remove |
|---------|---------|-----|---------|--------|--------|
| npm     | ✅ `npm install` | ✅ `npm run` | ✅ `npm exec` | ✅ `npm update` | ✅ `npm uninstall` |
| yarn    | ✅ `yarn add` | ✅ `yarn run` | ✅ `yarn exec` | ✅ `yarn upgrade` | ✅ `yarn remove` |
| pnpm    | ✅ `pnpm add` | ✅ `pnpm run` | ✅ `pnpm exec` | ✅ `pnpm update` | ✅ `pnpm remove` |
| bun     | ✅ `bun install` | ✅ `bun run` | ✅ `bun x` | ✅ `bun update` | ✅ `bun remove` |

## CLI Reference

### Global Options

- `-v, --verbose` - Show detailed output and commands being executed
- `-h, --help` - Show help information

### rr Options

- `-a, --add-script` - Add a new script to package.json interactively

### Commands

- `ri [packages...]` - Install packages
- `rr [script]` - Run package scripts
- `rr -a` - Add new script to package.json
- `rx <command> [args...]` - Execute package binaries
- `ru [packages...]` - Update packages  
- `rd [packages...]` - Remove packages

## Examples

### Node.js Project

```bash
# Create a new project
mkdir my-app && cd my-app
npm init -y

# Use ri to install dependencies
ri react react-dom

# Use rr to run scripts interactively
rr

# Use rx to execute CLI tools
rx create-react-app my-new-app

# Update all dependencies
ru

# Remove a package
rd react
```

### Bun Project

```bash
# Create a bun project
mkdir my-bun-app && cd my-bun-app
bun init

# ri will automatically detect bun
ri hono

# Run scripts with rr
rr dev

# Execute with rx
rx bun --version
```

### pnpm Project

```bash
# Create a pnpm project
mkdir my-pnpm-app && cd my-pnpm-app
pnpm init

# ri will automatically detect pnpm
ri express

# Run scripts with rr
rr start

# Execute with rx
rx node --version
```

## Interactive Mode

When no arguments are provided, **ri** defaults to interactive mode:

- **Package selection**: Type package names (comma-separated)
- **Script selection**: Navigate with arrow keys, press Enter to select
- **Simple interface**: Clean, minimal prompts

## Development

```bash
# Build the project
cargo build

# Run tests
cargo test

# Build release version
cargo build --release
```

## License

MIT

## Inspired By

This project is inspired by [@antfu/ni](https://github.com/antfu/ni) - the original intelligent package manager CLI.
