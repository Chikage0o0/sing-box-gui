use crate::sing_box::PROCESS_MANAGER;

#[tauri::command]
pub fn get_status() -> Result<String, String> {
    let status = PROCESS_MANAGER.get_state();
    match status {
        crate::sing_box::ProcessState::NotRunning => Ok("idle".to_string()),
        crate::sing_box::ProcessState::Starting => Ok("loading".to_string()),
        crate::sing_box::ProcessState::Running => Ok("success".to_string()),
        crate::sing_box::ProcessState::StartFailed(error) => {
            match error {
                crate::sing_box::Error::NoPermission(e) => Err(format!("NoPermission: {}", e)),
                crate::sing_box::Error::Unknown(e) => Err(format!("Unknown: {}", e)),
            }
        }
    }
}

#[tauri::command]
pub async fn start() -> Result<(), String> {
    PROCESS_MANAGER.start().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn stop() -> Result<(), String> {
    PROCESS_MANAGER.stop().await.map_err(|e| e.to_string())
}