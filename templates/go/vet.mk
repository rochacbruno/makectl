## makectl: description="Run Go vet static analysis"
.PHONY: vet
vet: ## Run go vet
	go vet ./...
