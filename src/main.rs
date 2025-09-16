mod crypto;

use aws_sdk_dynamodb::{self as dynamodb, Client, config::Credentials};

#[tokio::main]
async fn main() {
    do_db_stuff().await;
}

async fn do_db_stuff() {
    let client = fireup_db().await;
    check_db(client).await;
}

async fn fireup_db() -> Client {
    let creds = Credentials::new("dummy", "dummy", None, None, "dummy");
    let config = aws_config::from_env()
        .endpoint_url("http://localhost:8000")
        .region("us-west-2")
        .credentials_provider(creds)
        .load()
        .await;
    Client::new(&config)
}

async fn check_db(client: Client) {
    let resp = client
        .list_tables()
        .send()
        .await
        .expect("couldn't list tables");

    println!("Tables:");

    let names = resp.table_names();

    for name in names {
        println!("  {}", name);
    }

    println!();
    println!("Found {} tables", names.len());
}
