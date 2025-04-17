use tauri::{generate_handler};

fn main() {
    tauri::Builder::default()
        .invoke_handler(generate_handler![])
        .build(tauri::generate_context!())
        .expect("Ошибка при инициализации приложения")
        .run(|_app_handle, _event| {});
}
