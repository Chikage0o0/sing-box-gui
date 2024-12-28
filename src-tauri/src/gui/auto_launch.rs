use anyhow::Result;
use tracing::info;

#[cfg(target_os = "windows")]
fn set_windows_autostart(switch: bool) -> Result<()> {
    use tracing::warn;
    use windows::{
        core::{Interface, BSTR, GUID, VARIANT},
        Win32::{
            Foundation::{VARIANT_FALSE, VARIANT_TRUE},
            System::{
                Com::{
                    CoCreateInstance, CoInitializeEx, CoUninitialize, CLSCTX_INPROC_SERVER,
                    COINIT_MULTITHREADED,
                },
                TaskScheduler::{
                    IExecAction, ILogonTrigger, ITaskService, TaskScheduler, TASK_ACTION_EXEC,
                    TASK_CREATE_OR_UPDATE, TASK_LOGON_INTERACTIVE_TOKEN, TASK_RUNLEVEL_HIGHEST,
                    TASK_TRIGGER_LOGON,
                },
            },
        },
    };

    use crate::APP_NAME;
    let current_exe = std::env::current_exe().unwrap();

    unsafe {
        CoInitializeEx(None, COINIT_MULTITHREADED).unwrap();
    }

    // 创建任务服务实例
    let task_service: ITaskService =
        unsafe { CoCreateInstance(&TaskScheduler as *const GUID, None, CLSCTX_INPROC_SERVER)? };

    // 连接到任务服务
    unsafe {
        task_service.Connect(
            &VARIANT::default(), // 本地计算机
            &VARIANT::default(), // 默认用户
            &VARIANT::default(), // 默认域
            &VARIANT::default(), // 默认密码
        )?;
    }

    // 获取根文件夹
    let root_folder = unsafe { task_service.GetFolder(&BSTR::from("\\"))? };

    if !switch {
        // 如果switch为false，删除任务
        let task_name = &BSTR::from(APP_NAME);
        match unsafe { root_folder.DeleteTask(task_name, 0) } {
            Ok(_) => {
                info!("Task deleted successfully!");
            }
            Err(e) => {
                warn!("Failed to delete task: {:?}", e);
                // 如果任务不存在，不认为是错误
                if e.code().0 != 0x80070002u32 as i32 {
                    // ERROR_FILE_NOT_FOUND
                    return Err(e.into());
                }
            }
        }
    } else {
        // 创建任务定义
        let task_definition = unsafe { task_service.NewTask(0)? };

        // 设置常规信息
        let reg_info = unsafe { task_definition.RegistrationInfo()? };
        unsafe {
            reg_info.SetAuthor(&BSTR::from(APP_NAME))?;
            reg_info.SetDescription(&BSTR::from("Sing Box"))?;
        }

        // 设置主体安全选项
        let principal = unsafe { task_definition.Principal()? };
        unsafe {
            principal.SetRunLevel(TASK_RUNLEVEL_HIGHEST)?; // 使用最高权限运行
            principal.SetLogonType(TASK_LOGON_INTERACTIVE_TOKEN)?; // 使用当前用户登录
        }

        // 设置设置选项
        let settings = unsafe { task_definition.Settings()? };
        unsafe {
            settings.SetEnabled(VARIANT_TRUE)?;
            settings.SetStartWhenAvailable(VARIANT_TRUE)?;
            settings.SetAllowHardTerminate(VARIANT_TRUE)?;
            settings.SetRunOnlyIfNetworkAvailable(VARIANT_FALSE)?;
            settings.SetDisallowStartIfOnBatteries(VARIANT_FALSE)?;
            settings.SetStopIfGoingOnBatteries(VARIANT_FALSE)?;
        }

        // 创建触发器
        let triggers = unsafe { task_definition.Triggers()? };
        let trigger: ILogonTrigger = unsafe { triggers.Create(TASK_TRIGGER_LOGON)?.cast()? };
        unsafe {
            trigger.SetEnabled(VARIANT_TRUE)?;
            // 可以设置特定用户ID，这里使用当前用户
            // trigger.SetUserId(w!("CURRENT_USER"))?;
        }

        // 创建动作
        let actions = unsafe { task_definition.Actions()? };
        let action: IExecAction = unsafe { actions.Create(TASK_ACTION_EXEC)?.cast()? };

        // 设置要运行的程序路径
        unsafe {
            action.SetPath(&BSTR::from(current_exe.to_str().unwrap()))?;
            // 可选：设置工作目录
            action.SetWorkingDirectory(&BSTR::from(
                current_exe.parent().unwrap().to_str().unwrap(),
            ))?;
        }

        // 注册任务
        let task_name = &BSTR::from(APP_NAME);
        let _registered_task = unsafe {
            root_folder.RegisterTaskDefinition(
                task_name,
                &task_definition,
                TASK_CREATE_OR_UPDATE.0,
                &VARIANT::default(), // 使用当前用户凭据
                &VARIANT::default(),
                TASK_LOGON_INTERACTIVE_TOKEN,
                &VARIANT::default(), // 无服务器名称
            )?
        };

        info!("Task created successfully!");
    }

    // 清理 COM
    unsafe {
        CoUninitialize();
    }
    Ok(())
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
