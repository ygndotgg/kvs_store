use crate::Cmd;
use crate::Result;
use std::fs::OpenOptions;
use std::io::BufWriter;
use std::io::Write;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};
/// # A implementatoin of Key Value Store

pub struct KvStore {
    store: HashMap<String, String>,
    log_path: PathBuf,
}

impl KvStore {
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let cmd = Cmd::Set {
            key: key.clone(),
            value: value.clone(),
        };
        let serialized = serde_json::to_string(&cmd)?;

        // INFO Creating or Opening in Append Mode
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.log_path)?;

        writeln!(file, "{}", serialized)?;
        // INFO: Updating In Memory Index
        self.store.insert(key, value);
        Ok(())
    }
    pub fn get(&self, key: String) -> Result<Option<String>> {
        Ok(self.store.get(&key).cloned())
    }

    pub fn remove(&mut self, key: String) -> Result<()> {
        if !self.store.contains_key(&key) {
            return Err(failure::err_msg("Key not found"));
        }
        let cmd = Cmd::Rm { key: key.clone() };
        let serialized = serde_json::to_string(&cmd)?;
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.log_path)?;
        writeln!(file, "{}", serialized)?;
        // INFO Remove from in-memory index
        self.store.remove(&key);
        Ok(())
    }

    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        let dir = path.into();
        let log_path = dir.join("log");
        let mut store = HashMap::new();
        if log_path.exists() {
            let file = File::open(&log_path)?;
            let reader = BufReader::new(file);
            for f in reader.lines() {
                let f = f?;
                let cmd: Cmd = serde_json::from_str(&f)?;
                match cmd {
                    Cmd::Set { key, value } => {
                        store.insert(key, value);
                    }
                    Cmd::Rm { key } => {
                        store.remove(&key);
                    }
                }
            }
        }
        Ok(KvStore { store, log_path })
    }
}
