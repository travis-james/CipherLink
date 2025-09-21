DYNAMODB_CONTAINER_NAME=dynamodb-local
HOST_PORT=8000
CONTAINER_PORT=8000
IMAGE_TO_USE=amazon/dynamodb-local

run-db:
	docker run -d --name $(DYNAMODB_CONTAINER_NAME) -p $(HOST_PORT):$(CONTAINER_PORT) $(IMAGE_TO_USE)
stop-db:
	docker stop $(DYNAMODB_CONTAINER_NAME)
	docker rm $(DYNAMODB_CONTAINER_NAME)

seed:
	cargo run -- seed
server:
	cargo run -- server