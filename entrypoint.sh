#!/bin/bash

echo "Running entry point script..."
/app/seed.sh

echo "Starting DynamoDB Local..."
exec java -Djava.library.path=/home/dynamodblocal/DynamoDBLocal_lib \
     -jar /home/dynamodblocal/DynamoDBLocal.jar \
     -sharedDb -inMemory
