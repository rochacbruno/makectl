## makectl: description="Run Python tests with pytest"
PYTHON ?= python3
VENV ?= .venv

.PHONY: test
test: ## Run tests
	$(VENV)/bin/pytest tests/ -v

.PHONY: test-cov
test-cov: ## Run tests with coverage
	$(VENV)/bin/pytest tests/ -v --cov --cov-report=term-missing
