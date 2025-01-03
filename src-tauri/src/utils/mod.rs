use anyhow::Result;
use privilege::user::privileged;
use std::env;
pub mod github;
pub mod network;

#[cfg(target_os = "windows")]
pub fn restart_as_admin() -> Result<()> {
    // 获取当前可执行文件路径

    use std::{os::windows::process::CommandExt, process::Command};

    use windows::Win32::System::Threading::CREATE_NO_WINDOW;

    use crate::APP_HANDLE;

    let current_exe = env::current_exe()?;

    // 获取命令行参数
    let mut args: Vec<String> = env::args().skip(1).collect();

    // 检查是否已经以管理员权限运行
    if !privileged() {
        args.push("--restart-as-admin".to_string());
        let args_str = args.join(" ");

        // 以管理员权限重新启动
        Command::new("powershell")
            .arg("-Command")
            .arg(format!(
                "Start-Process -FilePath \"{}\" -ArgumentList \"{}\" -Verb RunAs",
                current_exe.display(),
                args_str
            ))
            .creation_flags(CREATE_NO_WINDOW.0)  
            .spawn()?;

        // 退出当前进程
        APP_HANDLE.get().unwrap().exit(0);
    }

    Ok(())
}

#[cfg(target_os = "linux")]
pub fn restart_as_admin() -> Result<()> {
    unimplemented!("restart_as_admin is not implemented on linux")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_restart_as_admin() {
        restart_as_admin().unwrap();
    }
}
