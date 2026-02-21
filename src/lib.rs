// #![deny(missing_docs)]
use failure::Error;
pub use kvs::KvStore;
use serde::{Deserialize, Serialize};
pub type Result<T> = std::result::Result<T, Error>;
pub mod kvs;
pub mod sled_engine;
pub mod thread_pool;
pub use server::KvServer;
pub use sled_engine::SledKvsEngine;
mod server;

#[derive(Serialize, Deserialize)]
pub enum Cmd {
    Set { key: String, value: String },
    Rm { key: String },
}

#[derive(Serialize, Deserialize)]
pub enum Request {
    Set { key: String, value: String },
    Get { key: String },
    Remove { key: String },
}

#[derive(Serialize, Deserialize)]
pub enum Response {
    Ok(Option<String>),
    Err(String),
}

pub trait KvsEngine: Clone + Send + 'static {
    fn set(&self, key: String, value: String) -> Result<()>;
    fn get(&self, key: String) -> Result<Option<String>>;
    fn remove(&self, key: String) -> Result<()>;
}
