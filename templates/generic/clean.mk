## makectl: description="Remove common build artifacts"
.PHONY: clean
clean: ## Remove build artifacts
	rm -rf build/ dist/ out/ tmp/
	find . -name '*.tmp' -delete
	find . -name '*~' -delete
