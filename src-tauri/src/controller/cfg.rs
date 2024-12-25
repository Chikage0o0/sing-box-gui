use tracing::error;

use crate::cfg::{global, Setting};

#[tauri::command]
pub fn get_setting() -> Result<Setting, String> {
    let mut config = global().load_full().as_ref().clone();

    Ok(config)
}

#[tauri::command]
pub async fn set_setting(cfg: Setting) -> Result<(), String> {
    cfg.save().await.map_err(|err| {
        error!("Save config failed: {}", err);
        err.to_string()
    })?;
    Ok(())
}
