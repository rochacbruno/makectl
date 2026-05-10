## makectl: description="Run Node.js tests"
.PHONY: test
test: ## Run tests
	npm test

.PHONY: test-watch
test-watch: ## Run tests in watch mode
	npm test -- --watch
