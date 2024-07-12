#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    ChaCha20Poly1305, Key, Nonce,
};
use db::{read_db, setup_db, YaepmDb};
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
use ui::heading::draw_heading;

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

fn main() -> eframe::Result<()> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let user_home = env::var("HOME").unwrap_or_default();
    let yaepm_home = env::var("YAEPM_HOME").unwrap_or(format!("{}/yaepm.txt", user_home));
    let yaepm_tree = env::var("YAEPM_TREE").unwrap_or("passwords".to_owned());

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 320.0]),
        ..Default::default()
    };

    let encrypted_map: HashMap<String, String> = HashMap::new();

    let mut note_name: String = String::new();
    let mut note_content: String = String::new();
    let mut create_new_note = false;

    let mut password: Cow<str> = Cow::from("");
    let mut password_confirmation: Cow<str> = Cow::from("");

    let mut fatal_msg = "";
    let mut fatal = false;
    let mut db: YaepmDb = YaepmDb {
        notes: HashMap::new(),
    };

    eframe::run_simple_native(
        "yaepm - Yet Another Encrypted Password Manager",
        options,
        move |ctx, _frame| {
            egui::CentralPanel::default().show(ctx, |ui| {
                draw_heading(ui);

                if !Path::try_exists(&Path::new(&yaepm_home))
                    .expect("Could not check existence of yaepm database")
                {
                    if setup_db(&yaepm_home).is_err() {
                        fatal = true;
                        fatal_msg = "Could not create db";
                    }

                    match read_db(&yaepm_home) {
                        Ok(read_db) => {
                            db = read_db;
                        }
                        Err(_) => {
                            fatal = true;
                            fatal_msg = "Could not read db";
                        }
                    }
                } else {
                    match read_db(&yaepm_home) {
                        Ok(read_db) => {
                            db = read_db;
                        }
                        Err(_) => {
                            fatal = true;
                            fatal_msg = "Could not read db";
                        }
                    }
                    // Add button to create a note, if it is clicked, show a popup with a text edit field and a password field, confirmation field and a save button
                    if ui.button("Create Note").clicked() {
                        create_new_note = true;
                    }
                    if !create_new_note {
                        // Iterate through the notes in the hashmap placing the key into a scrollable list
                        egui::ScrollArea::vertical()
                            .max_height(100.0)
                            .show(ui, |ui| {
                                // Add a lot of widgets here.
                                for (key, value) in db.notes.iter() {
                                    ui.label(key.clone());
                                }
                                if db.notes.iter().len() == 0 {
                                    ui.label("No notes found");
                                }
                            });
                    }
                }
                if fatal {
                    ui.label(RichText::new(fatal_msg.to_string()).color(Color32::RED));
                }

                if create_new_note {
                    ui.label("Note Name");
                    ui.text_edit_singleline(&mut note_name);
                    ui.label("Note Content");
                    ui.text_edit_multiline(&mut note_content);
                    ui.label("Password");
                    ui.text_edit_singleline(&mut password);
                    ui.label("Password Confirmation");
                    ui.text_edit_singleline(&mut password_confirmation);
                    if ui.button("Save").clicked() {
                        if password == password_confirmation {
                            let timestamp_millis = SystemTime::now()
                                .duration_since(SystemTime::UNIX_EPOCH)
                                .unwrap()
                                .as_secs();

                            // Encrypt the note content
                            let mut key = Key::default();
                            for byte in password.as_bytes() {
                                key.fill(*byte);
                            }
                            let cipher = ChaCha20Poly1305::new(&key);
                            let mut nonce = Nonce::default();

                            // Loop through the timestamp_millis and use nonce.fill to fill the nonce with the bytes
                            for byte in timestamp_millis.to_be_bytes() {
                                nonce.fill(byte);
                            }

                            let ciphertext =
                                cipher.encrypt(&nonce, note_content.as_bytes()).unwrap();

                            db.notes
                                .insert(note_name.clone(), (timestamp_millis,ciphertext.to_vec()));

                            // Open the file and write the encrypted note to it
                            let mut file =
                                File::create(&yaepm_home).expect("Could not open yaepm database");
                            file.write_all(serde_json::to_string(&db).unwrap().as_bytes())
                                .expect("Could not write to yaepm database");

                            db = read_db(&yaepm_home).unwrap();
                        } else {
                            fatal = true;
                            fatal_msg = "Passwords do not match";
                        }
                    }
                }

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
