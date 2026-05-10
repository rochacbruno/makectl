## makectl: description="Run Python linting tools"
VENV ?= .venv

.PHONY: lint
lint: ## Run linters
	$(VENV)/bin/ruff check .
	$(VENV)/bin/mypy .

.PHONY: lint-fix
lint-fix: ## Run linters and fix issues
	$(VENV)/bin/ruff check --fix .
