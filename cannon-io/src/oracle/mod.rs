//! Interact with the host preimage oracle to retrieve data by its key
mod key;
mod oracle_reader;

pub use key::PreimageKey;
pub use oracle_reader::{oracle_reader, OracleReader, Read};
