## makectl: description="Format Node.js code with Prettier"
.PHONY: format
format: ## Format code
	npx prettier --write .

.PHONY: format-check
format-check: ## Check code formatting
	npx prettier --check .
