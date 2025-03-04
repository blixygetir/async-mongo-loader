use mongodb::bson::Document;
use serde_json;
use std::fs;
use std::path::Path;

// converts the document to a BSON document
// BSON is a binary representation of JSON-like documents
// MongoDB drivers accept BSON file format for data storage
pub fn push_data(path: &Path) -> Vec<Document> {
    let contents = fs::read_to_string(path.to_str().expect("Failed to convert path to string"))
        .expect("Failed to read file");

    let json_array: Vec<Document> = serde_json::from_str(&contents).expect("Failed to parse JSON");
    let mut bson_arr: Vec<Document> = vec![];

    for arr in json_array.iter() {
        bson_arr.push(arr.clone());
    }

    return bson_arr;
}
