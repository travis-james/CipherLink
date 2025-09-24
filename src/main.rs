use std::env;

use crate::{app_config::AppConfig, crypto::encrypt, transformer::encrypt_data_to_item};

mod app_config;
mod crypto;
mod db;
mod handlers;
mod lambda;
mod rest;
mod transformer;
mod types;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let config = app_config::AppConfig::from_env();

    let args: Vec<String> = env::args().collect();
    let mode = if args.len() >= 2 {
        args[1].as_str()
    } else {
        "lambda" // cargo lambda watch won't take args.
    };

    match mode {
        "server" => rest::init(config).await,
        "seed" => seed_db(config).await,
        "lambda" => lambda::init(config).await,
        _ => {
            eprintln!("Unknown mode: '{}'.", mode);
            std::process::exit(1);
        }
    }
}

async fn seed_db(config: AppConfig) {
    println!("Starting 'db' mode, seeding DynamoDB....");

    let db_client = db::init(&config.db_url, &config.region).await;
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
        .insert(table_name, encrypt_data_to_item(id1, &encrypt_data1))
        .await
        .unwrap();

    let id2 = "id2";
    db_client
        .insert(table_name, encrypt_data_to_item(id2, &encrypt_data2))
        .await
        .unwrap();

    println!("items inserted");
    // db_client
    //     .dump_table(table_name)
    //     .await
    //     .expect("unable to dump table")
}
