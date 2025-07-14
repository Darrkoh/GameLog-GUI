// Crates/Imports
use eframe::egui::{Align, Button, Checkbox, Color32, Direction, Label, Layout, RichText, TextEdit, Vec2};

use crate::{app_setup::GameLog, clock::get_date, egui::Ui, enums::Rating, json_file_operations::{save_to_file, search_for_game, Game}};

// 'Editing' Window GUI Code
impl GameLog {
    pub fn editing_gui (&mut self, ui: &mut Ui) 
    {
        let label_size= Vec2::new(ui.available_width(), 20.0); // IMPORTANT: Biggest Element needs to be the same width as the ui if centering vertically to ensure it's actually centered when resizing
        let input_box_size = Vec2::new(150.0, 20.0);
        let button_size = Vec2::new(100.0, 20.0);
        let increment_button_size = Vec2::new(30.0, 20.0);

        ui.add_space(5.0);

        ui.vertical_centered(|ui| {

            // Title
            ui.add_sized(label_size, Label::new(RichText::new("Enter Game You Are Editing")
                .size(20.0)) // This is the text size, not the allocated space size
            );

            ui.add_space(2.0);
            
            // Search Box
            ui.add_sized(input_box_size, TextEdit::singleline(&mut self.editing_search_game_name)
                    .hint_text("Game Name (< 50 Char)")
                    .char_limit(50)
                    .horizontal_align(Align::Center)
                    .desired_width(input_box_size.x)
            );

            ui.add_space(5.0);

            // Feedback Message (Only visible when giving users feedback on their actions)
            if !self.editing_search_feedback.is_empty()
            {
                ui.add_sized(label_size, Label::new(RichText::new(&self.editing_search_feedback)
                    .size(20.0)
                    .color(
                        if self.editing_search_error_confirmation == true {
                            Color32::RED
                        }
                        else if self.dark_mode { // Better Color For Dark Mode
                            Color32::GREEN
                        }
                        else { // Better Color For Light Mode
                            Color32::DARK_GREEN
                        }
                    ))
                );
                ui.add_space(5.0);
            }

            ui.add_space(2.0);

            // Search Button
            if ui.add_sized(button_size, Button::new("Search")).clicked() {
                self.editing_search_error_confirmation = true;

                match search_for_game(&self.game_file_contents, &self.editing_search_game_name) // Make sure game isn't already in the log (Remind users who may have forgot)
                    {
                        Ok(index) =>  {
                            self.editing_search_feedback = format!("Game Found!");
                            self.editing_search_error_confirmation = false;
                            self.editing_selected_index = index
                        },
                        Err(_) => { 
                            self.editing_search_feedback = format!("Game Not Found!");
                        }
                    };
            }


            // Once a game is found, Expand the window and add options for editing game information
            // This also allows us to hide these assets should the user search for an invalid game after, as well as seamlessly updating the current information
            if !self.editing_search_error_confirmation {
                ui.add_space(20.0);

                let game: &mut Game = &mut self.game_file_contents[self.editing_selected_index];

                let mut container_width = 50.0 + 50.0 + 10.0 + input_box_size.x + 10.0; // Width to hold elements being held in a horizontal container (Updates with each Layout to match the new space needed)

                // NAME
                ui.allocate_ui_with_layout(
                    Vec2::new(container_width, 20.0),
                    Layout::left_to_right(Align::Center),
                    |ui| {
                        // Game Name
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
                                TextEdit::singleline(&mut self.edit_game_name)
                                    .hint_text("New Game Name (< 50 Char)")
                                    .char_limit(50)
                            );

                            ui.add_space(10.0);
                        });
                });
                
                ui.add_space(5.0);

                // RATING
                container_width = 50.0 + 50.0 + 10.0 + input_box_size.x + 10.0;

                ui.allocate_ui_with_layout(
                    Vec2::new(container_width, 20.0),
                    Layout::left_to_right(Align::Center),
                    |ui| {    
                        ui.add_sized(Vec2::new(50.0, 20.0),
                            Label::new(RichText::new("Current Rating: "))
                        );

                        ui.add_sized(Vec2::new(50.0, 20.0),
                            Label::new(RichText::new(&game.rating.to_string())
                            .strong())
                        );

                        ui.add_space(10.0);

                        ui.add_sized(input_box_size,
                            TextEdit::singleline(&mut self.edit_game_rating)
                                .hint_text("Number between 1-5")
                                .char_limit(1)
                        );
                        ui.add_space(10.0);
                    });

                ui.add_space(5.0);

                // NOTES
                container_width = 50.0 + 50.0 +  10.0 + input_box_size.x + 10.0;

                ui.allocate_ui_with_layout(
                    Vec2::new(container_width, 20.0),
                    Layout::left_to_right(Align::Center),
                    |ui| { 
                        ui.add_sized(Vec2::new(50.0, 20.0),
                            Label::new(RichText::new("Current Notes: "))
                        );

                        // Truncating Long Game Notes here, people can just check the notes themselves manually if they want to see it all
                        let notes = Self::truncate_game_attributes(&game.notes, 10);
                        
                        ui.add_sized(Vec2::new(50.0, 20.0),
                            Label::new(RichText::new(format!("{}", notes))
                            .strong())
                        );

                        ui.add_space(10.0);

                        ui.add_sized(input_box_size,
                            TextEdit::singleline(&mut self.edit_game_notes)
                                .hint_text("Enter your thoughts"));

                        ui.add_space(10.0);
                });

                ui.add_space(20.0);

                // INCREMENTOR
                container_width = 10.0 + 10.0 + increment_button_size.x + 10.0 + 20.0 + 10.0 + increment_button_size.x + input_box_size.x;

                ui.allocate_ui_with_layout(
                    Vec2::new(container_width, 20.0),
                    Layout::centered_and_justified(Direction::LeftToRight),
                    |ui| { 

                        ui.horizontal(|ui| {
                            ui.add_sized(Vec2::new(50.0, 20.0),
                                Label::new(RichText::new("Increment Times Played: "))
                            );

                            ui.add_space(10.0);

                            if ui
                                .add_sized(increment_button_size, Button::new("-").min_size(increment_button_size))
                                .clicked()
                            {
                                self.increment_times_played = self.increment_times_played.saturating_sub(1); // Saturating_Sub makes it so the number cant go below 0
                            }

                            ui.add_space(10.0);

                            ui.add_sized(
                                Vec2::new(20.0, 20.0),
                                Label::new(format!("{}", self.increment_times_played)),
                            );

                            ui.add_space(10.0);

                            if ui
                                .add_sized(increment_button_size, Button::new("+").min_size(increment_button_size))
                                .clicked()
                                {
                                self.increment_times_played += 1;
                            }

                            ui.add_space(input_box_size.x); // This will allow the general container to be the same approximate size as the other property changer rows. This allows us to put the Title and incrementor somewhat in line with the rest so it looks aesthetically pleasing. Obviously not exact but still looks good
                        });
                });

                ui.add_space(20.0);
                
                // Confirmation/Error Messages are displayed here
                if !self.editing_feedback_message.is_empty() {
                    ui.add_sized(Vec2::new(200.0, 20.0),
                            Label::new(RichText::new(&self.editing_feedback_message)
                            .color(
                                if self.error_confirmation {
                                    Color32::RED
                                }
                                else if self.dark_mode { // Better Color For Dark Mode
                                    Color32::GREEN
                                }
                                else { // Better Color For Light Mode
                                    Color32::DARK_GREEN
                                }
                            )
                        )
                    );
                }

                // Confirma Edit
                container_width = 30.0 + button_size.x + input_box_size.x;

                ui.allocate_ui_with_layout(
                    Vec2::new(container_width, 20.0),
                    Layout::left_to_right(Align::Center),
                    |ui| {

                            ui.add_sized(Vec2::new(20.0, 20.0), Checkbox::new(&mut self.checked, "Are you sure you want to commit these edits?"));

                            self.enabled = self.checked; // Again same variable, just different for readability

                            ui.add_space(20.0);

                            ui.add_enabled_ui(self.enabled, |ui| {
                               if ui.add_sized(button_size, Button::new("Confirm"))
                                    .clicked() {
                                        self.error_confirmation = true; // Reset Error Colour (so i dont have to set it for every possible error message and only for the success message)

                                        let game: &mut Game = &mut self.game_file_contents[self.editing_selected_index];

                                        if self.edit_game_name.is_empty() && self.edit_game_rating.is_empty() && self.edit_game_notes.is_empty() && (self.increment_times_played == 0) {
                                            self.editing_feedback_message = format!("Please Enter an Edit");
                                        }
                                        else {
                                            if !self.edit_game_name.is_empty() {
                                                game.name = self.edit_game_name.clone();
                                            }

                                            if !self.edit_game_rating.is_empty() {
                                                match self.edit_game_rating.trim().parse::<u8>()
                                                {
                                                    Ok(parsed) => 
                                                        if let Some(parsed_rating) = Rating::from_u8(parsed) {
                                                            game.rating = parsed_rating;
                                                            parsed_rating
                                                        }
                                                        else{ 
                                                            println!("Invalid Rating (INVALID NUMBER)"); // Just printing these to the terminal. Users will be able to visually see their invalid ratings havent been added
                                                            game.rating // Just use current game rating (Unchanged)
                                                        },
                                                    Err(_) => {
                                                        println!("Invalid Rating (ENTER A NUMBER)"); 
                                                        game.rating
                                                    }
                                                };
                                            }

                                            if !self.edit_game_notes.is_empty() { // Save Notes
                                                game.notes = self.edit_game_notes.clone();
                                            }

                                            
                                            game.last_playthrough = get_date().to_string(); // Get current date to save for last playthrough


                                            game.times_played = self.increment_times_played + game.times_played; // Save New times played

                                            // Save Edits
                                            self.editing_feedback_message = match save_to_file(&mut self.game_file_contents){
                                                Ok(_) => { 
                                                    self.error_confirmation = false;
                                                    format!("Edits Added")
                                                },
                                                Err(_) => format!("Error Saving to File"),
                                            };
                                        }
                                    }
                            });
                            ui.add_space(input_box_size.x); // This will allow the general container to be the same approximate size as the other property changer rows. This allows us to put the Title and incrementor somewhat in line with the rest so it looks aesthetically pleasing. Obviously not exact but still looks good
                    },
                );
            };
        });
    }

    // Make long strings that will be displayed smaller with an elipsis at the end
    fn truncate_game_attributes (attribute: &str, desired_length: usize) -> String
    { 
        let mut attribute_string = attribute.to_string();

        if attribute.len() > desired_length {

            attribute_string.truncate(desired_length);
            return attribute_string + "..."
        }

        return attribute_string
    }
}
