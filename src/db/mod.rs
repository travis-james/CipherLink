use std::collections::HashMap;

use aws_config::Region;
use aws_sdk_dynamodb::{
    Client, Error,
    config::Credentials,
    types::{
        AttributeDefinition, AttributeValue, KeySchemaElement, KeyType, ProvisionedThroughput,
        ScalarAttributeType,
    },
};

struct DynamoDBClient {
    client: Client,
}

// "http://localhost:8000"
// "us-west-2"
pub async fn init(url: &str, region: &str) -> Client {
    let creds = Credentials::new("dummy", "dummy", None, None, "dummy");
    let config = aws_config::from_env()
        .endpoint_url(url)
        .region(Region::new(region.to_string()))
        .credentials_provider(creds)
        .load()
        .await;
    Client::new(&config)
}

impl DynamoDBClient {
    async fn create_table(&self, table_name: &str, attribute_name: &str) -> Result<(), Error> {
        self.client
            .create_table()
            .table_name(table_name)
            .key_schema(
                KeySchemaElement::builder()
                    .attribute_name(attribute_name)
                    .key_type(KeyType::Hash)
                    .build()
                    .unwrap(),
            )
            .attribute_definitions(
                AttributeDefinition::builder()
                    .attribute_name(attribute_name)
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
            .await?;
        Ok(())
    }

    async fn insert_item(
        &self,
        table_name: &str,
        item: HashMap<String, AttributeValue>,
    ) -> Result<(), Error> {
        //let item = encrypt_data_to_item(id, data);
        self.client
            .put_item()
            .table_name(table_name)
            .set_item(Some(item))
            .send()
            .await?;
        Ok(())
    }

    pub async fn check_db(&self) -> Result<(), Error> {
        self.client.list_tables().send().await?;
        Ok(())
    }
}

// async fn do_db_stuff() {
//     let client = fireup_db().await;
//     check_db(&client).await;
//     create_table(&client).await;
//     check_db(&client).await;
//     let data = encrypt("foo", "bar").expect("failed");
//     insert_item(&client, "stuff", "1", &data).await;
// }
