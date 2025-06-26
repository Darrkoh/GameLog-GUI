use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Serialize, Deserialize)]
#[repr(u8)] // Rating is represented as an unsigned 8 bit integer in memory
#[serde(rename_all = "lowercase")] // Change all Enum Values to be lowercase for consistency
pub enum Rating {
    One = 1, // Assign each Enum a u8 value for mapping
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5
}

impl Rating {
    
// pub fn to_u8(&self) -> u8 // Parameter is the rating enum (Self allows for us to reference the object we're implementing on)
// {
//     return *self as u8; // We dont need a map here as I already mapped it out inb the enum
// }
    
    pub fn from_u8(value: u8) -> Option<Rating> // Parameter is the rating enum (Self allows for us to reference the object we're implementing on)
    {
        match value {
            1 => Some(Rating::One),
            2 => Some(Rating::Two),
            3 => Some(Rating::Three),
            4 => Some(Rating::Four),
            5 => Some(Rating::Five),
            _ => None
        }
    }
}

// So we can display the enum
impl fmt::Display for Rating {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Rating::One => "1",
            Rating::Two => "2",
            Rating::Three => "3",
            Rating::Four => "4",
            Rating::Five => "5",
        };
        write!(f, "{}", s)
    }
}