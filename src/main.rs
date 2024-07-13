#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    ChaCha20Poly1305, Key, Nonce,
};
use db::{YaepmDb};
use eframe::egui::{self, Color32, RichText, TextEdit, Ui, Widget};
use std::{
    borrow::Cow,
    collections::HashMap,
    env,
    fs::{self, File},
    io::{Bytes, Write},
    path::Path,
    thread::{self, sleep},
    time::{Duration, SystemTime},
};
use ui::{create::draw_create, heading::draw_heading, home::draw_home};
use zeroize::Zeroize;

mod db;
mod ui;
/*
fn search(search_term:String)-> Result<Option<IVec>,Error>  {

    let yaepm_tree=db.open_tree("passwords").expect(&format!("Could not open tree: {}",yaepm_tree));
    yaepm_tree.get(search_term)
}

fn decrypt(buffer:IVec,secret:String) {
    let buffer=buffer.to_vec();
    let secret=String::as_bytes(&secret).to_owned();
    let mut key = Key::default();
    for byte in secret {
        key.fill(byte);
    }
    let cipher = ChaCha20Poly1305::new(&key);
-
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng); // 96-bits; uniq
    let ciphertext = cipher.encrypt(&nonce, b"plaintext message".as_ref())?;
    let plaintext = cipher.decrypt(&nonce, ciphertext.as_ref())?;
    assert_eq!(&plaintext, b"plaintext message");
} */

// All but Home should have payloads. CreateNote should have four mutable strings called note_name, note_content, password, and password_confirmation. DecryptNote should have a string called note_name and password. Error should have a string called error_message
pub enum UiState {
    Home,
    CreateNote,
    DecryptNote,
    Error(&'static str),
}
#[derive(Default)]
pub struct NewNote {
    name: String,
    content: String,
    password: String,
}


#[derive(Default)]
pub struct ExistingNote {
    name: String,
    content: Vec<u8>,
    password: String,
    nonce: u64,
}



fn main() -> eframe::Result<()> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let user_home = env::var("HOME").unwrap_or_default();
    let yaepm_home = env::var("YAEPM_HOME").unwrap_or(format!("{}/yaepm", user_home));
    let yaepm_tree = env::var("YAEPM_TREE").unwrap_or("passwords".to_owned());
    let db=sled::open(yaepm_home).expect("Could not open yaepm database");
    let tree=db.open_tree(yaepm_tree).expect(&format!("Could not open tree"));

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default(),
        ..Default::default()
    };

    let mut ui_state = UiState::Home;
    let mut new_note=NewNote::default();
    let mut existing_note=ExistingNote::default();


    eframe::run_simple_native(
        "yaepm - Yet Another Encrypted Password Manager",
        options,
        move |ctx, _frame| {
            egui::CentralPanel::default().show(ctx, |ui| {
                draw_heading(ui);

                match ui_state {
                    UiState::Home => {
                        draw_home(ui, &tree, &mut ui_state)
                    },
                    UiState::CreateNote => {
                        draw_create(&mut ui_state, ui, &tree, &mut new_note)
                    },
                    UiState::DecryptNote => {

                    },
                    UiState::Error(error_message) => {

                    },
                }
        
                // Add button to create a note, if it is clicked, show a popup with a text edit field and a password field, confirmation field and a save button
  
                //let db:Db=open(&yaepm_home).expect(&format!("Could not open yaepm database at: {}",yaepm_home));

                /*
                ui.heading("");
                ui.text_edit_singleline(&mut encryption_password);


                if ui.button("Search").clicked() {

                }*/
            });
        },
    )
}
