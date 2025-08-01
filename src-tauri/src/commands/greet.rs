use tauri::{Manager, path::BaseDirectory };
use serde::{ Deserialize, Serialize };
use chrono::{ Datelike, Local, Weekday };
use rand::{ rng };
use std::collections::HashMap;
use rand::seq::IndexedRandom;

#[derive(Debug, Deserialize, Serialize)]
struct Greetings {
    #[serde(flatten)]
    greetings: HashMap<String, Vec<String>>,
}

#[tauri::command]
pub fn greet(app: tauri::AppHandle) -> Result<String, String> {
    // Try to get the app's data directory
    let resource_path = app.path().resolve("assets/greeting.json", BaseDirectory::Resource).map_err(|e| format!("Failed to resolve resource: {}", e))?;

    let json_file = std::fs::File::open(&resource_path).map_err(|e| format!("Failed to read greeting.json at {:?}", e))?;

    let greetings_data: Greetings = serde_json::from_reader(&json_file).map_err(|e| format!("Failed to parse to greetings data {:?}", e))?;

    let today: Weekday = Local::now().weekday();

    let day_name = match today {
        Weekday::Mon => "Monday",
        Weekday::Tue => "Tuesday",
        Weekday::Wed => "Wednesday",
        Weekday::Thu => "Thursday",
        Weekday::Fri => "Friday",
        Weekday::Sat => "Saturday",
        Weekday::Sun => "Sunday",
    };

    let greeting_for_day: &Vec<String> = greetings_data.greetings.get(day_name)
        .ok_or_else(|| format!("No greetings found for day: {}", day_name))?;
    
    let chosen_greeting: &String = greeting_for_day.choose(&mut rng())
        .ok_or_else(|| format!("No greetings available for {}", day_name))?;

    Ok(format!("{}", chosen_greeting))
}
