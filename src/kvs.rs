use std::collections::HashMap;

/// # A implementatoin of Key Value Store

pub struct KvStore {
    store: HashMap<String, String>,
}

impl KvStore {
    /// # Creation of Store
    /// ## it is used to create the Key value Stores
    /// # Example
    /// ```
    /// use kvs::kvs::KvStore;
    /// let store = KvStore::new();
    /// ````
    pub fn new() -> Self {
        KvStore {
            store: HashMap::new(),
        }
    }
    /// # Create a Key Value Item
    /// ## It is used to create a key Value Item
    /// # Example
    /// ```
    /// use kvs::kvs::KvStore;
    /// let mut store = KvStore::new();
    ///
    /// store.set("key".to_string(), "value".to_string());
    /// ```
    pub fn set(&mut self, a: String, b: String) {
        self.store.insert(a, b);
    }
    /// # Gets a Value
    /// ## its get a value from the store
    /// # Example
    ///   ```
    ///
    /// use kvs::kvs::KvStore;
    /// let store = KvStore::new();
    ///
    /// store.get("key".to_string());
    ///
    ///
    pub fn get(&self, a: String) -> Option<String> {
        self.store.get(&a).cloned()
    }

    pub fn remove(&mut self, a: String) {
        self.store.remove(&a);
    }
}

pub fn main() {
    println!("JAI");
}

pub fn kvs() {}
