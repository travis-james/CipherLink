mod crypto;

use aws_sdk_dynamodb as dynamodb;


async fn main() {
    let config = aws_config::from_env()
    .endpoint_url("http://localhost:8000")
    .region("us-west-2")
    .load()
    .await;
}