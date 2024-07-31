use chacha20poly1305::{
    aead::{Aead, OsRng},
    AeadCore, ChaCha20Poly1305, Key, KeyInit, Nonce,
};
use eframe::egui::{self, TextEdit};
use password_hash::SaltString;
use sled::Tree;

use crate::{db::set, Message, Note, TextBuffers};
use pbkdf2::pbkdf2_hmac;
use sha2::Sha256;
use zeroize::Zeroize;

pub fn draw_create(ui: &mut egui::Ui, tree: &Tree, note: &mut Note,buffers:&mut TextBuffers, message:&mut Message) {
    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.label("Note Name");
        TextEdit::singleline(&mut buffers.name)
            .desired_width(f32::INFINITY)
            .char_limit(15)
            .show(ui);

        ui.label("Note Content");
        TextEdit::multiline(&mut buffers.content)
            .desired_width(f32::INFINITY)
            .desired_rows(5)
            .char_limit(500)
            .show(ui);

        ui.label("Password");
        TextEdit::singleline(&mut buffers.password)
            .desired_width(f32::INFINITY)
            .password(true)
            .show(ui);

        if ui.button("Save").clicked() {
            *message=Message::Pending("Encrypting...");
            
            let salt = SaltString::generate(&mut OsRng);
            salt.as_str().as_bytes().clone_into(&mut note.salt);

            note.nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng)
                .as_slice()
                .to_vec(); // 96-bits; unique per message
        
            // number of iterations
            let n = 600_000;
            let mut encryption_key = [0u8; 32];
            pbkdf2_hmac::<Sha256>(buffers.password.as_bytes(), &note.salt, n, &mut encryption_key);
            buffers.password.zeroize();

            let cipher = ChaCha20Poly1305::new(Key::from_slice(&encryption_key));
            note.encrypted = cipher
                .encrypt(Nonce::from_slice(&note.nonce), buffers.content.as_bytes())
                .unwrap();
            buffers.content.zeroize();

            // Insert the encrypted note into the database
            match set::<Note>(tree, &buffers.name, note) {
                Ok(_) => {
                    buffers.name.zeroize();
                    *message=Message::Success("Encryption successful");
                }

                Err(error) => {
                    *message=Message::Error("Could not save note");
                    log::error!("Could not save note: {}", error);

                }
            }
        }
    });
}
