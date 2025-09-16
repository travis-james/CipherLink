DYNAMODB_CONTAINER_NAME=dynamodb-local
HOST_PORT=8000
CONTAINER_PORT=8000

run-db:
	docker run -d --rm --name $(DYNAMODB_CONTAINER_NAME) -p $(HOST_PORT):$(CONTAINER_PORT) amazon/dynamodb-local
stop-db:
	docker stop $(DYNAMODB_CONTAINER_NAME)