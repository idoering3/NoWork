use keyring_core::{Entry};

pub fn init_keyring() {
    keyring::use_native_store(false).expect("Failed to initialize keyring");
}

#[tauri::command]
pub fn save_credentials(email: &str, password: &str) -> Result<(), String> {
    let entry = Entry::new("my-app", email)
        .map_err(|e| e.to_string())?;
    entry.set_password(password)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn load_credentials(email: &str) -> Result<String, String> {
    let entry = Entry::new("my-app", email)
        .map_err(|e| e.to_string())?;
    entry.get_password()
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_credentials(email: &str) -> Result<(), String> {
    let entry = Entry::new("my-app", email)
        .map_err(|e| e.to_string())?;
    entry.delete_credential()
        .map_err(|e| e.to_string())
}