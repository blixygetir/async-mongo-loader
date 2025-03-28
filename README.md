# Async Mongo Loader
A high-performance API built with Rust, leveraging Tokio's async runtime to efficiently upload multiple files concurrently into MongoDB. This API processes files in parallel, ensuring fast and scalable data ingestion

# Features
1. Concurrent file ingestion using Tokio  
2. Asynchronous MongoDB integration  
3. Fast and scalable JSON processing  
4. Error handling and logging  


## Installation 
1. Clone the repository:
   ```sh
   git clone https://github.com/blixygetir/async-mongo-loader.git
   cd async-mongo-loader
2. Enter the connection string of your MongoDB database in the .env file.
3.  Install the dependencies.
      cargo build
4. Run the API
   cargo run <database_name> <collection_name>

## Requirements

### 1. Rust Installation
- Install the **latest Rust version** (currently `rustc 1.76.0` or later):
  ```sh
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
### 2. MongoDB Setup 
- Install MongoDB locally or use MongoDB Atlas (Sign up).
- Ensure MongoDB is running and set the MONGO_URI in .env
