use std::time::Duration;

use tauri::{AppHandle, Manager as _};
use tokio::time::sleep;
use tracing::{debug, error};

pub fn show_or_create(app_handle: &AppHandle) {
    debug!("create window");
    if let Some(window) = app_handle.get_webview_window("main") {
        debug!("Window is Exist, show it.");
        let _ = window.unminimize();
        let _ = window.show();
        let _ = window.set_focus();
        return;
    }

    debug!("Window is not Exist, create it.");
    let builder = tauri::WebviewWindow::builder(
        app_handle,
        "main".to_string(),
        tauri::WebviewUrl::App("index.html".into()),
    )
    .center()
    .fullscreen(false)
    .min_inner_size(800.0, 600.0);

    match builder
        .decorations(false)
        .inner_size(800.0, 600.0)
        .visible(false)
        .build()
    {
        Ok(_) => {
            let app_handle = app_handle.clone();

            // 加点延迟避免界面闪一下
            tauri::async_runtime::spawn(async move {
                sleep(Duration::from_millis(500)).await;

                if let Some(window) = app_handle.get_webview_window("main") {
                    let _ = window.set_shadow(true);
                    let _ = window.show();
                    let _ = window.unminimize();
                    let _ = window.set_focus();
                } else {
                    error!("failed to create window, get_window is None")
                }
            });
        }
        Err(err) => error!("{err}"),
    }
}
