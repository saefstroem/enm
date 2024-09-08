use std::{env, io};

use super::{EnmStorage, Note};

/**
 * Take a note with a name and updates the storage with it.
 */
pub fn write_note(name: String, note: Note) -> Result<(), io::Error> {
    let user_home = env::var("HOME").unwrap_or_default();
    let enm_home = env::var("ENM_HOME").unwrap_or(format!("{}/.enm", user_home));
    let mut enm_storage = match std::fs::read(&enm_home) {
        Ok(data) => serde_json::from_slice(&data).unwrap(),
        Err(_) => EnmStorage::default(),
    };
    enm_storage.notes.insert(name, note);
    let data = serde_json::to_vec(&enm_storage)?;
    std::fs::write(enm_home, data)?;
    Ok(())
}

/**
 * Replaces the current note storage with a new one. Used in deletion.
 */
pub fn write_notes(enm_storage: EnmStorage) -> Result<(), io::Error> {
    let user_home = env::var("HOME").unwrap_or_default();
    let enm_home = env::var("ENM_HOME").unwrap_or(format!("{}/.enm", user_home));
    let data = serde_json::to_vec(&enm_storage)?;
    std::fs::write(enm_home, data)?;
    Ok(())
}
