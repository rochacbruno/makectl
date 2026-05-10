# makectl

[![Build Status](https://img.shields.io/endpoint.svg?url=https%3A%2F%2Factions-badge.atrox.dev%2Frochacbruno%2Fmakectl%2Fbadge%3Fref%3Dmaster&style=flat)](https://actions-badge.atrox.dev/rochacbruno/makectl/goto?ref=master)
![Crate version](https://img.shields.io/crates/v/makectl)

Generate and manage targets in your Makefiles.

makectl is a command-line tool that helps you create, lint, format, and manage Makefiles with reusable templates for multiple languages and frameworks.

## Features

- **Template library** - Built-in templates for Python, Rust, Go, Node.js, Docker, and more
- **Makefile linting** - Detect common issues like missing `.PHONY`, spaces instead of tabs, duplicate targets
- **Makefile formatting** - Auto-format for consistent style
- **Syntax validation** - Check for errors before running make
- **Managed blocks** - Add and remove targets without breaking custom rules
- **Self-documenting help** - Every generated Makefile includes a `make help` target
- **Shell completions** - Tab completion for bash, zsh, fish, and PowerShell

## Installation

```bash
cargo install makectl
```

Or build from source:

```bash
git clone https://github.com/rochacbruno/makectl
cd makectl
cargo install --path .
```

## Quick Start

```bash
# Create a new Makefile for a Python project
makectl init --lang python

# See what targets were added
make help

# Add more templates
makectl add rust/build go/test

# Lint your Makefile
makectl lint

# Format it
makectl fmt
```

## Commands

### `makectl init`

Create a new Makefile from scratch. Optionally include all templates for a specific language.

```bash
makectl init                  # Minimal Makefile with help target
makectl init --lang python    # Python project (venv, test, lint, format, clean, publish)
makectl init --lang rust      # Rust project (build, test, fmt, clippy, release, doc)
makectl init --lang go        # Go project (build, test, fmt, vet, lint, clean)
makectl init --lang node      # Node.js project (install, build, test, lint, format, dev, clean)
makectl init --force          # Overwrite existing Makefile
```

### `makectl add`

Add template targets to an existing Makefile. Targets are wrapped in managed block markers so they can be tracked and removed later.

```bash
makectl add python/test           # Add Python test target
makectl add rust/build go/test    # Add multiple templates at once
makectl add generic/docker        # Add Docker build/run/push targets
```

Adding the same template twice is a no-op (deduplication).

### `makectl remove`

Remove managed targets from a Makefile. Only targets added by makectl (inside managed blocks) can be removed.

```bash
makectl remove test           # Remove the test target
makectl remove test build     # Remove multiple targets
```

### `makectl list`

List available templates or inspect targets in an existing Makefile.

```bash
makectl list                  # Show all available templates
makectl list --lang python    # Filter templates by language
makectl list --targets        # Show targets in ./Makefile
```

### `makectl lint`

Check a Makefile for common issues and best practices. Exits with code 1 if errors are found (useful in CI).

```bash
makectl lint
makectl lint --file path/to/Makefile
```

Lint rules:
- **missing-phony** - Targets like `test`, `clean`, `build` should be `.PHONY`
- **spaces-instead-of-tabs** - Recipe lines must use tabs, not spaces
- **duplicate-targets** - Same target defined more than once
- **missing-help-target** - No self-documenting `help` target
- **empty-recipes** - Target with no recipe and no prerequisites
- **hardcoded-paths** - Absolute paths that should be variables

### `makectl validate`

Validate Makefile syntax (strict subset of lint - only errors, no suggestions).

```bash
makectl validate
```

### `makectl fmt`

Format a Makefile for consistent style. Removes trailing whitespace, collapses blank lines, ensures the file ends with a newline.

```bash
makectl fmt                # Format in place
makectl fmt --check        # Check only (exit 1 if changes needed, for CI)
```

### `makectl tips`

Display Makefile best practices and tips.

```bash
makectl tips
```

### `makectl completions`

Generate shell completion scripts.

```bash
makectl completions bash > /etc/bash_completion.d/makectl
makectl completions zsh > ~/.zsh/completions/_makectl
makectl completions fish > ~/.config/fish/completions/makectl.fish
```

## Available Templates

| Category | Templates |
|----------|-----------|
| **generic** | help, clean, docker, docker-compose, git-hooks |
| **python** | venv, test, lint, format, clean, publish |
| **rust** | build, test, fmt, clippy, release, doc |
| **go** | build, test, fmt, vet, lint, clean |
| **node** | install, build, test, lint, format, dev, clean |

Use `makectl list` for full descriptions.

## Interactive Mode

Add `-i` (or `--interactive`) to get prompted with menus instead of passing arguments:

```bash
makectl init -i      # Choose language, pick templates, configure defaults
makectl add -i       # Multi-select from available templates (already-added are excluded)
makectl remove -i    # Multi-select from managed targets to remove
```

## Global Options

```
-f, --file <FILE>    Path to Makefile (default: ./Makefile)
-i, --interactive    Interactive mode (prompts for selections)
-h, --help           Show help
-V, --version        Show version
```

## How Managed Blocks Work

When makectl adds a target, it wraps it in markers:

```makefile
# MAKECTL MANAGED START test python/test
.PHONY: test
test: ## Run tests
	pytest tests/ -v
# MAKECTL MANAGED END test
```

This allows makectl to:
- Track which targets it manages
- Remove targets cleanly with `makectl remove`
- Prevent duplicate additions
- Never touch your custom targets

## License

MIT
