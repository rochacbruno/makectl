## makectl: description="Format Go code"
.PHONY: fmt
fmt: ## Format code
	go fmt ./...
	go mod tidy
