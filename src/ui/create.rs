use std::{fs::File, io::Write, time::SystemTime};

use chacha20poly1305::{aead::Aead, ChaCha20Poly1305, Key, KeyInit, Nonce};
use eframe::egui::{self, TextEdit};
use sled::Tree;

use crate::{db::set, NewNote, UiState};
use zeroize::Zeroize;

pub fn draw_create(ui_state:&mut UiState ,ui: &mut egui::Ui, tree: &Tree, note: &mut NewNote) {
    ui.label("Note Name");
    ui.text_edit_singleline(&mut note.name);
    ui.label("Note Content");
    TextEdit::multiline(&mut note.content).vertical_align(align)
    ui.text_edit_multiline(&mut note.content);
    ui.label("Password");
    ui.text_edit_singleline(&mut note.password);
    
    if ui.button("Save").clicked() {
        let timestamp_millis = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Encrypt the note content
        let mut key = Key::default();
        for byte in note.password.as_bytes() {
            key.fill(*byte);
        }
        let cipher = ChaCha20Poly1305::new(&key);
        let mut nonce = Nonce::default();

        // Loop through the timestamp_millis and use nonce.fill to fill the nonce with the bytes
        for byte in timestamp_millis.to_be_bytes() {
            nonce.fill(byte);
        }

        let ciphertext = cipher.encrypt(&nonce, note.content.as_bytes()).unwrap();
        let ciphertext: Vec<u8> = ciphertext.to_vec();

        // Insert the encrypted note into the database
        match set::<(u64,Vec<u8>)>(tree, &note.name, &(timestamp_millis, ciphertext)) {
            Ok(_) => {
                // Clear the note content and password fields
                note.name.zeroize();
                note.content.zeroize();
                note.password.zeroize();
            },
            Err(error) => {
                *ui_state = UiState::Error("Could not save note");
                log::error!("Could not save note: {}", error);
            }
        
        }
    }
}
