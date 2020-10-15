use unqlite::{UnQLite, KV, Cursor};

fn main() {
    // The database memory is not handled by Rust, and the database is on-disk,
    // so `mut` is not neccessary.
    let unqlite = UnQLite::create_temp();
    // Use any type that can use as `[u8]`
    unqlite.kv_store("key", "a long length value").unwrap();
    unqlite.kv_store("abc", [1,2,3]).unwrap();

    let mut entry = unqlite.first();
    // Iterate records
    loop {
        if entry.is_none() { break; }

        let record = entry.expect("valid entry");
        let (key, value) = record.key_value();
        println!("* Go through {:?} --> {:?}", key, value);

        if value.len() > 10 {
            println!("** Delete key {:?} by value length", key);
            entry = record.delete();
        } else {
            entry = record.next();
        }
    }
    
}