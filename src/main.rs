mod fs_utils;
use fs_utils::{read_dir, read_json};

use dotenv::dotenv;
use futures::lock::Mutex;
use mongodb::bson::Document;
use mongodb::{Client, Collection};
use std::collections::BTreeMap;
use std::env;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    dotenv().ok();
    let uri = env::var("MONGO_URI").expect("Failed to get MONGO_URI from .env file");
    let client = Client::with_uri_str(uri).await?;

    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <DB_NAME> <COLLECTION_NAME>", args[0]);
        std::process::exit(1);
    }

    let db_name = &args[1];
    let collection_name = &args[2];

    let collection: Collection<Document> = client.database(db_name).collection(collection_name);
    let thread_collection = Arc::new(Mutex::new(collection));

    let path: &str = "data";
    let files_map: BTreeMap<u64, PathBuf>;

    match read_dir::visit_dirs(Path::new(path), &|_entry| {}) {
        Ok(f_map) => files_map = f_map,
        Err(e) => panic!("Failed to read directory: {}", e),
    }

    let mut handles = vec![];

    for (_size, file_path) in files_map.iter() {
        let collection = Arc::clone(&thread_collection);
        let file_path = file_path.clone();

        let handle = tokio::spawn(async move {
            let json_data = read_json::push_data(&file_path);
            if !json_data.is_empty() {
                let col = collection.lock().await;
                col.insert_many(json_data)
                    .await
                    .expect("Failed to insert data");
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.await.expect("Failed to spawn thread");
    }

    Ok(())
}
