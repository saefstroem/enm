use chacha20poly1305::{
    aead::{Aead, OsRng},
    AeadCore, ChaCha20Poly1305, Key, KeyInit, Nonce,
};
use eframe::egui::{self, Color32, RichText, TextEdit};
use password_hash::SaltString;
use sled::Tree;

use crate::{db::set, Message, Note, TextBuffers};
use pbkdf2::pbkdf2_hmac;
use sha2::Sha256;
use zeroize::Zeroize;

/**
 * Prior to saving a note to db, we must be certain that zeroize has
 * emptied the contents of the buffers. We do this with an assertion,
 * which will panic if not satisfied.
 */
fn safety_check(note: &mut Note, buffers: &mut TextBuffers) {
    assert!(!note.cipher.is_empty());
    assert!(!note.nonce.is_empty());
    assert!(!note.salt.is_empty());
    assert!(buffers.content.is_empty());
    assert!(buffers.password.is_empty());
}

/**
 * Draws the UI responsible for encrypting data to disk.
 */
pub fn draw_create(
    ui: &mut egui::Ui,
    tree: &Tree,
    note: &mut Note,
    buffers: &mut TextBuffers,
    message: &mut Message,
) {
    // Create a scrollable area
    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.label("Note name");
        TextEdit::singleline(&mut buffers.name) // Single line text input
            .desired_width(f32::INFINITY)
            .char_limit(15)
            .show(ui);
        ui.label(
            RichText::new("Note name is not encrypted.")
                .color(Color32::RED)
                .size(10.0),
        );

        ui.label("Password");
        TextEdit::singleline(&mut buffers.password) // Single line text input
            .desired_width(f32::INFINITY)
            .password(true) // Hide text from display
            .show(ui);

        ui.label("Note content");
        // Multiline text editor for note content should always be taking up 60% of the screen height
        TextEdit::multiline(&mut buffers.content)
            .desired_width(f32::INFINITY)
            .desired_rows(10)
            .char_limit(500)
            .show(ui);

        ui.add_space(10.0);
        if ui.button("Encrypt").clicked() {
            // Create a button and a handler.
            *message = Message::Pending("Encrypting...");

            // Generate a new salt for encryption
            let salt = SaltString::generate(&mut OsRng);
            salt.as_str().as_bytes().clone_into(&mut note.salt);

            // Generate a random nonce used with encryption.
            note.nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng)
                .as_slice()
                .to_vec(); // 96-bits; unique per message

            // number of iterations
            let n = 600_000;
            let mut encryption_key = [0u8; 32];

            // Generage pbkdf2 hash with password and salt as seeds.
            pbkdf2_hmac::<Sha256>(
                buffers.password.as_bytes(),
                &note.salt,
                n,
                &mut encryption_key,
            );

            // Since we are done with the password we may discard it
            buffers.password.zeroize();

            // Create a new encryption key from the previously generated hash
            let cipher = ChaCha20Poly1305::new(Key::from_slice(&encryption_key));

            // Encrypt the content and store it in the note buffer.
            note.cipher = cipher
                .encrypt(Nonce::from_slice(&note.nonce), buffers.content.as_bytes())
                .unwrap();

            // Since we have encrypted, we may discard the content.
            buffers.content.zeroize();

            // Ensure that buffers are zeroized
            safety_check(note, buffers);

            // Insert the encrypted note into the database
            match set::<Note>(tree, &buffers.name, note) {
                Ok(_) => {
                    buffers.name.zeroize();
                    *message = Message::Success("Encryption successful");
                }

                Err(error) => {
                    *message = Message::Error("Could not save note");
                    log::error!("Could not save note: {}", error);
                }
            }
        }
    });
}
