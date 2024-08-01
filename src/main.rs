#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui::{self, Color32, RichText};
use serde::{Deserialize, Serialize};
use std::env;
use ui::{create::draw_create, decrypt::draw_decrypt, heading::draw_heading, home::draw_home};
use zeroize::ZeroizeOnDrop;

mod db;
mod ui;

/**
 * The UI state of the application
 * Home: The default state of the application, shows all notes (encrypted)
 * CreateNote: The state where the user can create a new note
 * DecryptNote: The state where the user can decrypt a note
 */
pub enum UiState {
    Home,
    CreateNote,
    DecryptNote,
}

/**
 * Throughout the application, the user can receive different notifications
 * that are out of different variants depending on the message.
 */
pub enum Message {
    Neutral(&'static str),
    Success(&'static str),
    Pending(&'static str),
    Error(&'static str),
}

// Implement the Default trait for the Message enum
impl Default for Message {
    fn default() -> Self {
        Self::Neutral("No new notifications")
    }
}

/**
 * This is the format that is used to store notes in the database.
 */
#[derive(Default, Serialize, Deserialize, ZeroizeOnDrop, Debug)]
pub struct Note {
    nonce: Vec<u8>,
    salt: Vec<u8>,
    cipher: Vec<u8>,
}

/**
 * This struct is used to store the text buffers for the text fields in the UI.
 */
#[derive(Default, ZeroizeOnDrop, Debug)]
pub struct TextBuffers {
    name: String,
    content: String,
    password: String,
}

fn main() -> eframe::Result<()> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let user_home = env::var("HOME").unwrap_or_default();
    // Directory of db
    // The DB is a tree.
    let yaepm_tree = env::var("YAEPM_TREE").unwrap_or("passwords".to_owned());
    let db = sled::open(yaepm_home).expect("Could not open yaepm database");
    let senma_home = env::var("SENMA_HOME").unwrap_or(format!("{}/.senma", user_home));
    let senma_home = env::var("ENMA_HOME").unwrap_or(format!("{}/.senma", user_home));
 


    // Setup the window options for the application
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_min_inner_size([350.0, 400.0]).with_max_inner_size([350.0,400.0]).with_resizable(false),
        ..Default::default()
    };

    // Ui state instance
    let mut ui_state = UiState::Home;
    // Buffer for the encrypted note used in R/W ops.
    let mut note = Note::default();

    // Text buffers are stored separately, but are used for R/W from/to UI.
    let mut buffers = TextBuffers::default();

    // Text buffer for the notifications
    let mut message = Message::default();

    eframe::run_simple_native(
        "yaepm - Yet Another Encrypted Password Manager",
        options,
        move |ctx, _frame| {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.horizontal_wrapped(|ui| {
                    // Setup a new row
                    draw_heading(ui);
                    if ui
                        .button(
                            // Create the home button
                            RichText::new("Home")
                                .underline()
                                .background_color(Color32::LIGHT_BLUE)
                                .color(Color32::BLACK),
                        )
                        .clicked()
                    {
                        buffers = TextBuffers::default();
                        note = Note::default();
                        message = Message::default();
                        ui_state = UiState::Home;
                    }
                    if ui
                        .button(
                            // Create the add new note button
                            RichText::new(" + ")
                                .size(14.0)
                                .underline()
                                .background_color(Color32::LIGHT_BLUE)
                                .color(Color32::BLACK),
                        )
                        .clicked()
                    {
                        message = Message::default();
                        ui_state = UiState::CreateNote;
                    }
                });
                ui.separator();

                // Depending on the variant, we have different colors on the error message
                let parsed_message: (&str, Color32) = {
                    match message {
                        Message::Success(message) => (message, Color32::GREEN),
                        Message::Error(message) => (message, Color32::RED),
                        Message::Neutral(message) => (message, Color32::WHITE),
                        Message::Pending(message) => (message, Color32::YELLOW),
                    }
                };

                // Continously render the notification, so if one exists it will be displayed
                ui.label(RichText::new(parsed_message.0).color(parsed_message.1));

                // Depending on the current state, a different UI is displayed.
                match ui_state {
                    UiState::Home => draw_home(ui, &tree, &mut ui_state, &mut note, &mut message),
                    UiState::CreateNote => {
                        draw_create(ui, &tree, &mut note, &mut buffers, &mut message)
                    }
                    UiState::DecryptNote => draw_decrypt(ui, &mut note, &mut buffers, &mut message),
                }
            });
        },
    )
}
