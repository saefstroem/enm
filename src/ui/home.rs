use eframe::egui;
use sled::Tree;

use crate::{db::{get, get_all, DatabaseError, YaepmDb}, ExistingNote, NewNote, UiState};

pub fn draw_home(ui: &mut egui::Ui, db: &Tree, ui_state: &mut UiState){
    if ui.button("Create Note").clicked() {
        *ui_state = UiState::CreateNote;
    }

    egui::ScrollArea::vertical()
    .max_height(f32::INFINITY)
    .show(ui, |ui| {
        match get_all::<(u64,Vec<u8>)>(db) {
            Ok(notes)=>{
                if notes.is_empty() {
                    ui.label("No notes found");
                }
                for (key, value) in notes {
                    // Label of the encrypted note and a button to decrypt it
                    ui.label(&key);
                    if ui.button("Decrypt").clicked() {
                        *ui_state = UiState::DecryptNote;
                    }
                }
            },
            Err(error)=>{
                match error {
                    DatabaseError::NotFound=>{
                        ui.label("No notes found");
                    },
                    _=>{
                        *ui_state = UiState::Error("Could not get notes from database");
                    }
                }
            }
        }
    });
}