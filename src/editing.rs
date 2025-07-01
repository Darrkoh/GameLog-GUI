use eframe::egui::{Button, Color32, Context, Direction, Label, Layout, RichText, TextEdit, Vec2, Window};

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

            ui.add_space(5.0);

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
                ui.add_space(5.0);
            }

            ui.add_space(2.0);

            if ui.add_sized(button_size, Button::new("Search")).clicked() {
                self.error_confirmation = false;

                self.search_result = match search_for_game(&self.game_file_contents, &self.editing_search_game_name) // Make sure game isn't already in the log (Remind users who may have forgot)
                    {
                        Ok(index) =>  {
                            self.editing_search_feedback = format!("Game Found!");
                            self.error_confirmation = true;
                            Some(vec![self.game_file_contents[index].clone()])
                        },
                        Err(_) => { 
                            self.editing_search_feedback = format!("Game Not Found!");
                            None
                        }
                    };
            }
            // Once a game is found, Expand the window and add options for editing game information
            // This also allows us to hide these assets should the user search for an invalid game after, as well as seamlessly updating the current information
            if self.error_confirmation {
                ui.add_space(20.0);
                
                let container_width = 300.0; // width to hold elements being held in a horizontal container

                ui.allocate_ui_with_layout(
                    Vec2::new(container_width, 20.0),
                    Layout::centered_and_justified(Direction::LeftToRight),
                    |ui| {

                        let game_found = self.search_result.as_ref().unwrap();
                        let game = &game_found[0];

                        ui.horizontal(|ui| {
                            ui.add_sized(Vec2::new(50.0, 20.0),
                                Label::new(RichText::new("Current Name: "))
                            );

                            ui.add_sized(Vec2::new(50.0, 20.0),
                                Label::new(RichText::new(&game.name)
                                .strong())
                            );

                            ui.add_space(10.0);

                            ui.add_sized(input_box_size,
                                TextEdit::singleline(&mut self.add_game_name)
                                    .hint_text("New Game Name (< 50 Char)")
                                    .char_limit(50)
                            );

                            ui.add_space(10.0);
                        });
                    },
                );
            }
        });
    }
}