## makectl: description="Generate and open Rust documentation"
.PHONY: doc
doc: ## Generate documentation
	cargo doc --no-deps

.PHONY: doc-open
doc-open: ## Generate and open documentation
	cargo doc --no-deps --open
