DYNAMODB_CONTAINER_NAME=dynamodb-local
HOST_PORT=8000
CONTAINER_PORT=8000
IMAGE_TO_USE=amazon/dynamodb-local:3.1.0

run-db:
	docker run -d --name $(DYNAMODB_CONTAINER_NAME) -p $(HOST_PORT):$(CONTAINER_PORT) $(IMAGE_TO_USE)
stop-db:
	docker stop $(DYNAMODB_CONTAINER_NAME)
	docker rm $(DYNAMODB_CONTAINER_NAME)
seed:
	cargo run -- seed

server:
	cargo run -- server

lambda:
	cargo lambda watch
health:
	echo '{"httpMethod":"GET","path":"/health"}' > lambda_event.json
	cargo lambda invoke --data-file lambda_event.json | jq
	rm lambda_event.json
encrypt:
	echo '{"httpMethod":"POST","path":"/encrypt","body":"{\"key\":\"music\",\"plain_text\":\"http://yahoo.com\"}"}' > lambda_event.json
	cargo lambda invoke --data-file lambda_event.json | jq
	rm lambda_event.json
decrypt:
	echo '{"httpMethod":"GET","path":"/decrypt/a83feb6f-540e-43ef-b44d-845f55be0406/music"}' > lambda_event.json
	cargo lambda invoke --data-file lambda_event.json | jq
	rm lambda_event.json

test-rest:
	./tests/rest_integration_tests.sh
test-lambda:
	./tests/lambda_integration_tests.sh