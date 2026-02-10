use crate::{KvsEngine, Result};
use std::path::PathBuf;

pub struct SledKvsEngine {
    db: sled::Db,
}

impl SledKvsEngine {
    pub fn open(path: impl Into<PathBuf>) -> Result<Self> {
        let db = sled::open(path.into())?;
        Ok(SledKvsEngine { db })
    }
}

impl KvsEngine for SledKvsEngine {
    fn set(&mut self, key: String, value: String) -> Result<()> {
        self.db.insert(key.as_bytes(), value.as_bytes())?;
        self.db.flush()?;
        Ok(())
    }
    fn get(&mut self, key: String) -> Result<Option<String>> {
        let value = self.db.get(key.as_bytes())?;
        match value {
            Some(bytes) => {
                let s = String::from_utf8(bytes.to_vec())?;
                Ok(Some(s))
            }
            None => Ok(None),
        }
    }
    fn remove(&mut self, key: String) -> Result<()> {
        let old = self.db.remove(key.as_bytes())?;
        self.db.flush()?;
        match old {
            Some(_) => Ok(()),
            None => Err(failure::err_msg("Key not found")),
        }
    }
}
