use anyhow::Result;
use tauri::{
    menu::{MenuBuilder, MenuEvent, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIcon, TrayIconBuilder, TrayIconEvent},
    App, AppHandle
};

pub fn get_menu(app: &App) -> Result<TrayIcon> {
    let quit = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
    let open = MenuItem::with_id(app, "open", "打开", true, None::<&str>)?;
    let menu = MenuBuilder::new(app)
        .item(&open)
        .separator()
        .item(&quit)
        .build()?;
    let tray = TrayIconBuilder::new()
        .menu(&menu)
        .icon(app.default_window_icon().unwrap().clone())
        .menu_on_left_click(false)
        .on_menu_event(menu_event)
        .on_tray_icon_event(tray_icon_event)
        .build(app)?;
    Ok(tray)
}

fn menu_event(app: &AppHandle, event: MenuEvent) {
    match event.id.as_ref() {
        "quit" => {
            app.exit(0);
        }
        "open" => super::window::show_or_create(app),

        _ => {
            println!("menu item {:?} not handled", event.id);
        }
    }
}

fn tray_icon_event(tray: &TrayIcon, event: TrayIconEvent) {
    match event {
        TrayIconEvent::Click {
            button: MouseButton::Right,
            button_state: MouseButtonState::Up,
            ..
        } => {
            let _ = tray.app_handle().show_menu();
        }
        _ => {}
    }
}
