use std::time::{SystemTime, UNIX_EPOCH};
use chrono::prelude::*;

fn main() {
    // Get the current local date and time
    let local: DateTime<Local> = Local::now();

    // Print the greeting
    println!("Hello ASL!");

    // Print the current date in the format "YYYY-MM-DD"
    println!("{}", local.format("%Y-%m-%d"));
}
