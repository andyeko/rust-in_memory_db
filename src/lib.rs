//! # In-Memory Key-Value Store
//! 
//! A simple in-memory key-value store implementation designed to demonstrate
//! Rust's ownership and move semantics.
//! 
//! ## Features
//! 
//! - `GET`: Retrieve a value by key
//! - `SET`: Insert or update a key-value pair
//! - `DELETE`: Remove a key-value pair
//! 
//! ## Example
//! 
//! ```
//! use in_memory_db::KeyValueStore;
//! 
//! let mut store = KeyValueStore::new();
//! 
//! // SET: Store takes ownership of the strings
//! store.set("name".to_string(), "Alice".to_string());
//! 
//! // GET: Returns a borrowed reference
//! if let Some(name) = store.get("name") {
//!     println!("Name: {}", name);
//! }
//! 
//! // DELETE: Returns owned value back to caller
//! let old_name = store.delete("name");
//! assert_eq!(old_name, Some("Alice".to_string()));
//! ```

mod store;

pub use store::KeyValueStore;
