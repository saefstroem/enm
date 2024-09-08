use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use zeroize::ZeroizeOnDrop;

pub mod delete;
pub mod read;
pub mod write;

/**
 * This is the format that is used to store notes in the database.
 */
#[derive(Default, Clone, Serialize, Deserialize, ZeroizeOnDrop, Debug)]
pub struct Note {
    pub nonce: Vec<u8>,
    pub salt: Vec<u8>,
    pub cipher: Vec<u8>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct EnmStorage {
    pub notes: HashMap<String, Note>,
}
