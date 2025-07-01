use eframe::egui::{Label, RichText, Vec2};

use crate::{app_setup::GameLog, egui::Ui};

// 'Editing' Window GUI Code
impl GameLog {
    pub fn editing_gui (&mut self, ui: &mut Ui) 
    {
        let search_label_size= Vec2::new(200.0, 20.0);

        ui.vertical(|ui| {
            ui.add_sized(search_label_size, Label::new(RichText::new("Enter Game You Are Editing")))
        });
        
    }
}