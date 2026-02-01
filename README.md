# In-Memory Key-Value Store in Rust

A simple yet instructive in-memory key-value store implementation in Rust, designed to explore ownership, borrowing, and move semantics.

## 🎯 Learning Objectives

This project is specifically designed for senior developers learning Rust to tackle:

### Ownership & Move Semantics
- Understanding how Rust's ownership model prevents data races
- Learning when values are moved vs borrowed
- Implementing data structures that manage owned data

### Borrowing & Lifetimes
- Distinguishing between `&T` (shared reference) and `&mut T` (mutable reference)
- Understanding the borrow checker's rules
- Returning references to data stored in collections

### Standard Library Collections
- Working with `HashMap<K, V>` for key-value storage
- Using `Option<T>` for nullable values
- Handling `Result<T, E>` for error cases

## 📋 Features

| Operation | Description | Time Complexity |
|-----------|-------------|-----------------|
| `GET`     | Retrieve a value by key | O(1) average |
| `SET`     | Insert or update a key-value pair | O(1) average |
| `DELETE`  | Remove a key-value pair | O(1) average |

## 🏗️ Project Structure

```
in_memory_db/
├── Cargo.toml          # Project dependencies and metadata
├── src/
│   ├── main.rs         # Entry point with CLI or examples
│   ├── lib.rs          # Library root (exposes modules)
│   └── store.rs        # Core KeyValueStore implementation
├── tests/
│   └── integration.rs  # Integration tests
└── README.md           # This file
```

## 🔧 Implementation Details

### Core Data Structure

```rust
use std::collections::HashMap;

pub struct KeyValueStore {
    data: HashMap<String, String>,
}
```

### API Design

```rust
impl KeyValueStore {
    /// Creates a new empty store
    pub fn new() -> Self;
    
    /// Retrieves a value by key
    /// Returns Option<&String> - demonstrates borrowing
    pub fn get(&self, key: &str) -> Option<&String>;
    
    /// Inserts or updates a key-value pair
    /// Takes ownership of both key and value
    pub fn set(&mut self, key: String, value: String);
    
    /// Removes a key-value pair
    /// Returns the owned value if it existed
    pub fn delete(&mut self, key: &str) -> Option<String>;
}
```

### Key Ownership Patterns Demonstrated

| Method | `self` | `key` | `value` | Why? |
|--------|--------|-------|---------|------|
| `get` | `&self` | `&str` | - | Only reading, no mutation needed |
| `set` | `&mut self` | `String` (owned) | `String` (owned) | Store takes ownership of data |
| `delete` | `&mut self` | `&str` | - | Returns owned value to caller |

## 🚀 Getting Started

### Prerequisites
- Rust 1.93.0+ (install via [rustup](https://rustup.rs/))

### Build & Run

```bash
# Build the project
cargo build

# Run tests
cargo test

# Run the example/CLI
cargo run
```

## 📝 Example Usage

```rust
use in_memory_db::KeyValueStore;

fn main() {
    let mut store = KeyValueStore::new();
    
    // SET: ownership of strings is transferred to the store
    store.set("name".to_string(), "Alice".to_string());
    store.set("city".to_string(), "Seattle".to_string());
    
    // GET: returns a borrowed reference (&String)
    if let Some(name) = store.get("name") {
        println!("Name: {}", name);  // Can read the borrowed value
    }
    
    // DELETE: returns owned value back to caller
    if let Some(old_city) = store.delete("city") {
        println!("Removed city: {}", old_city);  // We own this now
    }
    
    // Key no longer exists
    assert!(store.get("city").is_none());
}
```

## 🧠 Ownership Challenges to Explore

### Challenge 1: Why can't `get` return `String` instead of `&String`?
If we returned `String`, we'd have to clone the data or move it out of the HashMap, which would remove it!

### Challenge 2: Why does `set` take `String` instead of `&str`?
The store needs to own the data. Taking `&str` would require cloning inside the method anyway, so we let the caller decide when to allocate.

### Challenge 3: What happens if you call `get` after `delete`?
```rust
let value = store.get("key");  // Returns Option<&String>
store.delete("key");           // ERROR! Can't mutate while borrowed
println!("{:?}", value);       // Would use invalid reference
```
The borrow checker prevents this at compile time!

## 🔄 Extension Ideas

1. **Generic Types**: Make the store generic over `K: Hash + Eq` and `V`
2. **TTL Support**: Add expiration times to entries
3. **Persistence**: Save/load from disk using `serde`
4. **Thread Safety**: Wrap in `Arc<RwLock<_>>` for concurrent access
5. **LRU Eviction**: Implement a capacity limit with LRU cache policy

## 📚 Resources

- [The Rust Book - Ownership](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)
- [Rust by Example - HashMap](https://doc.rust-lang.org/rust-by-example/std/hash.html)
- [std::collections::HashMap](https://doc.rust-lang.org/std/collections/struct.HashMap.html)

## License

MIT
