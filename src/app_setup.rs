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

            // TOP BAR BUTTONS
            ui.horizontal(|ui| {

                // Buttons are set to appear at the left most available point
                // UNCOMMENT TO ADD A BUTTON TO APP BAR WITH SPACER
                    // ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| { });
                    // ui.add_space(ui.available_width());

                // Dark mode toggle is an exception and is set to the end of the top bar at all times. This is because it will be familiar for users as many other websites do this
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let label = if self.dark_mode { "Light Mode" } else { "Dark Mode" };

                    if ui.add(egui::Button::new(label)
                        .min_size(Vec2::new(100.0, 40.0 )))
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