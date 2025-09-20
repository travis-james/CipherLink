use std::env;

use crate::{
    crypto::encrypt,
    transformer::encrypt_data_to_item,
};

mod crypto;
pub mod db;
mod transformer;
mod server;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("must specify mode: 'db'");
        std::process::exit(1);
    }

    let mode = args[1].as_str();
    match mode {
        "server" => server::init().await,
        "db" => seed_db().await,
        _ => {
            eprintln!("Unknown mode: '{}'. Use 'db'.", mode);
            std::process::exit(1);
        }
    }
}

async fn seed_db() {
    println!("Starting 'db' mode, seeding DynamoDB....");
    let url = "http://localhost:8000";
    let region = "us-west-2";
    let db_client = db::init(url, region).await;
    let table_name = "encryptData";
    let attribute_name = "id";
    db_client
        .init_table(table_name, attribute_name)
        .await
        .expect("unable to initialize db");

    let plain_text1 = "google.com";
    let key1: &'static str = "key1";
    let encrypt_data1 = encrypt(plain_text1, key1).unwrap();

    let plain_text2 = "amazon.co.jp";
    let key2 = "key2";
    let encrypt_data2 = encrypt(plain_text2, key2).unwrap();

    let id1 = "id1";
    db_client
        .insert_item(table_name, encrypt_data_to_item(id1, &encrypt_data1))
        .await
        .unwrap();

    let id2 = "id2";
    db_client
        .insert_item(table_name, encrypt_data_to_item(id2, &encrypt_data2))
        .await
        .unwrap();

    println!("items inserted");
    // db_client
    //     .dump_table(table_name)
    //     .await
    //     .expect("unable to dump table")
}
