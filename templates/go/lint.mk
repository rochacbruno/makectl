## makectl: description="Run Go linters"
.PHONY: lint
lint: ## Run linters
	golangci-lint run ./...

.PHONY: lint-fix
lint-fix: ## Run linters and fix issues
	golangci-lint run --fix ./...
