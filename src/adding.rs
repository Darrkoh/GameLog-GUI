use eframe::egui::{Checkbox, Label, RichText, Spacing, TextEdit, Vec2};

use crate::{app_setup::GameLog, egui::Ui};

// "Adding" Window GUI Coded        

impl GameLog{
    pub fn adding_gui (&mut self, ui: &mut Ui) 
    {
        let label_size= Vec2::new(100.0, 20.0);
        ui.vertical(|ui| {

            ui.horizontal(|ui| {
                ui.add_sized(label_size,
                    Label::new(RichText::new("Game Name").strong())
                );

                let game_name_response = ui.add_sized(Vec2::new(150.0, 20.0), TextEdit::singleline(&mut self.add_game_name));
            });

            ui.add_space(2.0);

            ui.horizontal(|ui| {
                ui.add_sized(label_size,
                    Label::new(RichText::new("Game Rating").strong())
                );

                let game_rating_response = ui.add_sized(Vec2::new(150.0, 20.0), TextEdit::singleline(&mut self.add_game_rating));
            });

            ui.add_space(2.0);

            ui.horizontal(|ui| {
                ui.add_sized(label_size,
                    Label::new(RichText::new("Game Notes").strong())
                );

                let game_notes_response = ui.add_sized(Vec2::new(150.0, 20.0), TextEdit::singleline(&mut self.add_game_notes));
            });

            ui.add_space(2.0);  

            ui.horizontal(|ui| {
                ui.add_sized(label_size,
                    Checkbox::new(&mut self.checked, "Are you sure you wish to add this game?")
                );
            });
        });
    }
}
