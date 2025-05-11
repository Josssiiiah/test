// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri::{AppHandle, Manager, Result, Runtime, WebviewUrl};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn open_window<R: Runtime>(app: AppHandle<R>) -> Result<()> {

    // Check if the window already exists
    if let Some(window) = app.get_webview_window("test") {
        // If it exists, bring it to the front
        window.set_focus()?;
    } else {
        // If it doesn't exist, create it
        let builder = tauri::WebviewWindowBuilder::new(&app, "test", WebviewUrl::App("drag.html".into()))
            .title("Test Window")
            .inner_size(420.0, 300.0)
            .position(200.0, 200.0)
            .transparent(true) 
            .decorations(false) // No window decorations (title bar, etc.)
            .resizable(true)
            .skip_taskbar(true)
            .focused(true)
            .shadow(false)
            .always_on_top(true); // Let the user move it behind other windows

        // Create the window
        let _window = builder.build()?;
    }
    Ok(())
}

#[tauri::command]
async fn close_window<R: Runtime>(app: AppHandle<R>) -> Result<()> {
    if let Some(window) = app.get_webview_window("test") {
        window.close()?;
    }
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, open_window, close_window])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
