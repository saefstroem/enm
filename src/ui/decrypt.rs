use chacha20poly1305::{aead::Aead, ChaCha20Poly1305, Key, KeyInit, Nonce};
use eframe::egui::{self, Color32, RichText, TextEdit};
use pbkdf2::pbkdf2_hmac;
use sha2::Sha256;

use crate::{Message, Note, TextBuffers};
use zeroize::Zeroize;

pub fn draw_decrypt(
    ui: &mut egui::Ui,
    note: &mut Note,
    buffers: &mut TextBuffers,
    message: &mut Message,
) {
    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.label("Password");
        ui.horizontal_wrapped(|ui| {
            TextEdit::singleline(&mut buffers.password) // New single line input
                .desired_width(f32::INFINITY)
                .password(true)
                .show(ui);

            ui.add_space(10.0);
            if ui.button(RichText::new("Decrypt")).clicked() {
                // number of iterations
                let n = 600_000;
                let mut encryption_key = [0u8; 32]; // Buffer for the encryption key
                pbkdf2_hmac::<Sha256>(
                    // Generate pbkdf2 hash from password
                    buffers.password.as_bytes(),
                    &note.salt,
                    n,
                    &mut encryption_key,
                );

                // Clear the password from memory
                buffers.password.zeroize();

                // Create a new key from the pbkdf2 hash
                let cipher = ChaCha20Poly1305::new(Key::from_slice(&encryption_key));

                // Use the same nonce for decryption
                let nonce = Nonce::from_slice(&note.nonce);

                // Attempt to decrypt the cipher
                match cipher.decrypt(nonce, note.cipher.as_ref()) {
                    Ok(plaintext) => {
                        // Discard the cipher
                        note.cipher.zeroize();
                        // Display the decrypted note
                        buffers.content = String::from_utf8(plaintext).unwrap();
                        *message = Message::Success("Decryption successful");
                    }
                    Err(error) => {
                        *message = Message::Error("Could not decrypt note");
                        log::error!("Could not decrypt note: {}", error);
                    }
                }
            }
        });
        ui.add_space(10.0);

        // Continously render the content of the buffer.
        ui.label(
            RichText::new(&buffers.content)
                .background_color(Color32::BLACK)
                .color(Color32::WHITE),
        );
    });
}
