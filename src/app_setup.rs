use eframe::{egui::{self, CentralPanel, Context, FontId, Layout, RichText, TextEdit, TextureHandle, TopBottomPanel}, App, Frame};
use image::GenericImageView;
use crate::{enums::WindowOpened, json_file_operations::{reading_json, search_for_game, Game}, removing::removing_gui};


/// Stores the application's state, including UI settings and user input.
/// 
/// This struct holds texture assets, toggles for dark mode, and the current game search query.
pub struct GameLog { 
    // General Settings/File Importing
    pub dark_mode: bool, 
    pub assets: Vec<egui::TextureHandle>,
    pub game_file_contents: Vec<Game>, // Grabbing Gamelog details from the JSON file

    // Search Game
    pub search_game: String,
    pub last_searched_term: String, // Stores last input of "search_game" so input feedback messages can linger after search_game is cleared
    pub invalid_search_message: String, // Display a message telling users their game isnt found. This shouldn't be updated each frame but needs to be global hence its a field
    pub search_result: Option<Vec<Game>>, // Store search results for games

    pub(crate) // Opening External Windows
    open_window: bool, // When this is true, code will execute to open a new window in the app
    current_window_opened: WindowOpened,

    // Global Variables to be used all over the program 
    pub checked: bool, // Checkbox variable
    pub enabled: bool, // This isnt needed as i could replace it with checked, however i'm using it as an explain variable so the codes easier to read
    pub error_confirmation: bool, // For changing text colours and stuff in error messages. Will be true when no error

    // Adding
    pub add_game_name: String,
    pub add_game_rating: String,
    pub add_game_notes: String,
    pub add_feedback_message: String,


    // Removing


    // Editing
    pub edit_game_name: String,
    pub edit_game_rating: String,
    pub edit_game_notes: String,
    pub increase_times_played: i32,
    pub editing_search_game_name: String,
    pub editing_search_feedback: String
}

