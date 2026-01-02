// Prevents additional console window on Windows in release, do not remove!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// This is the function Svelte will call
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You are running Path Studio on Ubuntu.", name)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet]) // Register the function here
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}