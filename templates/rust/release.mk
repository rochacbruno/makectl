## makectl: description="Build optimized release binary"
.PHONY: release
release: ## Build release binary
	cargo build --release

.PHONY: install-local
install-local: ## Install binary locally
	cargo install --path .
