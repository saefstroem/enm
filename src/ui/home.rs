use eframe::egui::{self, Color32, RichText, Separator};
use sled::Tree;

use crate::{
    db::{delete, get_all, DatabaseError},
    Message, Note, UiState,
};

pub fn draw_home(
    ui: &mut egui::Ui,
    db: &Tree,
    ui_state: &mut UiState,
    note: &mut Note,
    message: &mut Message,
) {
    ui.label(RichText::new("Your notes").size(14.0).underline());

    egui::ScrollArea::vertical()
        .max_height(f32::INFINITY)
        .show(ui, |ui| {
            match get_all::<Note>(db) {
                Ok(notes) => {
                    if notes.is_empty() {
                        ui.label("No notes found");
                    }
                    ui.add_space(5.0);
                    for (key, encrypted_note) in notes {
                        // Label of the encrypted note and a button to decrypt it
                        ui.horizontal_wrapped(|ui| {
                            ui.horizontal_centered(|ui| {
                                let mut total_width = ui.available_width();
                                let note_name = ui.label(&key);
                                total_width -= note_name.rect.width();
                                ui.add(Separator::default().vertical());
                                ui.add_space(total_width - 115.0);
                                let decrypt_button = ui.button("Decrypt");
                                let delete_button = ui.button(
                                    RichText::new(" X ")
                                        .background_color(Color32::LIGHT_RED)
                                        .color(Color32::BLACK),
                                );
                                if decrypt_button.clicked() {
                                    *note = encrypted_note;
                                    *message = Message::default();
                                    *ui_state = UiState::DecryptNote;
                                }
                                if delete_button.clicked() && delete(db, &key).is_err() {
                                    *message = Message::Error("Could not delete note");
                                }
                            });
                        });
                        ui.separator();
                    }
                }
                Err(error) => match error {
                    DatabaseError::NotFound => {
                        ui.label("No notes found");
                    }
                    _ => {
                        *message = Message::Error("Could not get notes from database");
                    }
                },
            }
        });
}
