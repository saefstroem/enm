use chacha20poly1305::{aead::Aead, ChaCha20Poly1305, Key, KeyInit, Nonce};
use eframe::egui::{self, Color32, RichText, TextEdit};
use pbkdf2::pbkdf2_hmac;
use sha2::Sha256;

use crate::{storage::Note, Message, TextBuffers, UiState};
use zeroize::Zeroize;

pub fn draw_decrypt(
    ui_state: &mut UiState,
    ui: &mut egui::Ui,
    note: Note,
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
                        let utf_8 = String::from_utf8(plaintext);

                        if let Ok(utf_8) = utf_8 {
                            buffers.zeroize();
                            *ui_state = UiState::Read(utf_8);
                        } else {
                            *message = Message::Error(
                                "Could not convert decrypted note to UTF-8".to_string(),
                            );
                            log::error!("Could not convert decrypted note to UTF-8");
                        }
                    }
                    Err(error) => {
                        *message = Message::Error(format!("Could not decrypt note: {}", error));
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
