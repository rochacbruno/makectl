## makectl: description="Build and publish Python package to PyPI"
VENV ?= .venv

.PHONY: build-pkg
build-pkg: clean ## Build package
	$(VENV)/bin/python -m build

.PHONY: publish
publish: build-pkg ## Publish to PyPI
	$(VENV)/bin/twine upload dist/*

.PHONY: publish-test
publish-test: build-pkg ## Publish to Test PyPI
	$(VENV)/bin/twine upload --repository testpypi dist/*
