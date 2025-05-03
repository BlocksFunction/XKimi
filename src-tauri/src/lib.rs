#[cfg_attr(mobile, tauri::mobile_entry_point)]
use tauri::Manager;

#[tauri::command]
fn show_window(window: tauri::Window, label: &str) {
    window.get_webview_window(label).unwrap().show().unwrap();
}

#[tauri::command]
fn close_window(app: tauri::AppHandle, label: &str) {
    let webview_window = app.get_webview_window(label);
    if let Some(webview_window) = webview_window {
        webview_window.close().unwrap();
    }
}

#[tauri::command]
fn hide_window(app: tauri::AppHandle, label: &str) {
    let webview_window = app.get_webview_window(label);
    if let Some(webview_window) = webview_window {
        webview_window.hide().unwrap();
    }
}

#[tauri::command]
fn minimize_window(app: tauri::AppHandle, label: &str) {
    let webview_window = app.get_webview_window(label);
    if let Some(webview_window) = webview_window {
        webview_window.minimize().unwrap();
    }
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let splashscreen_window = app.get_webview_window("splashscreen").unwrap();
            let main_window = app.get_webview_window("main").unwrap();

            tauri::async_runtime::spawn(async move {
                std::thread::sleep(std::time::Duration::from_secs(5));
                splashscreen_window.close().unwrap();
                main_window.show().unwrap();
            });
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            show_window,
            hide_window,
            close_window,
            minimize_window
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application")
}
