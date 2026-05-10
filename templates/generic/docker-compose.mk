## makectl: description="Docker Compose management targets"
COMPOSE_FILE ?= docker-compose.yml

.PHONY: compose-up
compose-up: ## Start services with Docker Compose
	docker compose -f $(COMPOSE_FILE) up -d

.PHONY: compose-down
compose-down: ## Stop services
	docker compose -f $(COMPOSE_FILE) down

.PHONY: compose-logs
compose-logs: ## Show service logs
	docker compose -f $(COMPOSE_FILE) logs -f

.PHONY: compose-restart
compose-restart: ## Restart services
	docker compose -f $(COMPOSE_FILE) restart

.PHONY: compose-ps
compose-ps: ## List running services
	docker compose -f $(COMPOSE_FILE) ps
