## makectl: description="Git hooks installation targets"
.PHONY: hooks-install
hooks-install: ## Install git hooks
	@echo "Installing git hooks..."
	@mkdir -p .git/hooks
	@for hook in .githooks/*; do \
		ln -sf ../../$$hook .git/hooks/$$(basename $$hook); \
	done
	@echo "Git hooks installed."

.PHONY: hooks-uninstall
hooks-uninstall: ## Remove git hooks
	@rm -f .git/hooks/pre-commit .git/hooks/pre-push .git/hooks/commit-msg
	@echo "Git hooks removed."
