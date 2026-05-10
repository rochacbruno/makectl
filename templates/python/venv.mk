## makectl: description="Create and manage Python virtual environment"
PYTHON ?= python3
VENV ?= .venv

.PHONY: venv
venv: ## Create virtual environment
	$(PYTHON) -m venv $(VENV)
	$(VENV)/bin/pip install --upgrade pip

.PHONY: install
install: venv ## Install dependencies
	$(VENV)/bin/pip install -r requirements.txt

.PHONY: install-dev
install-dev: venv ## Install development dependencies
	$(VENV)/bin/pip install -r requirements-dev.txt
