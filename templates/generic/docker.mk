## makectl: description="Docker build, run, and push targets"
IMAGE_NAME ?= $(notdir $(CURDIR))
IMAGE_TAG ?= latest

.PHONY: docker-build
docker-build: ## Build Docker image
	docker build -t $(IMAGE_NAME):$(IMAGE_TAG) .

.PHONY: docker-run
docker-run: ## Run Docker container
	docker run --rm -it $(IMAGE_NAME):$(IMAGE_TAG)

.PHONY: docker-push
docker-push: ## Push Docker image to registry
	docker push $(IMAGE_NAME):$(IMAGE_TAG)

.PHONY: docker-clean
docker-clean: ## Remove Docker image
	docker rmi $(IMAGE_NAME):$(IMAGE_TAG) || true
