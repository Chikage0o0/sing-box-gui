use tracing::error;

use crate::setting::{global, Setting};

#[tauri::command]
pub fn get_setting() -> Result<Setting, String> {
    let config = global().load_full().as_ref().clone();

    Ok(config)
}

#[tauri::command]
pub async fn set_setting(setting: Setting) -> Result<(), String> {
    setting.save().await.map_err(|err| {
        error!("Save config failed: {}", err);
        err.to_string()
    })?;
    Ok(())
}
