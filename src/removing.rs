use eframe::egui::{Align, Label, Layout, RichText, TextEdit, Vec2};

use crate::{app_setup::GameLog, egui::Ui};

// 'Removing' Window GUI Code
impl GameLog {
    pub fn removing_gui (&mut self, ui: &mut Ui) {

        let label_size = Vec2::new(100.0, 20.0);
        let input_box_size = Vec2::new(150.0, 20.0);
        let container_height = label_size.y + input_box_size.y + 20.0; // Width to hold elements being held in a horizontal container (Updates with each Layout to match the new space needed)
        ui.add_space(5.0);

        ui.vertical_centered(|ui| {
            ui.group(|ui| {
                ui.set_height(container_height); // constrain group width to input box width
                ui.with_layout(Layout::left_to_right(Align::Center), |ui| {

                    ui.add_sized(label_size,
                    Label::new(RichText::new("Game Name").strong())
                    );

                    ui.add_sized(
                    input_box_size,
                    TextEdit::singleline(&mut self.remove_game_name)
                    .hint_text("Game Name")
                    .horizontal_align(Align::Center)
                    .char_limit(50));

                    ui.add_space(5.0);
                });
            });
        });
    }
}