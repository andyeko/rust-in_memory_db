use in_memory_db::KeyValueStore;

fn main() {
    println!("🗄️  In-Memory Key-Value Store Demo\n");
    println!("Demonstrating Rust Ownership & Move Semantics\n");
    println!("{}", "=".repeat(50));
    
    let mut store = KeyValueStore::new();
    
    // Example 1: Basic SET and GET
    println!("\n📝 Example 1: Basic Operations");
    println!("{}", "-".repeat(50));
    
    let name = "Alice".to_string();
    let city = "Seattle".to_string();
    
    println!("Setting: name = {}, city = {}", name, city);
    store.set("name".to_string(), name);
    store.set("city".to_string(), city);
    // Note: name and city are moved into the store - can't use them anymore!
    
    if let Some(stored_name) = store.get("name") {
        println!("Retrieved name: {}", stored_name);
        // stored_name is a borrowed reference - we can read but don't own it
    }
    
    if let Some(stored_city) = store.get("city") {
        println!("Retrieved city: {}", stored_city);
    }
    
    println!("Store size: {}", store.len());
    
    // Example 2: UPDATE (overwriting existing key)
    println!("\n🔄 Example 2: Updating Values");
    println!("{}", "-".repeat(50));
    
    store.set("city".to_string(), "Portland".to_string());
    println!("Updated city to: {:?}", store.get("city"));
    
    // Example 3: DELETE (ownership returns to caller)
    println!("\n🗑️  Example 3: Deleting Values");
    println!("{}", "-".repeat(50));
    
    if let Some(deleted_city) = store.delete("city") {
        println!("Deleted city: {}", deleted_city);
        // We now OWN deleted_city - the store no longer has it
        println!("Deleted value length: {}", deleted_city.len());
    }
    
    println!("City after delete: {:?}", store.get("city"));
    println!("Store size: {}", store.len());
    
    // Example 4: Multiple entries
    println!("\n📊 Example 4: Multiple Entries");
    println!("{}", "-".repeat(50));
    
    store.set("language".to_string(), "Rust".to_string());
    store.set("year".to_string(), "2026".to_string());
    store.set("level".to_string(), "Senior".to_string());
    
    println!("Store now contains {} entries:", store.len());
    
    // Example 5: Demonstrating borrowing rules
    println!("\n🔒 Example 5: Borrowing Rules");
    println!("{}", "-".repeat(50));
    
    // This works: we can have multiple immutable borrows
    let lang1 = store.get("language");
    let lang2 = store.get("language");
    println!("Borrow 1: {:?}", lang1);
    println!("Borrow 2: {:?}", lang2);
    
    // After borrows go out of scope, we can mutate again
    store.set("language".to_string(), "Rust 2021".to_string());
    println!("Updated language: {:?}", store.get("language"));
    
    // Example 6: What happens with a missing key
    println!("\n❓ Example 6: Missing Keys");
    println!("{}", "-".repeat(50));
    
    match store.get("missing_key") {
        Some(value) => println!("Found: {}", value),
        None => println!("Key 'missing_key' not found (returns None)"),
    }
    
    let deleted = store.delete("nonexistent");
    println!("Deleting nonexistent key returns: {:?}", deleted);
    
    // Final state
    println!("\n✅ Final Store State");
    println!("{}", "-".repeat(50));
    println!("Total entries: {}", store.len());
    println!("Is empty? {}", store.is_empty());
    
    println!("\n{}", "=".repeat(50));
    println!("🎓 Key Takeaways:");
    println!("  • SET moves ownership to the store");
    println!("  • GET borrows a reference (&String)");
    println!("  • DELETE moves ownership back to caller");
    println!("  • Borrow checker prevents use-after-free bugs!");
}
