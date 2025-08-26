use std::fs;
use std::path::PathBuf;

use rand::rng;
use rand::seq::IndexedRandom;
use tauri::path::BaseDirectory;
use tauri::Manager;

#[tauri::command]
pub fn random_gif(app: tauri::AppHandle) -> Result<Vec<u8>, String> {
    // Resolve to resource folder in packaged app
    let resource_path = app
        .path()
        .resolve("assets/chief", BaseDirectory::Resource)
        .map_err(|e| format!("Failed to resolve resource: {}", e))?;

    // Collect all files
    let mut gifs = vec![];
    if let Ok(entries) = std::fs::read_dir(&resource_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().map(|e| e == "gif").unwrap_or(false) {
                gifs.push(path);
            }
        }
    }

    let chosen_gif = gifs
        .choose(&mut rng())
        .ok_or_else(|| format!("Chief is very sad! Something broke! {:?}", gifs))?
        .clone();

    let file = fs::read(chosen_gif).map_err(|e| format!("Failed to read GIF: {}", e));

    file
}

