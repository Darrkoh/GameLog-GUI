use eframe::egui::{Align, Button, Checkbox, Label, Layout, RichText, TextEdit, Vec2};

use crate::{app_setup::GameLog, egui::Ui};

// 'Removing' Window GUI Code
impl GameLog {
    pub fn removing_gui (&mut self, ui: &mut Ui) {

        ui.set_max_size(Vec2::new(200.0, 50.0)); // Set to match element sizes for a perfect layout

        let label_size = Vec2::new(ui.available_width(), 20.0); // Should use available width on the left most element (in this case the label) as this will center them
        let input_box_size = Vec2::new(150.0, 20.0);
        let button_size = Vec2::new(100.0, 20.0);

        self.enabled = self.checked;

        // Enter Game Name
        ui.vertical_centered(|ui| {
            ui.with_layout(Layout::left_to_right(Align::Center), |ui| {

                ui.add_sized(
                    label_size,
                    Label::new(RichText::new("Game Name")
                        .strong())
                );

                ui.add_sized(
                input_box_size,
                TextEdit::singleline(&mut self.remove_game_name)
                    .hint_text("Game Name")
                    .horizontal_align(Align::Center)
                    .char_limit(50)
                );

                ui.add_space(20.0);
            });
        });

        ui.add_space(20.0);

        // Confirm Removal Check + Button
        ui.vertical_centered(|ui| {
            ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                ui.add_sized(
                    label_size,
                    Checkbox::new(
                        &mut self.checked,
                        "Are you sure you wish to Remove this Game?",
                    ),
                );

                ui.add_space(10.0);

                ui.add_enabled_ui(self.enabled, |ui| {
                    if ui.add_sized(Vec2::new(150.0, 20.0), Button::new("Go"))
                        .clicked() {
                             // Code Here to remove a Game
                        }
                });

                ui.add_space(5.0);

            });
        });
        ui.add_space(5.0);
    }
}