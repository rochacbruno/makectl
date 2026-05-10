## makectl: description="Run clippy linter"
.PHONY: clippy
clippy: ## Run clippy
	cargo clippy --all -- -D warnings
