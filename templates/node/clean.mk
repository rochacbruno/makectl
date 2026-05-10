## makectl: description="Clean Node.js build artifacts"
.PHONY: clean
clean: ## Clean build artifacts
	rm -rf node_modules/ dist/ build/ .cache/ coverage/
