mod crypto;

use aws_sdk_dynamodb::{self as dynamodb, config::Credentials, Client};

#[tokio::main]
async fn main() {
    let creds = Credentials::new("dummy", "dummy", None, None, "dummy");
    let config = aws_config::from_env()
        .endpoint_url("http://localhost:8000")
        .region("us-west-2")
        .credentials_provider(creds)
        .load()
        .await;
    let client = Client::new(&config);
    let resp = client.list_tables()
    .send().await.expect("couldn't list tables");

    println!("Tables:");

    let names = resp.table_names();

    for name in names {
        println!("  {}", name);
    }

    println!();
    println!("Found {} tables", names.len());
}