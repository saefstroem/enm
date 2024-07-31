use chacha20poly1305::{
    aead::Aead,
    ChaCha20Poly1305, Key, KeyInit, Nonce,
};
use eframe::egui::{self, Color32, RichText, TextEdit};
use pbkdf2::pbkdf2_hmac;
use sha2::Sha256;

use crate::{Message, Note, TextBuffers};
use zeroize::Zeroize;

pub fn draw_decrypt(
    ui: &mut egui::Ui,
    note: &mut Note,
    buffers:&mut TextBuffers,
    message:&mut Message
) {
    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.label("Password");
        ui.horizontal_wrapped(|ui| {
            TextEdit::singleline(&mut buffers.password)
                .desired_width(f32::INFINITY)
                .password(true)
                .show(ui);

            if ui.button(RichText::new("Decrypt")).clicked() {
                // number of iterations
                let n = 600_000;
                let mut encryption_key = [0u8; 32];
                pbkdf2_hmac::<Sha256>(buffers.password.as_bytes(), &note.salt, n, &mut encryption_key);
                buffers.password.zeroize();
                let cipher = ChaCha20Poly1305::new(Key::from_slice(&encryption_key));

                let nonce = Nonce::from_slice(&note.nonce);

                match cipher.decrypt(nonce, note.encrypted.as_ref()) {
                    Ok(plaintext) => {
                        note.encrypted.zeroize();
                        // Display the decrypted note
                        buffers.content = String::from_utf8(plaintext).unwrap();
                        *message=Message::Success("Decryption successful");
                    }
                    Err(error) => {
                        *message=Message::Error("Could not decrypt note");
                        log::error!("Could not decrypt note: {}", error);
                    }
                }
            }
        });
        ui.add_space(10.0);
        ui.label(
            RichText::new(&buffers.content)
                .background_color(Color32::BLACK)
                .color(Color32::WHITE),
        );
    });
}
