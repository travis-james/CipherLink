#!/bin/bash

echo "Running seed script..."
sleep 5

aws dynamodb create-table \
  --table-name EncryptedData \
  --attribute-definitions AttributeName=id,AttributeType=S \
  --key-schema AttributeName=id,KeyType=HASH \
  --provisioned-throughput ReadCapacityUnits=5,WriteCapacityUnits=5 \
  --endpoint-url http://localhost:8000 \
  --region us-west-2

aws dynamodb batch-write-item \
  --request-items file://batch_seed.json \
  --endpoint-url http://localhost:8000 \
  --region us-west-2
