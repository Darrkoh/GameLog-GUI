use eframe::egui::{Align, Button, Checkbox, Label, Layout, RichText, TextEdit, Vec2};

use crate::{app_setup::GameLog, egui::Ui};

// 'Removing' Window GUI Code
impl GameLog {
    pub fn removing_gui (&mut self, ui: &mut Ui) {

        let label_size = Vec2::new(ui.available_width(), 20.0); 
        let input_box_size = Vec2::new(label_size.x / 2.0, 20.0);
        let check_box_size = Vec2::new(ui.available_width() / 2.0, 20.0);
        let button_size = Vec2::new(80.0, 40.0);

        self.enabled = self.checked;

        ui.vertical_centered(|ui| {
            ui.add_sized(
                label_size,
                Label::new(RichText::new("Enter The Game's Name")
                    .size(20.0))
            );
            ui.add_space(10.0);

            // Enter Game Name
            ui.add_sized(
            input_box_size,
            TextEdit::singleline(&mut self.remove_game_name)
                .hint_text("Game Name")
                .horizontal_align(Align::Center)
                .char_limit(50)
            );
    
            ui.add_space(10.0);

            ui.add_sized(
                check_box_size,
                Checkbox::new(
                    &mut self.checked,
                    "Are you sure you wish to Remove this Game?",
                ),
            );

            ui.add_space(10.0);

            ui.add_enabled_ui(self.enabled, |ui| {
                if ui.add_sized(button_size, Button::new("Go"))
                    .clicked() {
                            // Code Here to remove a Game
                    }
            });
        });
    }
}