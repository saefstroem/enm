#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::{egui::{self, Button, Color32, IconData, RichText}, epaint::image};
use ::image::load_from_memory;
use serde::{Deserialize, Serialize};
use storage::Note;
use ui::{create::draw_create, decrypt::draw_decrypt, heading::draw_heading, list::draw_list};
use zeroize::ZeroizeOnDrop;
mod storage;
mod ui;

/**
 * The UI state of the application
 * Home: The default state of the application, shows all notes (encrypted)
 * CreateNote: The state where the user can create a new note
 * DecryptNote: The state where the user can decrypt a note
 */
#[derive(Clone, ZeroizeOnDrop)]
pub enum UiState {
    List,
    Create,
    Decrypt(Note),
    Read(String),
}

/**
 * Throughout the application, the user can receive different notifications
 * that are out of different variants depending on the message.
 */
pub enum Message {
    Neutral(&'static str),
    Success(&'static str),
    Pending(&'static str),
    Error(String),
}

// Implement the Default trait for the Message enum
impl Default for Message {
    fn default() -> Self {
        Self::Neutral("No new notifications")
    }
}

#[derive(Default, Clone, Serialize, Deserialize, ZeroizeOnDrop)]
pub struct TextBuffers {
    pub name: String,
    pub password: String,
    pub content: String,
}

fn main() -> eframe::Result<()> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let icon = include_bytes!("../icon.png");
    let image = load_from_memory(icon).expect("Failed to open icon path").to_rgba8();
    let (width, height) = image.dimensions();


    // Setup the window options for the application
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_min_inner_size([350.0, 400.0])
            .with_max_inner_size([350.0, 400.0])
            .with_resizable(false)
            .with_icon(IconData{
                rgba:image.into_raw(),
                width,
                height
            }),
        ..Default::default()
    };

    // Ui state instance
    let mut ui_state = UiState::List;

    // Text buffer for the notifications
    let mut message = Message::default();

    let mut buffers = TextBuffers::default();

    let native_green=Color32::from_hex("#34fe40").unwrap_or(Color32::LIGHT_GREEN);

    eframe::run_simple_native(
        "enm - encrypted note manager",
        options,
        move |ctx, _frame| {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.horizontal_wrapped(|ui| {
                    // Setup a new row
                    draw_heading(ui,native_green);

                    let home_button = Button::new(RichText::new("Home").color(Color32::BLACK))
                        .fill(native_green);
                    let add_button = Button::new(RichText::new(" + ").color(Color32::BLACK))
                        .fill(native_green);
                    if ui.add(home_button).clicked() {
                        message = Message::default();
                        ui_state = UiState::List;
                    }
                    if ui.add(add_button).clicked() {
                        message = Message::default();
                        ui_state = UiState::Create;
                    }
                });
                ui.separator();

                // Depending on the variant, we have different colors on the error message
                let parsed_message: (String, Color32) = {
                    match message {
                        Message::Success(message) => (message.to_string(), Color32::GREEN),
                        Message::Error(ref message) => (message.clone(), Color32::RED),
                        Message::Neutral(message) => (message.to_string(), Color32::WHITE),
                        Message::Pending(message) => (message.to_string(), Color32::YELLOW),
                    }
                };

                // Continously render the notification, so if one exists it will be displayed
                ui.label(RichText::new(parsed_message.0).color(parsed_message.1));
                ui.separator();

                // Depending on the current state, a different UI is displayed.
                match &mut ui_state {
                    UiState::List => draw_list(ui, &mut ui_state, &mut message),
                    UiState::Create => draw_create(ui, &mut buffers, &mut message),
                    UiState::Decrypt(ref note) => {
                        let note_clone = note.clone();
                        draw_decrypt(&mut ui_state, ui, note_clone, &mut buffers, &mut message);
                    }
                    UiState::Read(content) => {
                        ui.add_space(2.0);
                        ui.label(RichText::new("Decrypted content").size(14.0).underline());
                        ui.label(RichText::new(content.as_str()).size(13.0));
                    }
                }
            });
        },
    )
}
