use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct YaepmDb {
    pub notes: HashMap<String, (u64, Vec<u8>)>,
}
use sled::Tree;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("No matches found")]
    NotFound,
    #[error("Could not get from database")]
    Get,
    #[error("Could not set to database")]
    Set,
    #[error("Could not communicate with database")]
    Communicate,
    #[error("Could not deserialize binary data")]
    Deserialize,
    #[error("Could not serialize binary data")]
    Serialize,
    #[error("Database internal error: {0}")]
    SledError(#[from] sled::Error),
}

pub struct KVPair {
    pub key: String,
    pub value: Vec<u8>,
}

/// Retrieve all key,value pairs from a specified tree
fn get_all_from_tree(db: &Tree) -> Result<Vec<KVPair>, DatabaseError> {
    let mut all = Vec::new();
    for result in db.iter() {
        let (key, value) = result.map_err(|error| {
            log::error!("Db Interaction Error: {}", error);
            DatabaseError::Get
        })?;
        all.push(KVPair {
            key: String::from_utf8(key.to_vec()).map_err(|error| {
                log::error!("Db Interaction Error: {}", error);
                DatabaseError::Deserialize
            })?,
            value: value.to_vec(),
        });
    }
    Ok(all)
}

/// Wrapper for retrieving all key value pairs from a tree
pub fn get_all<T>(tree: &sled::Tree) -> Result<Vec<(String, T)>, DatabaseError>
where
    T: DeserializeOwned,
{
    let binary_data = get_all_from_tree(tree)?;
    let mut all = Vec::with_capacity(binary_data.len());
    for pair in binary_data {
        // Deserialize binary value to invoice
        let value = bincode::deserialize::<T>(&pair.value).map_err(|error| {
            log::error!("Db Interaction Error: {}", error);
            DatabaseError::Deserialize
        })?;

        all.push((pair.key, value));
    }
    Ok(all)
}

/// Sets a value to a tree
fn set_to_tree(db: &Tree, key: &str, bin: Vec<u8>) -> Result<(), DatabaseError> {
    match db.insert(key, bin) {
        Ok(_) => Ok(()),
        Err(error) => {
            log::error!("Db Interaction Error: {}", error);
            Err(DatabaseError::Set)
        }
    }
}

/// Wrapper for setting a value to a tree
pub fn set<T>(tree: &Tree, key: &str, data: &T) -> Result<(), DatabaseError>
where
    T: Serialize,
{
    let binary_data = bincode::serialize::<T>(data).map_err(|error| {
        log::error!("Db Interaction Error: {}", error);
        DatabaseError::Serialize
    })?;
    set_to_tree(tree, key, binary_data).map_err(|_| DatabaseError::Communicate)?;
    Ok(())
}

/// Used to delete from a tree
pub fn delete(tree: &Tree, key: &str) -> Result<(), DatabaseError> {
    let result = tree.remove(key)?;
    match result {
        Some(_deleted_value) => Ok(()),
        None => Err(DatabaseError::NotFound),
    }
}
