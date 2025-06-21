use eframe::{egui::{self, CentralPanel, Context, TopBottomPanel, Vec2}, App, Frame};

// The app
pub struct GameLog { 
    pub dark_mode: bool, // Attribute for Toggling dark mode
}

// Default app settings
impl Default for GameLog {
    fn default() -> Self {
        Self { dark_mode: true }
    }
}
// Define the app's behvaiour and contents
impl App for GameLog {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        TopBottomPanel::top("top_panel").min_height(30.0).show(ctx, |ui| {
            // Dark/Light mode toggle (Plan to move to somewhere else eventually) \\
            if self.dark_mode {
                ctx.set_visuals(egui::Visuals::dark());
            }
            else {
                ctx.set_visuals(egui::Visuals::light());
            }

            // Otherwise Text in button isn't centered. This also puts buttons to the right of the Top Bar
            ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::RightToLeft), |ui| {

                // Purpose of this is so we can have multiple buttons aligned horizontally. Without it, the first button created takes up the whole top bar with no room for the others
                ui.horizontal(|ui|
                    {
                    let label = if self.dark_mode { "Light Mode" } else { "Dark Mode" };

                    if ui.add(egui::Button::new(label)
                        .min_size(Vec2::new(60.0, 40.0 )))
                    .clicked() {
                        self.dark_mode = !self.dark_mode;
                    }
                });
            });
        });

        CentralPanel::default().show(ctx, |ui| {
            ui.label("Hello World!");
        });
    }
}