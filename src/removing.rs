use eframe::egui::{Align, Button, Checkbox, Color32, Label, RichText, TextEdit, Vec2};

use crate::{app_setup::GameLog, egui::Ui, json_file_operations::{save_to_file, search_for_game}};

// 'Removing' Window GUI Code
impl GameLog {
    pub fn removing_gui (&mut self, ui: &mut Ui) {

        let label_size = Vec2::new(ui.available_width(), 20.0); 
        let input_box_size = Vec2::new(label_size.x / 2.0, 20.0);
        let check_box_size = Vec2::new(ui.available_width() / 2.0, 20.0);
        let button_size = Vec2::new(ui.available_width() - 120.0, 40.0); // Buttons Width Will Increase as window width increases

        self.enabled = self.checked;

        ui.vertical_centered(|ui| {
            // Title
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

            // Confirmation Checkbox
            ui.add_sized(
                check_box_size,
                Checkbox::new(
                    &mut self.checked,
                    "Are you sure you wish to Remove this Game?",
                ),
            );

            ui.add_space(10.0);

            // Feedback Message On whether or not removal was successful
            if !self.feedback_message.is_empty() {
                ui.add_sized(
                label_size,
                Label::new(RichText::new(&self.feedback_message)
                    .size(20.0)
                    .color(
                        if self.error_confirmation {
                            Color32::RED
                        }
                        else if self.dark_mode {
                            Color32::GREEN
                        }
                        else {
                            Color32::DARK_GREEN
                        }
                    ))
                );
            }

            ui.add_space(5.0);

            // Button 
            ui.add_enabled_ui(self.enabled, |ui| {
                if ui.add_sized(button_size, Button::new("Go"))
                    .clicked() {
                        self.error_confirmation = true;

                        self.feedback_message = match search_for_game(&self.game_file_contents, &self.remove_game_name) {
                            Ok(i) => {
                                self.game_file_contents.remove(i);
                                // Save Edits
                                match save_to_file(&mut self.game_file_contents) {
                                    Ok(_) => { 
                                        self.error_confirmation = false;
                                        self.remove_game_name.clear(); // Clear Input Box
                                        format!("Game Removed Successfully")
                                    },
                                    Err(_) => format!("Error Removing Game"),
                                }
                            },
                            Err(_) => format!("Invalid Game Entered") // If user enters an invalid Game, Let them Know
                        };
                    }
            });

            ui.add_space(5.0);

        });
    }
}