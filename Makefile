.DEFAULT_GOAL := help

.PHONY: help
help: ## Show this help message
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

.PHONY: build
build: ## Build the project
	cargo build

.PHONY: test
test: ## Run all tests
	cargo test

.PHONY: fmt
fmt: ## Format code
	cargo fmt --all

.PHONY: fmt-check
fmt-check: ## Check code formatting
	cargo fmt --all -- --check

.PHONY: clippy
clippy: ## Run clippy
	cargo clippy --all -- -D warnings

.PHONY: check
check: fmt-check clippy test ## Run all CI checks

.PHONY: release
release: ## Build release binary
	cargo build --release

.PHONY: install
install: ## Install locally
	cargo install --path .
