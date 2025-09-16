# Stage 1: Seeder with AWS CLI
FROM amazonlinux:2 AS seeder

RUN yum install -y unzip curl && \
    curl "https://awscli.amazonaws.com/awscli-exe-linux-x86_64.zip" -o "awscliv2.zip" && \
    unzip awscliv2.zip && \
    ./aws/install

WORKDIR /app
COPY seed.sh seed.sh
COPY batch_seed.json batch_seed.json

# Stage 2: Runtime with DynamoDB Local
FROM amazon/dynamodb-local

WORKDIR /app

# Copy AWS CLI binary from seeder stage
COPY --from=seeder /usr/local/bin/aws /usr/local/bin/aws
COPY --from=seeder /usr/local/aws-cli /usr/local/aws-cli

# Copy seed script and data
COPY --from=seeder /app/seed.sh seed.sh
COPY --from=seeder /app/batch_seed.json batch_seed.json
COPY entrypoint.sh entrypoint.sh

ENTRYPOINT ["/app/entrypoint.sh"]
