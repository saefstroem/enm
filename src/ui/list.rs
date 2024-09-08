use eframe::egui::{self, Button, Color32, RichText, Separator};

use crate::{
    storage::{delete::delete_note, read::read_notes},
    Message, UiState,
};

pub fn draw_list(ui: &mut egui::Ui, ui_state: &mut UiState, message: &mut Message, confirm_delete: &mut String) {
    ui.label(RichText::new("Your notes").size(14.0).underline());

    egui::ScrollArea::vertical()
        .max_height(f32::INFINITY)
        .show(ui, |ui| {
            match read_notes() {
                Ok(storage) => {
                    let notes = storage.notes;
                    
                    if notes.is_empty() {
                        ui.label("No notes found");
                    }
                    ui.add_space(5.0);
                    let mut keys:Vec<String>=Vec::new();
                    for (key, _) in &notes {
                        keys.push(key.clone());
                    }
                    keys.sort();
                    
                    for key in keys {
                        // Label of the encrypted note and a button to decrypt it
                        ui.horizontal_wrapped(|ui| {
                            ui.horizontal_centered(|ui| {
                                let mut total_width = ui.available_width();
                                let note_name = ui.label(&key);
                                total_width -= note_name.rect.width();
                                ui.add(Separator::default().vertical());
                                
                                if confirm_delete==&key {
                                    ui.add_space(total_width - 170.0);

                                    ui.label("Are you sure?");
                                    let yes_button = Button::new(RichText::new("Yes").color(Color32::BLACK)).fill(Color32::LIGHT_RED);
                                    let no_button = Button::new(RichText::new("No").color(Color32::BLACK)).fill(Color32::LIGHT_GREEN);
                                    if ui.add(yes_button).clicked() {
                                        let _ = delete_note(confirm_delete.to_string()).map_err(|err| {
                                            *message = Message::Error(format!(
                                                "Could not delete note {}",
                                                err
                                            ));
                                        });
                                        *confirm_delete = String::new();

                                    }
                                    if ui.add(no_button).clicked() {
                                        *confirm_delete = String::new();
                                    }
                                } else {
                                    ui.add_space(total_width - 115.0);

                                    let decrypt_button = ui.button("Decrypt");

                                let delete_button =
                                    Button::new(RichText::new(" X ").color(Color32::BLACK))
                                        .fill(Color32::LIGHT_RED);

                                if decrypt_button.clicked() {
                                    *message = Message::default();
                                    if let Some(note) = notes.get(&key) {
                                        *ui_state = UiState::Decrypt(note.clone());
                                    } else {
                                        *message = Message::Error(format!("Could not read note {}", key));
                                    }
                                }

                                if ui.add(delete_button).clicked() {
                                    *confirm_delete = key;
                                }

                            }
                                


                            });
                        });
                        
                        ui.separator();
                    }
                    
                }
                Err(error) => {
                    *message = Message::Error(format!("Could not read notes: {:?}", error));
                }
            }
        });
}