/// App settings on startup
impl GameLog {
    /// Constructor to create app and load assets
    /// 
    ///  - 'dark_mode': Used for toggling dark mode
    ///  - 'assets': Calls a method which loads assets in the 'assets' folder and turns them into textures to be used in the app
    ///  - 'search_game': Used at startup to hold the contents of the app search bar
    pub fn startup(ctx: &egui::Context)  -> Self {
        // General Settings/File Importing
        let assets = Self::load_assets_from_bytes(ctx);
        let game_file_contents = reading_json(); // Grabbing Gamelog details from the JSON file

        // Main Menu Searching
        let search_game: String = String::new(); 
        let last_searched_term: String = String::new(); 
        let invalid_search_message = String::new(); 
        

        // Opening External Windows
        let open_window = false;
        let current_window_opened = WindowOpened::Default; // Tracks current window so the program knows what window to open

        // Global Check Box Variable and Enable Button Variable (Wont be used for more than one page at a time)
        let checked = false;
        let enabled = checked;

        // Adding
        let add_game_name = String::new();
        let add_game_rating = String::new();
        let add_game_notes = String::new();
        let add_feedback_message = String::new();
        let error_confirmation = false; // Will be used for telling the feedback message what colour to be

        // Removing

        // Editing
        let edit_game_name= String::new();
        let edit_game_rating= String::new();
        let edit_game_notes= String::new();
        let increase_times_played = 0;
        let editing_search_game_name = String::new();
        let editing_search_feedback = String::new();

        
        Self { dark_mode: true, 
                assets,
                search_game,
                last_searched_term,
                invalid_search_message,
                search_result: None,
                game_file_contents,
                open_window,
                current_window_opened,
                checked,
                enabled,
                add_game_name,
                add_game_rating,
                add_game_notes,
                add_feedback_message,
                error_confirmation,
                edit_game_name,
                edit_game_rating,
                edit_game_notes,
                increase_times_played,
                editing_search_game_name,
                editing_search_feedback
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
                if ui.add_sized(appearance_size, egui::Button::new("Add"))
                .clicked() {
                    self.open_window = true;
                    self.current_window_opened = WindowOpened::Adding;
                };

                if ui.add_sized(appearance_size, egui::Button::new("Edit"))
                .clicked() {
                    self.open_window = true;
                    self.current_window_opened = WindowOpened::Editing;
                };


                if ui.add_sized(appearance_size, egui::Button::new("Remove"))
                .clicked() {
                    self.open_window = true;
                    self.current_window_opened = WindowOpened::Removing;
                };


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
            let available_width = ui.available_size().x; // Used for hiding elements if screen is shrunk to a certain point
            let min_width_for_search = 600.0; // Widest Letters are W and M, this width is in place to hide the Feedback search message before a 50 character message of W/M would be overlapped by the Dark/Light Mode Button
            let search_size = egui::Vec2::new(300.0, 30.0);

            

            // Game Log Display Variables
            let mut game_log_display = format!("The Game Log is empty :/"); // Message for when the game log is empty

            ui.vertical_centered(|ui| {
                ui.add_space(30.0);
                ui.label(RichText::new("WELCOME TO YOUR GAME LOG!").font(FontId::proportional(60.0)).underline());
                ui.add_space(20.0);
        
        
                // Searching for a Game
                ui.vertical_centered(|ui|{
                    ui.label("Search:"); // Affordance, telling users what the search bar is for
                    
                    let search_response = ui.add_sized(search_size, TextEdit::singleline(&mut self.search_game)// Save user's search input
                        .hint_text("Enter the Game's Name")
                        .char_limit(50) // Enforce a 50 character search limit so users can't break the layout :D 
                        .frame(true) // Frame appears upon cursor hover
                        .horizontal_align(egui::Align::Center)
                        .vertical_align(egui::Align::Center)
                    ); 

                    // If Enter Key is pressed, execute a "Search File" function which will search the game log for the game name inputted.
                    // Right now i dont have this method so nothing really happens lol
                    if search_response.lost_focus() && ui.input(|input| input.key_pressed(egui::Key::Enter ))
                    { 

                        self.last_searched_term = self.search_game.clone(); // Save the users input for message displaying 

                        self.search_result = match search_for_game(&self.game_file_contents, &self.last_searched_term)
                        {
                            Ok(game_index) => {
                                self.invalid_search_message.clear();
                                Some(vec![self.game_file_contents[game_index].clone()]) // Save the game found to a field
                            }
                            Err(_) =>  {
                                self.invalid_search_message = format!("Invalid Game"); // Display this message to tell users the game doesn't exist in the game log
                                None // Contents of search_result is reset, displaying the whole list again
                            }
                        };
                        
                        self.search_game.clear(); // Clear input, ready for next input
                    }

                    // Hide feedback message if user starts making the window smaller and theres a long search message
                    if (available_width >= min_width_for_search) || (self.search_game.len() <= 30)
                    {
                        // Tell users their input has been detected (Feedback)
                        let feedback_message = egui::Label::new(
                            RichText::new(format!("Currently Searching For: {}", self.search_game))
                        ).wrap_mode(egui::TextWrapMode::Truncate); // No wrap as it isnt needed and results in pixel overflow

                        if !self.search_game.is_empty() {
                            ui.add(feedback_message);
                        }

                        if !self.invalid_search_message.is_empty() {
                            ui.label(RichText::new(&self.invalid_search_message).color(egui::Color32::RED));
                        }
                    }
                });
                

                ui.add_space(10.0);

                egui::ScrollArea::vertical().auto_shrink([false; 2]).show(ui, |ui| { // Allow horizontal & vertical growth
                    egui::Frame::default()
                        .fill(ui.visuals().extreme_bg_color)
                        .corner_radius(egui::CornerRadius::same(10))
                        .stroke(egui::Stroke::new(1.0, egui::Color32::BLACK))
                        .show(ui, |ui| {

                            // Content of frame. Will contain Game Log Info
                            ui.set_width(600.0); // Set width so the frame doesnt take up the whole panel lol
                            
                            if !self.game_file_contents.is_empty() {

                                // If there is a valid search result
                                if !self.search_result.is_none() {

                                    let game_found = self.search_result.as_ref().unwrap();
                                    let game = &game_found[0];
                                    game_log_display = format!(
                                        "Name: {}\nRating: {}\nTimes Played: {}\nLast Playthrough: {}\nNotes: {}\n\n",
                                        game.name,
                                        game.rating,
                                        game.times_played,
                                        game.last_playthrough,
                                        game.notes
                                    );
                                    // Display Search Results
                                    ui.label(RichText::new(&game_log_display)
                                            .size(20.0)
                                            .strong()
                                            .color(ui.visuals().text_color())
                                        );
                                }

                                // Display whole list if no search result, and data is in JSON file
                                else {
                                    // Create and display a label for every game in the game log in a structured and consistent manner
                                    for (i, game) in self.game_file_contents.iter().enumerate() {
                                        game_log_display = format!("Index: {} \nName: {} \nRating: {} \nTimes Played: {} \n Last Playthrough: {} \nNotes: {}\n\n",
                                            i,
                                            game.name,
                                            game.rating,
                                            game.times_played,
                                            game.last_playthrough,
                                            game.notes
                                        );
                                        // This is needed to print out every game in the list, as just using one outside the loop results in variable overwriting, so the label wont display every game
                                        ui.label(RichText::new(&game_log_display)
                                            .size(20.0)
                                            .strong()
                                            .color(ui.visuals().text_color())
                                    );
                                    }   
                                }
                            }
                            else {
                                // Will Automatically print a "No Games Found Message" if no games are found
                                ui.label(RichText::new(&game_log_display)
                                            .size(20.0)
                                            .strong()
                                            .color(ui.visuals().text_color())
                                        );
                            }
                        });
                });
            });

            // Opening another window when a button is pressed
            if self.open_window {

                let mut open_window = self.open_window; // Pass the self variable by val as we cannot borrow a mutable and immutable referencer of a self variable at the same time

                match self.current_window_opened // Each match statement will execute GUI code in the respective file for each window's display
                {
                    WindowOpened::Adding => { 
                            egui::Window::new("Adding Games")
                            .open(&mut open_window)
                            .show(ctx, |ui| {
                                self.adding_gui(ui)
                            });
                    },
                    WindowOpened::Editing => {
                            egui::Window::new("Editing Game")
                                .min_width(300.0)
                                .open(&mut open_window)
                                .show(ctx, |ui| {
                                        self.editing_gui(ui)
                                });
                    },
                    WindowOpened::Removing => {
                            egui::Window::new("Removing Games")
                                .open(&mut self.open_window)
                                .show(ctx, |ui| {
                                        removing_gui(ui)
                                });
                    },
                    WindowOpened::Default => { // This Will never be reached as it just exists as a default value
                            println!("All External Windows Closed")
                    },
                };
                self.open_window = open_window; // Copy contents entered in the adding window to the self variable for consistency
            }

            // Keep global variables synced when pages are closed
            if !self.open_window
            {
                self.current_window_opened = WindowOpened::Default;
                self.checked = false; // This also changes the enabled variable
                self.error_confirmation = false; // Error Messages will be red again (Default)
                self.add_feedback_message.clear();;
                self.editing_search_feedback.clear();
            }
        });
    }
}