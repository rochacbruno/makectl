## makectl: description="Build Rust project with cargo"
.PHONY: build
build: ## Build the project
	cargo build

.PHONY: build-verbose
build-verbose: ## Build with verbose output
	cargo build --verbose
