// #![deny(missing_docs)]
use failure::Error;
pub use kvs::KvStore;
use serde::{Deserialize, Serialize};
pub type Result<T> = std::result::Result<T, Error>;
pub mod kvs;
#[derive(Serialize, Deserialize)]
pub enum Cmd {
    Set { key: String, value: String },
    Rm { key: String },
}

pub trait KvsEngine {
    fn set(&mut self, key: String, value: String) -> Result<()>;
    fn get(&mut self, key: String) -> Result<Option<String>>;
    fn remove(&mut self, key: String) -> Result<()>;
}
