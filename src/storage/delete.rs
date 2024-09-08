use std::io;

use super::{read::read_notes, write::write_notes};

/**
 * Deletes a note from the storage
 */
pub fn delete_note(key: String) -> Result<(), io::Error> {
    let mut enm_storage = read_notes()?;
    enm_storage.notes.remove(&key);
    write_notes(enm_storage)?;
    Ok(())
}
