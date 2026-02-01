use in_memory_db::KeyValueStore;

#[test]
fn test_basic_set_and_get() {
    let mut store = KeyValueStore::new();
    
    store.set("key1".to_string(), "value1".to_string());
    
    assert_eq!(store.get("key1"), Some(&"value1".to_string()));
}

#[test]
fn test_get_returns_none_for_missing_key() {
    let store = KeyValueStore::new();
    
    assert_eq!(store.get("nonexistent"), None);
}

#[test]
fn test_set_overwrites_existing_value() {
    let mut store = KeyValueStore::new();
    
    store.set("key".to_string(), "original".to_string());
    store.set("key".to_string(), "updated".to_string());
    
    assert_eq!(store.get("key"), Some(&"updated".to_string()));
}

#[test]
fn test_delete_removes_key_and_returns_value() {
    let mut store = KeyValueStore::new();
    
    store.set("key".to_string(), "value".to_string());
    let deleted = store.delete("key");
    
    assert_eq!(deleted, Some("value".to_string()));
    assert_eq!(store.get("key"), None);
}

#[test]
fn test_delete_returns_none_for_missing_key() {
    let mut store = KeyValueStore::new();
    
    let deleted = store.delete("missing");
    
    assert_eq!(deleted, None);
}

#[test]
fn test_len_and_is_empty() {
    let mut store = KeyValueStore::new();
    
    assert!(store.is_empty());
    assert_eq!(store.len(), 0);
    
    store.set("key1".to_string(), "value1".to_string());
    assert!(!store.is_empty());
    assert_eq!(store.len(), 1);
    
    store.set("key2".to_string(), "value2".to_string());
    assert_eq!(store.len(), 2);
    
    store.delete("key1");
    assert_eq!(store.len(), 1);
    
    store.delete("key2");
    assert!(store.is_empty());
}

#[test]
fn test_multiple_keys() {
    let mut store = KeyValueStore::new();
    
    // Insert multiple entries
    store.set("name".to_string(), "Alice".to_string());
    store.set("city".to_string(), "Seattle".to_string());
    store.set("language".to_string(), "Rust".to_string());
    
    assert_eq!(store.len(), 3);
    assert_eq!(store.get("name"), Some(&"Alice".to_string()));
    assert_eq!(store.get("city"), Some(&"Seattle".to_string()));
    assert_eq!(store.get("language"), Some(&"Rust".to_string()));
}

#[test]
fn test_ownership_delete_returns_owned_value() {
    let mut store = KeyValueStore::new();
    
    let original_value = "owned_value".to_string();
    store.set("key".to_string(), original_value.clone());
    
    let deleted = store.delete("key").unwrap();
    
    // We now own the deleted value
    assert_eq!(deleted, original_value);
    assert_eq!(deleted.len(), 11);
    
    // Can use the deleted value however we want
    let uppercase = deleted.to_uppercase();
    assert_eq!(uppercase, "OWNED_VALUE");
}

#[test]
fn test_default_constructor() {
    let store: KeyValueStore = Default::default();
    
    assert!(store.is_empty());
    assert_eq!(store.len(), 0);
}

#[test]
fn test_stress_many_operations() {
    let mut store = KeyValueStore::new();
    
    // Insert 100 entries
    for i in 0..100 {
        store.set(format!("key{}", i), format!("value{}", i));
    }
    
    assert_eq!(store.len(), 100);
    
    // Verify all entries exist
    for i in 0..100 {
        assert_eq!(store.get(&format!("key{}", i)), Some(&format!("value{}", i)));
    }
    
    // Update even-numbered entries
    for i in (0..100).step_by(2) {
        store.set(format!("key{}", i), format!("updated{}", i));
    }
    
    assert_eq!(store.len(), 100);
    
    // Delete odd-numbered entries
    for i in (1..100).step_by(2) {
        let deleted = store.delete(&format!("key{}", i));
        assert_eq!(deleted, Some(format!("value{}", i)));
    }
    
    assert_eq!(store.len(), 50);
    
    // Verify remaining entries are the even ones with updated values
    for i in (0..100).step_by(2) {
        assert_eq!(store.get(&format!("key{}", i)), Some(&format!("updated{}", i)));
    }
}
