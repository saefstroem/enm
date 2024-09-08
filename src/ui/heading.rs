use eframe::egui::{Color32, RichText, Ui};

pub fn draw_heading(ui: &mut Ui,native_green:Color32) {
    ui.label(
        RichText::new("ENM")
            .italics()
            .heading()
            .extra_letter_spacing(2.0)
            .color(native_green),
    );
}
