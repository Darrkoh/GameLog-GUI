use eframe::{egui::{self, CentralPanel, ColorImage, Context, FontId, ImageButton, RichText, TextureHandle, TextureOptions, TopBottomPanel, Vec2}, epaint, App, Frame};
use image::GenericImageView;


// The app
pub struct GameLog { 
    pub dark_mode: bool, // Attribute for Toggling dark mode
    pub assets: Vec<egui::TextureHandle>
}

// App settings on startup
impl GameLog {
    // Constructor to create app and load assets
    pub fn startup(ctx: &egui::Context)  -> Self {
        let assets = Self::load_assets_from_bytes(ctx); // Load in app assets upon app startup
        Self { dark_mode: true, 
                assets
            }
    }

    // This function is implemented so all assets used in the app will be loaded in upon app startup and never need to be loaded up again, saving GPU resources and increasing processing speed
    fn load_assets_from_bytes(ctx: &egui::Context) -> Vec<TextureHandle> {

        let asset_path = vec![
            ("moon", include_bytes!("../assets/Moon_Crescent.png"))
            // ("sun", include_bytes!("Add sun path when its added to assets"))
        ];

        asset_path
        .into_iter() // Do for each asset
        .map(|(name, bytes)| {

            let image = image::load_from_memory(bytes).expect("Failed to load image"); // Load the image from memory or return an error message should this fail
            let size = image.dimensions(); // Size the image
            let rgba = image.to_rgba8(); // Saves Image Colour Data
            let pixels = rgba.as_flat_samples(); // Saves Pixel and MetaData.

            // Create the asset for use in the app using raw pixel data
            let color_image = egui::ColorImage::from_rgba_unmultiplied(
                [size.0 as usize, size.1 as usize],
                pixels.as_slice(),
            );

            // Load the asset into the app
            ctx.load_texture(name, color_image, egui::TextureOptions::default())
        })
        .collect() // Once all iterations are complete, turn it back into a Collection (Vector)
    }
}

// Define the app's behvaiour and contents
impl App for GameLog {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        TopBottomPanel::top("top_panel").min_height(30.0).show(ctx, |ui| {
            // Dark/Light mode toggle (Plan to move to somewhere else eventually) \\
            let dark_texture = &self.assets[0];
            let size = egui::Vec2::new(35.0, 35.0);

            // Create a SizedTexture from TextureHandle and size
            let sized_dark_texture = egui::Image::new(dark_texture).max_size(size);

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

                    if ui.add( egui::ImageButton::new(sized_dark_texture))
                    .clicked() {
                        self.dark_mode = !self.dark_mode;
                    }
                });
            });
        });

        // Contents of the window
        CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(30.0);
                ui.label(RichText::new("WELCOME TO YOUR GAME LOG!").font(FontId::proportional(60.0)).underline());
            });
        });
    }
}