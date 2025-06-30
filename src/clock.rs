use chrono::{Local, NaiveDate};

pub fn get_date() -> NaiveDate {
    let today = Local::now().date_naive(); // Gets the local date (e.g., 2025-06-12)
    println!("Today's date is: {}", today);
    return today
}