## makectl: description="Format Python code"
VENV ?= .venv

.PHONY: format
format: ## Format code
	$(VENV)/bin/ruff format .

.PHONY: format-check
format-check: ## Check code formatting
	$(VENV)/bin/ruff format --check .
