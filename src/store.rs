use std::collections::HashMap;

/// An in-memory key-value store demonstrating Rust's ownership and move semantics.
/// 
/// This implementation uses String keys and values to highlight ownership patterns:
/// - Keys and values are owned by the store
/// - Methods carefully balance ownership vs borrowing
/// - The borrow checker prevents common bugs at compile time
#[derive(Debug)]
pub struct KeyValueStore {
    data: HashMap<String, String>,
}

impl KeyValueStore {
    /// Creates a new empty key-value store.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use in_memory_db::KeyValueStore;
    /// 
    /// let store = KeyValueStore::new();
    /// ```
    pub fn new() -> Self {
        KeyValueStore {
            data: HashMap::new(),
        }
    }

    /// Retrieves a value by key.
    /// 
    /// Returns a reference to the value if the key exists, or `None` if it doesn't.
    /// 
    /// # Ownership Notes
    /// - Takes `&self`: Only needs to read, doesn't modify
    /// - Takes `&str`: Doesn't need to own the key, just compare it
    /// - Returns `Option<&String>`: Borrows the value from the store
    /// 
    /// # Examples
    /// 
    /// ```
    /// use in_memory_db::KeyValueStore;
    /// 
    /// let mut store = KeyValueStore::new();
    /// store.set("name".to_string(), "Alice".to_string());
    /// 
    /// assert_eq!(store.get("name"), Some(&"Alice".to_string()));
    /// assert_eq!(store.get("missing"), None);
    /// ```
    pub fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }

    /// Inserts or updates a key-value pair.
    /// 
    /// If the key already exists, the old value is replaced.
    /// 
    /// # Ownership Notes
    /// - Takes `&mut self`: Needs to modify the internal HashMap
    /// - Takes `String` (owned): Store takes ownership of both key and value
    /// - Why not `&str`? The store needs to own the data for its lifetime
    /// 
    /// # Examples
    /// 
    /// ```
    /// use in_memory_db::KeyValueStore;
    /// 
    /// let mut store = KeyValueStore::new();
    /// store.set("name".to_string(), "Alice".to_string());
    /// store.set("name".to_string(), "Bob".to_string());  // Updates
    /// 
    /// assert_eq!(store.get("name"), Some(&"Bob".to_string()));
    /// ```
    pub fn set(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }

    /// Removes a key-value pair from the store.
    /// 
    /// Returns the owned value if the key existed, or `None` if it didn't.
    /// 
    /// # Ownership Notes
    /// - Takes `&mut self`: Needs to modify the HashMap
    /// - Takes `&str`: Only needs to find the key, not own it
    /// - Returns `Option<String>`: Gives ownership of the value back to the caller
    /// 
    /// # Examples
    /// 
    /// ```
    /// use in_memory_db::KeyValueStore;
    /// 
    /// let mut store = KeyValueStore::new();
    /// store.set("city".to_string(), "Seattle".to_string());
    /// 
    /// let old_value = store.delete("city");
    /// assert_eq!(old_value, Some("Seattle".to_string()));
    /// assert_eq!(store.get("city"), None);
    /// ```
    pub fn delete(&mut self, key: &str) -> Option<String> {
        self.data.remove(key)
    }

    /// Returns the number of key-value pairs in the store.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use in_memory_db::KeyValueStore;
    /// 
    /// let mut store = KeyValueStore::new();
    /// assert_eq!(store.len(), 0);
    /// 
    /// store.set("key".to_string(), "value".to_string());
    /// assert_eq!(store.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns `true` if the store contains no key-value pairs.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use in_memory_db::KeyValueStore;
    /// 
    /// let store = KeyValueStore::new();
    /// assert!(store.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

impl Default for KeyValueStore {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_store_is_empty() {
        let store = KeyValueStore::new();
        assert!(store.is_empty());
        assert_eq!(store.len(), 0);
    }

    #[test]
    fn test_set_and_get() {
        let mut store = KeyValueStore::new();
        store.set("name".to_string(), "Alice".to_string());
        
        assert_eq!(store.get("name"), Some(&"Alice".to_string()));
        assert_eq!(store.len(), 1);
    }

    #[test]
    fn test_get_nonexistent_key() {
        let store = KeyValueStore::new();
        assert_eq!(store.get("missing"), None);
    }

    #[test]
    fn test_set_updates_existing_key() {
        let mut store = KeyValueStore::new();
        store.set("name".to_string(), "Alice".to_string());
        store.set("name".to_string(), "Bob".to_string());
        
        assert_eq!(store.get("name"), Some(&"Bob".to_string()));
        assert_eq!(store.len(), 1);
    }

    #[test]
    fn test_delete_existing_key() {
        let mut store = KeyValueStore::new();
        store.set("city".to_string(), "Seattle".to_string());
        
        let deleted = store.delete("city");
        assert_eq!(deleted, Some("Seattle".to_string()));
        assert_eq!(store.get("city"), None);
        assert!(store.is_empty());
    }

    #[test]
    fn test_delete_nonexistent_key() {
        let mut store = KeyValueStore::new();
        let deleted = store.delete("missing");
        assert_eq!(deleted, None);
    }

    #[test]
    fn test_multiple_operations() {
        let mut store = KeyValueStore::new();
        
        // Insert multiple entries
        store.set("name".to_string(), "Alice".to_string());
        store.set("city".to_string(), "Seattle".to_string());
        store.set("country".to_string(), "USA".to_string());
        assert_eq!(store.len(), 3);
        
        // Update one
        store.set("city".to_string(), "Portland".to_string());
        assert_eq!(store.len(), 3);
        
        // Delete one
        store.delete("country");
        assert_eq!(store.len(), 2);
        
        // Verify remaining
        assert_eq!(store.get("name"), Some(&"Alice".to_string()));
        assert_eq!(store.get("city"), Some(&"Portland".to_string()));
        assert_eq!(store.get("country"), None);
    }
}
