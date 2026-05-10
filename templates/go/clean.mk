## makectl: description="Clean Go build artifacts"
BINARY_NAME ?= $(notdir $(CURDIR))

.PHONY: clean
clean: ## Clean build artifacts
	go clean
	rm -f $(BINARY_NAME) $(BINARY_NAME)-*
	rm -f coverage.out coverage.html
