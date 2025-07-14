use eframe::egui::{Button, Checkbox, Color32, Label, RichText, TextEdit, Vec2};

use crate::{app_setup::GameLog, clock::get_date, egui::Ui, enums::Rating, json_file_operations::{save_to_file, search_for_game, Game}};

// "Adding" Window GUI Coded        

// Still implementing the GameLog
impl GameLog{
    pub fn adding_gui (&mut self, ui: &mut Ui) 
    {
        let label_size= Vec2::new(100.0, 20.0);
        let input_box_size = Vec2::new(150.0, 20.0);
        
        ui.add_space(5.0);

        ui.vertical(|ui| {

            ui.horizontal(|ui| {
                ui.add_sized(label_size,
                    Label::new(RichText::new("Game Name").strong())
                );

                ui.add_sized(input_box_size, TextEdit::singleline(&mut self.add_game_name)
                    .hint_text("Game Name (< 50 Char)")
                    .char_limit(50));
            });

            ui.add_space(2.0);

            ui.horizontal(|ui| {
                ui.add_sized(label_size,
                    Label::new(RichText::new("Game Rating").strong())
                );

                ui.add_sized(input_box_size, TextEdit::singleline(&mut self.add_game_rating)
                    .hint_text("Number between 1-5")
                    .char_limit(1)
                );
            });

            ui.add_space(2.0);

            ui.horizontal(|ui| {
                ui.add_sized(label_size,
                    Label::new(RichText::new("Game Notes").strong())
                );

                ui.add_sized(input_box_size, TextEdit::singleline(&mut self.add_game_notes)
                    .hint_text("This is optional")
                );
            });

            ui.add_space(2.0);  

            // Confirmation/Error Messages are displayed here
            if !self.adding_feedback_message.is_empty() {
                ui.add_sized(Vec2::new(200.0, 20.0),
                        Label::new(RichText::new(&self.adding_feedback_message)
                        .color(
                            if self.error_confirmation == true {
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
            }

            ui.horizontal(|ui| {
                ui.add_sized(label_size,
                    Checkbox::new(&mut self.checked, "Are you sure you wish to add this game?")
                );

                ui.add_space(5.0);  

                self.enabled = self.checked; // They are the same variable basically. Only seperate for purposes of readability
                
                ui.add_enabled_ui(self.enabled, |ui| {
                    if ui.add_sized(Vec2::new(150.0, 20.0), Button::new("Go"))
                        .clicked() {

                            self.error_confirmation = true; // Reset text colour to red, will be changed back to green if the process is successful again 

                            if !self.add_game_name.is_empty() && !self.add_game_rating.is_empty() // Making sure the user has inputted into the game name and ratings box (Notes is optional)
                            {
                                match search_for_game(&self.game_file_contents, &self.add_game_name) // Make sure game isn't already in the log (Remind users who may have forgot)
                                {
                                    Ok(index) => self.adding_feedback_message = format!{"Game is already in game log at index: {}", index},

                                    Err(_) => match self.add_game_rating.trim().parse::<u8>() // Parsing into a number and dealing with potential errors
                                            {
                                        Ok(parsed) => {
                                            if let Some(parsed_rating) = Rating::from_u8(parsed) { //Parsing into an enum

                                                let date = get_date();

                                                let game = Game {
                                                    name: self.add_game_name.clone(),
                                                    rating: parsed_rating,
                                                    times_played: 1,
                                                    last_playthrough: date.to_string(),
                                                    notes: self.add_game_notes.clone(),
                                                };

                                                self.game_file_contents.push(game); // Add it to the vector so we can easily save it to the JSON file

                                                match save_to_file(&self.game_file_contents)
                                                {
                                                    Ok(_) => {
                                                        println!("CREATED"); // Game is added to the game log (Terminal Message)
                                                        self.adding_feedback_message = format!("Game Added!"); // Remove any error messages previously acquired
                                                        self.error_confirmation = false;
                                                    },
                                                    Err(_) => self.adding_feedback_message = format!("There was an error when adding the game to the file"),
                                                };
                                            }
                                            else {
                                                self.adding_feedback_message = format!("Invalid number entered!\nTry again buddy");
                                            }
                                        },
                                        _ => self.adding_feedback_message = format!("Invalid rating entered!\nTry again buddy"), // Both this and the error on line 81 are because of a parse error, hence same error message
                                    }
                                }
                            } else {
                                self.adding_feedback_message = format!("You have missed some required boxes") // If users didn't fill in required boxes
                            }

                            // Clear all inout boxes once the button is clicked and operations have been executed
                            self.add_game_name.clear();
                            self.add_game_rating.clear();
                            self.add_game_notes.clear();
                        };
                });
            });
        });
    }
}
