use eframe::egui::{Color32, RichText, Ui};

pub fn draw_heading(ui: &mut Ui) {


    ui.label(
        RichText::new("YAEPM")
            .italics()
            .heading()
            .extra_letter_spacing(1.0)
            .color(Color32::LIGHT_BLUE),
    );
}
