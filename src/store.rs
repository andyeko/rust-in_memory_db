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

    // This test intentionally uses unsafe raw-pointer aliasing to demonstrate
    // what can go wrong when a borrowed reference is read while another
    // thread concurrently deletes the value. The behavior is undefined and
    // the test is ignored by default to avoid crashing CI or local runs.
    #[test]
    #[ignore]
    fn unsafe_concurrent_get_and_delete() {
        use std::thread;
        use std::time::Duration;

        // Allocate the store on the heap and get a raw pointer so we can
        // dereference it unsafely in multiple threads.
        let mut boxed = Box::new(KeyValueStore::new());
        boxed.set("k".to_string(), "v".to_string());
        let ptr: *mut KeyValueStore = Box::into_raw(boxed);

        // Use the pointer's integer address (usize) to move a Send value into
        // the spawned threads. Raw pointers are not considered `Send` here,
        // but `usize` is Send, so cast and cast back inside the thread.
        let addr = ptr as usize;

        // t1: take a borrowed reference via `get` and convert it to a raw
        // pointer; sleep so t2 can delete the key before t1 reads via the
        // raw pointer (this read is UB if the value has been removed).
        let p1 = addr;
        let t1 = thread::spawn(move || {
            unsafe {
                let p1: *mut KeyValueStore = p1 as *mut KeyValueStore;
                let kv_ref: &KeyValueStore = &*p1;
                let val_ref: &String = kv_ref.get("k").unwrap();
                let val_ptr: *const String = val_ref as *const String;

                // Allow t2 to run and delete the entry.
                thread::sleep(Duration::from_millis(100));

                // UB: dereferencing `val_ptr` after the entry may have been
                // removed by t2. This demonstrates the kind of invalid
                // memory access that can occur without proper synchronization.
                (&*val_ptr).clone()
            }
        });

        // t2: delete the key while t1 is still holding the borrowed pointer.
        let p2 = addr;
        let t2 = thread::spawn(move || {
            unsafe {
                // Small sleep so t1 obtains the raw pointer first.
                thread::sleep(Duration::from_millis(50));
                let p2: *mut KeyValueStore = p2 as *mut KeyValueStore;
                let kv_mut: &mut KeyValueStore = &mut *p2;
                kv_mut.delete("k");
            }
        });

        let v = t1.join().unwrap();
        t2.join().unwrap();

        // Reconstruct the Box to properly drop the store and avoid leaking.
        unsafe { Box::from_raw(ptr); }

        // The value may or may not be "v" depending on timing and UB; this
        // assert is present to show what a successful (non-crashing) run
        // might look like — but running this test is unsafe by design.
        assert_eq!(v, "v".to_string());
    }

    #[test]
    fn safe_concurrent_clone_and_delete() {
        use std::sync::{Arc, RwLock};
        use std::thread;
        use std::time::Duration;

        // Prepare a store with an entry and wrap it for shared access.
        let mut s = KeyValueStore::new();
        s.set("k".to_string(), "v".to_string());
        let store = Arc::new(RwLock::new(s));

        // Reader: take a read lock, clone the value (owned), then sleep.
        let r = store.clone();
        let reader = thread::spawn(move || {
            let val_opt = {
                let guard = r.read().unwrap();
                guard.get("k").cloned() // Option<String>
            }; // guard dropped here

            // Simulate work after obtaining the owned value.
            thread::sleep(Duration::from_millis(100));
            val_opt
        });

        // Writer: wait a bit so reader clones first, then delete the key.
        let w = store.clone();
        let writer = thread::spawn(move || {
            thread::sleep(Duration::from_millis(50));
            let mut guard = w.write().unwrap();
            guard.delete("k");
        });

        let read_value = reader.join().unwrap();
        writer.join().unwrap();

        // Reader must have received the owned value even though writer deleted it later.
        assert_eq!(read_value, Some("v".to_string()));

        // Store should no longer contain the key.
        let final_state = store.read().unwrap();
        assert_eq!(final_state.get("k"), None);
    }
}
