// #![deny(missing_docs)]
use failure::Error;
pub type Result<T> = std::result::Result<T, Error>;
pub mod kvs;
