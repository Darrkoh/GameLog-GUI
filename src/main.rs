pub mod app_setup;
use eframe::{run_native, App, NativeOptions};
use crate::app_setup::GameLog;

fn main() -> Result<(), eframe::Error> {
    let native_options = NativeOptions::default(); // Can be edited to change the windows display options (Size and VSync for example but we dont need that for my app)

    // Uses a closure '| |' to execute a function which creates the app window when called. This is basically a lamda. Here 'cc' is the parameter and can be used to configure the app on startup (Persist Storage, Light/Dark mode, etc.))
    let app_creator = Box::new(|cc: &eframe::CreationContext|  { 
            Ok(Box::new(GameLog::startup(&cc.egui_ctx)) as Box<dyn App>) // Creates the app through creating a window using the GameLog data 
        }
    );

    run_native("Game Log", native_options, app_creator)?; // Starts the desktop app

    Ok(())
}