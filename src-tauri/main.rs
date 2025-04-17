use std::fs;
use tauri::{generate_handler, Manager, Menu, MenuItem, Submenu};
use tauri::api::notification::Notification;
use tauri::{
    CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu,
};

// Импортируем стили
mod src {
    pub mod styles;
}
use src::styles::{WINDOW_STYLES, TITLEBAR_SCRIPT};

#[cfg(target_os = "macos")]
use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};

#[cfg(target_os = "windows")]
use window_shadows::set_shadow;

#[tauri::command]
fn open_document(filepath: String) -> Result<String, String> {
    match fs::read_to_string(&filepath) {
        Ok(content) => Ok(content),
        Err(e) => Err(format!("Не удалось открыть документ: {}", e)),
    }
}

#[tauri::command]
fn save_document(filepath: String, content: String) -> Result<(), String> {
    match fs::write(&filepath, content) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Не удалось сохранить документ: {}", e)),
    }
}

#[tauri::command]
fn send_notification(title: String, body: String) -> Result<(), String> {
    println!("Уведомление: {} - {}", title, body);

    let notification = Notification::new("com.readerai.app")
        .title(title)
        .body(body);

    notification.show().map_err(|e| e.to_string())
}

fn main() {
    // ----- System tray -----
    let quit_item = CustomMenuItem::new("quit", "Выход");
    let hide_item = CustomMenuItem::new("hide", "Свернуть");
    let tray_menu = SystemTrayMenu::new().add_item(quit_item).add_item(hide_item);

    // ----- Application menu -----
    let open_menu_item = CustomMenuItem::new("open", "Открыть документ");
    let preferences_menu_item = CustomMenuItem::new("preferences", "Настройки");
    let main_menu = Menu::new()
        .add_item(open_menu_item)
        .add_native_item(MenuItem::Separator)
        .add_item(preferences_menu_item);

    tauri::Builder::default()
        .system_tray(SystemTray::new().with_menu(tray_menu))
        .menu(Menu::new().add_submenu(Submenu::new("ReaderAI", main_menu)))
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "quit" => std::process::exit(0),
                "hide" => {
                    if let Some(main_window) = app.get_window("main") {
                        main_window.hide().unwrap();
                    }
                }
                _ => {}
            },
            _ => {}
        })
        .on_menu_event(|event| match event.menu_item_id() {
            "open" => {
                // Обработка открытия файла будет реализована на стороне фронтенда
                if let Some(window) = event.window().get_window("main") {
                    window.emit("menu-action", "open").unwrap();
                }
            }
            "preferences" => {
                // Отправляем событие на фронтенд вместо открытия отдельного окна
                if let Some(window) = event.window().get_window("main") {
                    window.emit("menu-action", "preferences").unwrap();
                }
            }
            _ => {}
        })
        .invoke_handler(generate_handler![open_document, save_document, send_notification])
        .setup(|app| {
            // Получаем главное окно, которое создано автоматически из tauri.conf.json
            let main_window = app.get_window("main").expect("Не удалось получить главное окно");
            
            // Внедряем CSS для скругления и JavaScript для кастомного заголовка
            main_window.eval(&format!(
                "const style = document.createElement('style'); style.textContent = `{}`; document.head.appendChild(style);",
                WINDOW_STYLES
            )).expect("Не удалось внедрить CSS");
            
            main_window.eval(TITLEBAR_SCRIPT).expect("Не удалось внедрить скрипт заголовка");
            
            // Применяем тени и эффекты в зависимости от платформы
            #[cfg(target_os = "windows")]
            set_shadow(&main_window, true).expect("Ошибка при применении теней к окну");
            
            #[cfg(target_os = "macos")]
            apply_vibrancy(&main_window, NSVisualEffectMaterial::HudWindow, None, Some(28.0))
                .expect("Ошибка при применении эффекта vibrancy");
            
            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("Ошибка при запуске приложения")
        .run(|_app_handle, _event| {});
}