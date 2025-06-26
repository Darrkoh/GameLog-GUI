// Crates //
use std::fs::OpenOptions;
use std::io::{BufReader, Write};
use std::fs::File;
use serde::{Deserialize, Serialize};
use anyhow::anyhow; // So i can have easy error handling with anyhow
use crate::enums::Rating;
use std::result::Result;

#[derive(Serialize, Deserialize, Clone)]
pub struct Game {
    pub name: String,
    pub rating: Rating,
    pub times_played: u8,
    pub last_playthrough: String,
    pub notes: String
}

// Read a parse JSON from text file into a vector 
pub fn reading_json() -> Vec<Game> // Result is wrapped around incase there is an error returned
{
    let file = File::open("src/GameLog.Json"); // Gets file contents

    let reader = match file {
        Ok(games) => BufReader::new(games),
        Err(_) => return Vec::new()
    };

    let game = serde_json::from_reader(reader); // Adds every JSON entry in the text file to a vector

    // Custom try catch statement using match. If an error is found, an empty vector will be produced, if not the returned vector held in the 'game' variable will be produced.
    // This is to make sure the program will run if an empty list is found (it won't if line 22 just propagates errors with ?)
    match game {
        Ok(games) => games, // Either return the parsed Json List, or an error 
        Err(_e) => Vec::new()
    }
}

// Create a Game and add it's Json data to the text file
pub fn save_to_file(game_log: &Vec<Game>) -> Result<(), Box<dyn std::error::Error>>
{

    // Serialising a the game_log into JSON and overwriting the previous file with this new data. It's literally the same with the new data added

     let new_json = serde_json::to_string_pretty(&game_log)?;

    // Write back to file (overwrite)
    let mut file = OpenOptions::new()
    .write(true)
    .truncate(true).
    open("src/GameLog.Json")?;
    file.write_all(new_json.as_bytes())?;

    Ok(())
}

pub fn search_for_game(game_log: &Vec<Game>, query: &str) -> Result<usize, anyhow::Error>
{
    let query_lower = query.to_lowercase(); // lowercase so it wont be case sensitive to users
    for (i, game) in game_log.iter().enumerate() {
        if game.name.to_lowercase() == query_lower{
            return Ok(i); // Game found
        }
    }
    Err(anyhow!("No Game")) // If no game name is found, return error
}
