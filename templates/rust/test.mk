## makectl: description="Run Rust tests"
.PHONY: test
test: ## Run tests
	cargo test

.PHONY: test-verbose
test-verbose: ## Run tests with verbose output
	cargo test --verbose
