use tauri::{Manager, generate_handler, WindowBuilder, WindowUrl};
use std::fs;

#[tauri::command]
fn open_document(filepath: String) -> Result<String, String> {
    fs::read_to_string(&filepath).map_err(|e| e.to_string())
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            window.set_decorations(false).unwrap();
            Ok(())
        })
        .invoke_handler(generate_handler![open_document])
        .build(tauri::generate_context!())
        .expect("Ошибка при инициализации приложения")
        .run(|_app_handle, _event| {});
}
