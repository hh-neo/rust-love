use std::collections::BTreeMap;

// Define the type alias
pub type KVEngine = BTreeMap<Vec<u8>, Option<Vec<u8>>>;

fn main() {
    // Create a new instance of our KVEngine.
    // Rust understands this is just a BTreeMap.
    let mut engine = KVEngine::new();

    // --- 1. Insert (put) a value ---
    // The key "greeting" and the value "Hello, world!" are converted to Vec<u8>
    let key1 = b"greeting".to_vec();
    let value1 = b"Hello, world!".to_vec();
    engine.insert(key1.clone(), Some(value1.clone()));

    println!("Inserted key: {:?}", String::from_utf8(key1).unwrap());
    println!("Inserted value: {:?}", String::from_utf8(value1).unwrap());
    println!("Current map: {:?}", engine);

    // --- 2. Get (retrieve) a value ---
    let retrieved_value = engine.get(&key1);
    match retrieved_value {
        Some(Some(val)) => {
            println!("\nRetrieved value for {:?}: {:?}",
                String::from_utf8(key1).unwrap(),
                String::from_utf8(val.clone()).unwrap()
            );
        },
        Some(None) => println!("\nKey {:?} found, but its value is marked for deletion.", String::from_utf8(key1).unwrap()),
        None => println!("\nKey {:?} not found.", String::from_utf8(key1).unwrap()),
    }

    // --- 3. "Delete" a value ---
    // In this model, deletion is often a special insertion.
    // We insert a `None` value to mark the key for deletion.
    let key2 = b"to_delete".to_vec();
    let value2 = b"This will be deleted".to_vec();
    engine.insert(key2.clone(), Some(value2));
    
    // Now, we "delete" it by inserting `None` for the same key.
    engine.insert(key2.clone(), None);

    println!("\nAfter marking key {:?} for deletion...", String::from_utf8(key2).unwrap());
    println!("Current map: {:?}", engine); // Note that the key is still in the map, but its value is None

    // --- 4. Try to get the deleted value ---
    let deleted_value = engine.get(&key2);
    match deleted_value {
        Some(Some(_)) => println!("This should not happen."),
        Some(None) => println!("Confirmed: Key {:?} is marked for deletion.", String::from_utf8(key2).unwrap()),
        None => println!("Key was not found, which is another way to handle deletions."),
    }

    // In a real application, you would periodically run a "compaction" process
    // that removes keys with `None` values from the map to free up space.
}