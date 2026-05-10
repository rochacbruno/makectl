## makectl: description="Run Node.js linters"
.PHONY: lint
lint: ## Run linters
	npx eslint .

.PHONY: lint-fix
lint-fix: ## Run linters and fix issues
	npx eslint --fix .
