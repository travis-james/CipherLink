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

#[derive(Clone)]
pub struct DynamoDBClient {
    client: Client,
}

// "http://localhost:8000"
// "us-west-2"
pub async fn init(url: &str, region: &str) -> DynamoDBClient {
    let creds = Credentials::new("dummy", "dummy", None, None, "dummy");
    let config = aws_config::from_env()
        .endpoint_url(url)
        .region(Region::new(region.to_string()))
        .credentials_provider(creds)
        .load()
        .await;

    DynamoDBClient {
        client: Client::new(&config),
    }
}

impl DynamoDBClient {
    pub async fn init_table(&self, table_name: &str, attribute_name: &str) -> Result<(), Error> {
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

    pub async fn insert_item(
        &self,
        table_name: &str,
        item: HashMap<String, AttributeValue>,
    ) -> Result<(), Error> {
        self.client
            .put_item()
            .table_name(table_name)
            .set_item(Some(item))
            .send()
            .await?;
        Ok(())
    }

    pub async fn get(
        &self,
        table_name: &str,
        key: &str,
        val: &str,
    ) -> Result<HashMap<String, AttributeValue>, String> {
        let response = self
            .client
            .get_item()
            .table_name(table_name)
            .key(key, AttributeValue::S(val.to_string()))
            .send()
            .await
            .map_err(|e| format!("DynamoDB get_item failed: {}", e))?;

        response
            .item
            .ok_or_else(|| format!("Item not found for: {}", val))
    }

    pub async fn delete(&self, table: &str, key: &str, value: &str) -> Result<(), String> {
        self.client
            .delete_item()
            .table_name(table)
            .key(key, AttributeValue::S(value.into()))
            .send()
            .await
            .map_err(|e| format!("Failed to delete item: {}", e))?;

        println!("Deleted item from table");
        Ok(())
    }

    pub async fn check_db(&self) -> Result<(), Error> {
        self.client.list_tables().send().await?;
        Ok(())
    }

    pub async fn dump_table(&self, table_name: &str) -> Result<(), Error> {
        let resp = self.client.scan().table_name(table_name).send().await?;
        for item in resp.items() {
            println!("{:?}", item)
        }
        Ok(())
    }
}
