use tauri::{WindowBuilder, generate_handler};

fn main() {
    let app = tauri::Builder::default()
        .invoke_handler(generate_handler![])
        .build(tauri::generate_context!())
        .expect("Ошибка при инициализации приложения");

    WindowBuilder::new(&app, "main", tauri::WindowUrl::App("index.html".into()))
        .title("ReaderAI")
        .inner_size(1200.0, 800.0)
        .build()
        .expect("Не удалось создать главное окно");

    app.run(|_app_handle, _event| {});
}
