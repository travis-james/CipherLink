mod crypto;

use crate::crypto::{encrypt, encrypt_data_to_item, EncryptData};
use aws_sdk_dynamodb::{self as dynamodb, config::Credentials, types::{AttributeDefinition, KeySchemaElement, KeyType,ProvisionedThroughput, ScalarAttributeType}, Client};

#[tokio::main]
async fn main() {
    do_db_stuff().await;
}

async fn do_db_stuff() {
    let client = fireup_db().await;
    check_db(&client).await;
    create_table(&client).await;
    check_db(&client).await;
    let data = encrypt("foo", "bar").expect("failed");
    insert_item(&client, "stuff", "1", &data).await;
}

async fn create_table(client: &Client) {
    client
        .create_table()
        .table_name("stuff")
        .key_schema(
            KeySchemaElement::builder()
                .attribute_name("id")
                .key_type(KeyType::Hash)
                .build()
                .unwrap(),
        )
        .attribute_definitions(
            AttributeDefinition::builder()
                .attribute_name("id")
                .attribute_type(ScalarAttributeType::S)
                .build()
                .unwrap(),
        )
        .provisioned_throughput(
            ProvisionedThroughput::builder()
                .read_capacity_units(5)
                .write_capacity_units(5)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("couldn't create table");

    println!("Table 'stuff' created");
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

async fn check_db(client: &Client) {
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

async fn insert_item(client: &Client, table_name: &str, id: &str, data: &EncryptData) {
    let item = encrypt_data_to_item(id, data);

    let result = client
        .put_item()
        .table_name(table_name)
        .set_item(Some(item))
        .send()
        .await;

    match result {
        Ok(_) => println!("Item inserted successfully"),
        Err(e) => eprintln!("Failed to insert item: {}", e),
    }
}
