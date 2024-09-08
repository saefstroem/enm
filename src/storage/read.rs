use std::{env, io};

use super::EnmStorage;

/**
 * Reads encrypted notes from disk
 */
pub fn read_notes() -> Result<EnmStorage, io::Error> {
    let user_home = env::var("HOME").unwrap_or_default();
    let enm_home = env::var("ENM_HOME").unwrap_or(format!("{}/.enm", user_home));
    let data = std::fs::read(enm_home)?;
    let notes = serde_json::from_slice(&data)?;
    Ok(notes)
}
