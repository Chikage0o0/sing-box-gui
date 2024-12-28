use anyhow::Result;
use tracing::info;

#[cfg(target_os = "windows")]
fn set_windows_autostart(switch: bool) -> Result<()> {
    use privilege::runas::Command;

    use crate::APP_NAME;

    let exe_path = std::env::current_exe()?;
    let exe_path = exe_path.to_str().unwrap();

    if switch {
        Command::new("reg")
            .args(&[
                "add",
                "HKLM\\Software\\Microsoft\\Windows\\CurrentVersion\\Run",
                "/v",
                APP_NAME,
                "/t",
                "REG_SZ",
                "/d",
                exe_path,
                "/f",
            ])
            .run()?;
    } else {
        Command::new("reg")
            .args(&[
                "delete",
                "HKLM\\Software\\Microsoft\\Windows\\CurrentVersion\\Run",
                "/v",
                APP_NAME,
                "/f",
            ])
            .run()?;
    }

    Ok(())
}

#[cfg(target_os = "windows")]
fn get_windows_autostart() -> Result<bool> {
    use std::process::Command;

    use crate::APP_NAME;
    let exe_path = std::env::current_exe()?;
    let exe_path = exe_path.to_str().unwrap();

    let output = Command::new("reg")
        .args(&[
            "query",
            "HKLM\\Software\\Microsoft\\Windows\\CurrentVersion\\Run",
            "/v",
            APP_NAME,
        ])
        .output()?;

    let output = String::from_utf8(output.stdout)?;

    Ok(output.contains(exe_path))
}

pub fn set(switch: bool) -> Result<()> {
    info!("Set auto launch: {}", switch);
    #[cfg(target_os = "windows")]
    set_windows_autostart(switch)?;
    #[cfg(target_os = "macos")]
    set_macos_autostart(switch)?;
    #[cfg(target_os = "linux")]
    set_linux_autostart(switch)?;
    Ok(())
}

pub fn get() -> Result<bool> {
    #[cfg(target_os = "windows")]
    return get_windows_autostart();
    #[cfg(target_os = "macos")]
    return get_macos_autostart();
    #[cfg(target_os = "linux")]
    return get_linux_autostart();
}
