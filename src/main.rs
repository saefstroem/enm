#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng}, ChaCha20Poly1305, Key, Nonce
};
use eframe::egui::{self, Color32, RichText, TextEdit, Ui, Widget};
use ui::heading::draw_heading;
use std::{
    borrow::Cow,
    collections::HashMap,
    env,
    fs::{self, File},
    io::{Bytes, Write},
    path::Path,
    thread::{self, sleep},
    time::Duration,
};

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
    let yaepm_home = env::var("YAEPM_HOME").unwrap_or(format!("{}/.yaepm", user_home));
    let yaepm_tree = env::var("YAEPM_TREE").unwrap_or("passwords".to_owned());

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 200.0]),
        ..Default::default()
    };

    let encrypted_map: HashMap<String, String> = HashMap::new();

    let mut search_term: String = String::new();
    let mut password: Cow<str> = Cow::from("");
    let mut password_confirmation: Cow<str> = Cow::from("");
    let mut fatal_msg="";
    let mut fatal = false;
    let mut decrypted = false;

    eframe::run_simple_native(
        "yaepm - Yet Another Encrypted Password Manager",
        options,
        move |ctx, _frame| {
            egui::CentralPanel::default().show(ctx, |ui| {
                draw_heading(ui);

                if !Path::try_exists(&Path::new(&yaepm_home))
                    .expect("Could not check existence of yaepm database")
                {
                    ui.add_space(10.0);
                    ui.vertical_centered(|ui: &mut egui::Ui| {
                        ui.label("No db found, set encryption password");

                        ui.add_space(5.0);

                        let first_password_field = ui.label(RichText::new("Password").size(10.0));
                        TextEdit::singleline(&mut password)
                            .password(true)
                            .ui(ui)
                            .labelled_by(first_password_field.id);

                        ui.add_space(4.0);
                        let second_password_field =
                            ui.label(RichText::new("Confirm password").size(10.0));

                        TextEdit::singleline(&mut password_confirmation)
                            .password(true)
                            .ui(ui)
                            .labelled_by(second_password_field.id);

                        ui.add_space(5.0);
                        if ui.button("Save").clicked()
                            && password == password_confirmation
                            && password.len() > 0
                            && password_confirmation.len() > 0
                        {   
                            
                            match File::create(&yaepm_home) {
                                Ok(mut db)=>{
                                    let password_buffer=password.as_bytes();
                                    let mut key = Key::default();
                                    for byte in password_buffer {
                                        key.fill(*byte);
                                    }
                                    let key = ChaCha20Poly1305::new(&key);
                                    let mut nonce = Nonce::default();
                                    nonce.fill(0);
                                    

                                    match key.encrypt(&nonce, b"yaepm".as_ref()) {
                                        Ok(cipher)=>{
                                            let mut cipher:Vec<u8>=cipher;
                                            for byte in b"\n" {
                                                cipher.push(*byte);
                                            }
                                            if db.write_all(&cipher).is_err() {
                                                fatal_msg="Could not write to db";
                                                fatal=true;
                                            }                                         
                                        },
                                        Err(_)=>{
                                            fatal_msg=""
                                            fatal=true;
                                        }
                                    }
                                },
                                Err(_)=>{
                                    fatal=true;
                                }
                            }

                            
                            
                        }

                        if password != password_confirmation
                            && password.len() > 0
                            && password_confirmation.len() > 0
                        {
                            ui.label(RichText::new("Passwords not maching").color(Color32::RED));
                        }

                        if fatal {
                            ui.label(RichText::new(fatal_msg).color(Color32::RED));
                        }
                    });
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
