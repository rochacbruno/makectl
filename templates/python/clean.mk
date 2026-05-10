## makectl: description="Clean Python cache and build artifacts"
.PHONY: clean
clean: ## Clean Python artifacts
	find . -type f -name '*.pyc' -delete
	find . -type d -name '__pycache__' -exec rm -rf {} + 2>/dev/null || true
	find . -type d -name '*.egg-info' -exec rm -rf {} + 2>/dev/null || true
	rm -rf .cache .pytest_cache .mypy_cache .ruff_cache
	rm -rf build/ dist/ htmlcov/ .tox/ .coverage
