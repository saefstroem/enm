#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui::{self, Color32, RichText};
use serde::{Deserialize, Serialize};
use std::env;
use ui::{create::draw_create, decrypt::draw_decrypt, heading::draw_heading, home::draw_home};
use zeroize::ZeroizeOnDrop;

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
    DecryptNote
}

pub enum Message {
    Neutral(&'static str),
    Success(&'static str),
    Pending(&'static str),
    Error(&'static str),
}

impl Default for Message {
    fn default() -> Self {
        Self::Neutral("No new notifications")
    }
}




#[derive(Default, Serialize, Deserialize, ZeroizeOnDrop, Debug)]
pub struct Note {
    nonce: Vec<u8>,
    salt: Vec<u8>,
    encrypted: Vec<u8>,
}

#[derive(Default, ZeroizeOnDrop, Debug)]
pub struct TextBuffers {
    name: String,
    content: String,
    password: String,
    
}
fn main() -> eframe::Result<()> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let user_home = env::var("HOME").unwrap_or_default();
    let yaepm_home = env::var("YAEPM_HOME").unwrap_or(format!("{}/yaepm", user_home));
    let yaepm_tree = env::var("YAEPM_TREE").unwrap_or("passwords".to_owned());
    let db = sled::open(yaepm_home).expect("Could not open yaepm database");
    let tree = db
        .open_tree(yaepm_tree)
        .unwrap_or_else(|_| panic!("Could not open tree"));

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([250.0, 300.0])
            .with_max_inner_size([250.0,300.0]).with_resizable(false),
        ..Default::default()
    };

    let mut ui_state = UiState::Home;
    let mut note = Note::default();
    let mut buffers = TextBuffers::default();
    let mut message=Message::default();

    eframe::run_simple_native(
        "yaepm - Yet Another Encrypted Password Manager",
        options,
        move |ctx, _frame| {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.horizontal_wrapped(|ui| {
                    draw_heading(ui);
                    if ui
                        .button(
                            RichText::new("Home")
                                .underline()
                                .background_color(Color32::LIGHT_BLUE)
                                .color(Color32::BLACK),
                        )
                        .clicked()
                    {
                        message=Message::default();
                        ui_state = UiState::Home;
                    }
                    if ui
                        .button(
                            RichText::new(" + ")
                                .size(14.0)
                                .underline()
                                .background_color(Color32::LIGHT_BLUE)
                                .color(Color32::BLACK),
                        )
                        .clicked()
                    {
                        message=Message::default();
                        ui_state = UiState::CreateNote;
                    }
                });
                ui.separator();

                let parsed_message: (&str,Color32) = {
                    match message {
                        Message::Success(message) =>{
                            (message,Color32::GREEN)
                        },
                        Message::Error(message)=>{
                            (message,Color32::RED)

                        },
                        Message::Neutral(message)=>{
                            (message,Color32::WHITE)

                        },
                        Message::Pending(message)=>{
                            (message,Color32::YELLOW)
                        }

                    }
                };
                
                ui.label(RichText::new(parsed_message.0).color(parsed_message.1));

             
                match ui_state {
                    UiState::Home => draw_home(ui, &tree, &mut ui_state, &mut note, &mut message),
                    UiState::CreateNote => {
                        draw_create(ui, &tree, &mut note, &mut buffers,&mut message)
                    }
                    UiState::DecryptNote => {
                        draw_decrypt(ui, &mut note, &mut buffers,&mut message)
                    }
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
