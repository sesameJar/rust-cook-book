#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate lazy_static;

use std::sync::Mutex;
error_chain!{ }

lazy_static! {
    static ref FRUIT: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

fn insert(fruit : &str) -> Result<()> {
    let mut db = FRUIT.lock().map_err(|_| "Failed to acquire MutexGuard")?;
    db.push(fruit.to_string());
    Ok(())
}

fn main() -> Result<()> {
    insert("Apple")?;
    insert("orange")?;
    insert("peach")?;
    {
        let db = FRUIT.lock().map_err(|_| "Failed to acuire MutexGaurd")?;
        db.iter().enumerate().for_each(|(i, item)| println!("{}: {}", i, item));
    }
    insert("GRAPE")?;
    Ok(())
}