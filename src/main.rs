use eframe::{
    egui::{self, CentralPanel, Context}, run_native, App, Frame, NativeOptions
};

// The app
struct GameLog { 
    dark_mode: bool, // Attribute for Toggling dark mode
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
        CentralPanel::default().show(ctx, |ui| {
            // Dark/Light mode toggle (Plan to move to somewhere else eventually)
            if self.dark_mode {
                ctx.set_visuals(egui::Visuals::dark());
            }
            else {
                ctx.set_visuals(egui::Visuals::light());
            }

            let label = if self.dark_mode { "Light Mode" } else { "Dark Mode" };
            
            if ui.button(label).clicked() {
                self.dark_mode = !self.dark_mode;
            }
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let native_options = NativeOptions::default(); // Can be edited to change the windows display options (Size and VSync for example but we dont need that for my app)

    // Uses a closure '| |' to execute a function which creates the app window when called. This is basically a lamda. Here 'cc' is the parameter and can be used to configure the app on startup (Persist Storage, Light/Dark mode, etc.))
    let app_creator = Box::new(|cc: &eframe::CreationContext|  { 
            Ok(Box::new(GameLog {dark_mode: true}) as Box<dyn App>) // Creates the app through creating a window using the GameLog data 
        }
    );

    run_native("Game Log", native_options, app_creator)?; // Starts the desktop app

    Ok(())
}