use eframe::{egui::{self, CentralPanel, Context, FontId, Layout, RichText, TextEdit, TextureHandle, TopBottomPanel}, App, Frame};
use image::GenericImageView;


/// Stores the application's state, including UI settings and user input.
/// 
/// This struct holds texture assets, toggles for dark mode, and the current game search query.
pub struct GameLog { 
    pub dark_mode: bool, 
    pub assets: Vec<egui::TextureHandle>,
    pub search_game: String
}

/// App settings on startup
impl GameLog {
    /// Constructor to create app and load assets
    /// 
    ///  - 'dark_mode': Used for toggling dark mode
    ///  - 'assets': Calls a method which loads assets in the 'assets' folder and turns them into textures to be used in the app
    ///  - 'search_game': Used at startup to hold the contents of the app search bar
    pub fn startup(ctx: &egui::Context)  -> Self {
        let assets = Self::load_assets_from_bytes(ctx);
        let search_game: String = String::new(); 

        Self { dark_mode: true, 
                assets,
                search_game
            }
    }

    /// Load embedded image assets into the app context upon startup
    /// 
    /// This results in not needing to constantly load in textures whenever we want to use them, saving GPU resources 
    fn load_assets_from_bytes(ctx: &egui::Context) -> Vec<TextureHandle> { 

        let asset_path = vec![
            ("moon", &include_bytes!("../assets/Moon_Crescent.png")[..]), // [..] used to convert a fixed size array into a dynamic reference
            ("sun", &include_bytes!("../assets/Sun.png")[..])
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

/// Define the app's Behaviour and contents
impl App for GameLog {

    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        TopBottomPanel::top("top_panel").exact_height(40.0).show(ctx, |ui| {
            // Set the correct image depending on whether the appearance is currently light mode or dark mode
            let appearance_texture = if self.dark_mode {&self.assets[1]} else {&self.assets[0]};

            // Content Sizes
            let appearance_size = egui::Vec2::new(20.0, 20.0); // Image Size

            // Image Sizeing so they're not taking up the whole goddamn screen
            let sized_appearance_texture = egui::Image::new(appearance_texture).max_size(appearance_size);

            if self.dark_mode {
                ctx.set_visuals(egui::Visuals::dark());
            }
            else {
                ctx.set_visuals(egui::Visuals::light());
            }

            // TOP BAR CONTENT
            ui.horizontal_centered(|ui|{
                // Nav Buttons
                ui.add_sized(appearance_size, egui::Button::new("Add"));
                ui.add_sized(appearance_size, egui::Button::new("Remove"));

                // Dark/Light mode toggle
                ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui|{
                    if ui.add_sized(appearance_size, egui::ImageButton::new(sized_appearance_texture))
                    .clicked() {
                        self.dark_mode = !self.dark_mode;
                    }
                });     
            });
        });

        // Actual Contents of the window
        CentralPanel::default().show(ctx, |ui| {

            // Variables for hiding ui elements depending on window size
            let available_width = ui.available_size().x;
            let min_width_for_search = 800.0; // Widest Letters are W and M, this width is in place to hide the Feedback search message before a 50 character message of W/M would be overlapped by the Dark/Light Mode Button
            let search_size = egui::Vec2::new(300.0, 30.0);

            ui.vertical_centered(|ui| {
                ui.add_space(30.0);
                ui.label(RichText::new("WELCOME TO YOUR GAME LOG!").font(FontId::proportional(60.0)).underline());
                ui.add_space(20.0);
        
        
                // Searching for a Game
                ui.vertical_centered(|ui|{
                    ui.label("Search:"); // Affordance, telling users what the search bar is for
                    
                    let response = ui.add_sized(search_size, TextEdit::singleline(&mut self.search_game)// Save user's search input
                        .hint_text("Enter the Game's Name")
                        .char_limit(50) // Enforce a 50 character search limit so users can't break the layout :D 
                        .frame(true) // Frame appears upon cursor hover
                        .horizontal_align(egui::Align::Center)
                        .vertical_align(egui::Align::Center)
                    ); 

                    // Hide feedback message if user starts making the window smaller and theres a long search message
                    if (available_width >= min_width_for_search) || (self.search_game.len() <= 23)
                    {
                        // Tell users their input has been detected (Feedback)
                        let feedback_message = egui::Label::new(
                            RichText::new(format!("Currently Searching For: {}", self.search_game))
                        ).wrap_mode(egui::TextWrapMode::Truncate); // No wrap as it isnt needed and results in pixel overflow

                        if !self.search_game.is_empty() {
                            ui.add(feedback_message);
                        }
                    }

                    // If Enter Key is pressed, execute a "Search File" function which will search the game log for the game name inputted.
                    // Right now i dont have this method so nothing really happens lol
                    if response.lost_focus() && ui.input(|input| input.key_pressed(egui::Key::Enter ))
                    {
                        self.search_game = String::new(); // Clear input, ready for next input
                    }
                });

                ui.add_space(10.0);

                egui::Frame::default()
                    .fill(ui.visuals().extreme_bg_color)
                    .corner_radius(egui::CornerRadius::same(10))
                    .stroke(egui::Stroke::new(1.0, egui::Color32::BLACK))
                    .show(ui, |ui| {
                        let frame_size = egui::Vec2::new(600.0, 800.0);
                    
                        // Content of frame. Will contain Game Log Info
                        ui.set_min_size(frame_size); // Set Size of frame (Overritdes ui.vertical_centered in terms of taking up available space)
                        ui.set_max_size(frame_size); // Need to set both of these so frame will always be the same size no matter the content
                        
                        ui.label(RichText::new("PLACEHOLDER")
                            .size(20.0)
                            .strong()
                            .color(ui.visuals().text_color())
                        )
                    });
            });
        });
    }
}