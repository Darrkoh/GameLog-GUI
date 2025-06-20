use eframe::{
    egui::{self, CentralPanel, Context}, run_native, App, Frame, NativeOptions
};

struct GameLog; // The app

// Define the app's behvaiour and contents
impl App for GameLog {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.label("Hello World"); // Text displayed in the app window
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let native_options = NativeOptions::default(); // Can be edited to change the windows display options (Size and VSync for example but we dont need that for my app)

    // Uses a closure '| |' to execute a function which creates the app window when called. This is basically a lamda. Here 'cc' is the parameter and can be used to configure the app on startup (Persist Storage, Light/Dark mode, etc.))
    let app_creator = Box::new(|cc: &eframe::CreationContext|  { 
            cc.egui_ctx.set_visuals(egui::Visuals::dark()); // Dark Mode Theme for the app. I plan to add a settings menu eventually so this can be toggled
            Ok(Box::new(GameLog) as Box<dyn App>) // Creates the app through creating a window using the GameLog data 
        }
    );

    run_native("Game Log", native_options, app_creator)?; // Starts the desktop app

    Ok(())
}