## makectl: description="Run Go tests"
.PHONY: test
test: ## Run tests
	go test -v -race ./...

.PHONY: test-cov
test-cov: ## Run tests with coverage
	go test -v -race -coverprofile=coverage.out ./...
	go tool cover -html=coverage.out -o coverage.html
