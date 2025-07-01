use eframe::egui::{Button, Color32, Label, RichText, TextEdit, Vec2};

use crate::{app_setup::GameLog, egui::Ui, json_file_operations::search_for_game};

// 'Editing' Window GUI Code
impl GameLog {
    pub fn editing_gui (&mut self, ui: &mut Ui) 
    {
        let label_size= Vec2::new(ui.available_width(), 20.0); // IMPORTANT: Biggest Element needs to be the same width as the ui if centering vertically to ensure it's actually centered when resizing
        let input_box_size = Vec2::new(150.0, 20.0);
        let button_size = Vec2::new(100.0, 20.0);

        ui.add_space(5.0);

        ui.vertical_centered( |ui| {
            ui.add_sized(label_size, Label::new(RichText::new("Enter Game You Are Editing")
                .size(20.0)) // This is the text size, not the allocated space size
            );

            ui.add_space(2.0);
            
            ui.add_sized(input_box_size, TextEdit::singleline(&mut self.editing_search_game_name)
                    .hint_text("Game Name (< 50 Char)")
                    .char_limit(50)
                    .desired_width(input_box_size.x)
            );

            ui.add_space(2.0);

            if !self.editing_search_feedback.is_empty()
            {
                ui.add_sized(label_size, Label::new(RichText::new(&self.editing_search_feedback)
                    .size(20.0)
                    .color(
                        if self.error_confirmation == false {
                            Color32::RED
                        }
                        else {
                            Color32::GREEN
                        }
                    ))
                );
            }

            ui.add_space(2.0);

            if ui.add_sized(button_size, Button::new("Search")).clicked() {
                self.error_confirmation = false;

                match search_for_game(&self.game_file_contents, &self.editing_search_game_name) // Make sure game isn't already in the log (Remind users who may have forgot)
                    {
                        Ok(_) =>  {
                            self.editing_search_feedback = format!("Game Found!");
                            self.error_confirmation = true;
                        },
                        Err(_) => self.editing_search_feedback = format!("Game Not Found!")
                    }
            }  
        });
    }
}