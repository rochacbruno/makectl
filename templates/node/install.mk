## makectl: description="Install Node.js dependencies"
.PHONY: install
install: ## Install dependencies
	npm install

.PHONY: install-ci
install-ci: ## Install dependencies (CI mode)
	npm ci
