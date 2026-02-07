use crate::Result;
use std::{collections::HashMap, path::PathBuf};
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
    pub fn new() -> KvStore {
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
    pub fn set(&mut self, _key: String, _value: String) -> Result<()> {
        panic!("not yet implemented")
    }
    //
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
    pub fn get(&self, _key: String) -> Result<Option<String>> {
        panic!("not yet implemented")
    }

    pub fn remove(&mut self, _key: String) -> Result<()> {
        panic!("not yet implemented")
    }

    pub fn open(_path: impl Into<PathBuf>) -> Result<KvStore> {
        panic!("not yet implemented")
    }
}
