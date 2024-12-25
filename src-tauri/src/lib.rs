use std::sync::OnceLock;

use gui::window;
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{fmt, layer::SubscriberExt, Layer};
use utils::network;

mod utils;
mod service;
mod gui;
mod setting;
mod controller;

const APP_NAME: &str = "sing-box-gui";
static APP_HANDLE: OnceLock<tauri::AppHandle> = OnceLock::new();

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    init().await;

    info!("Starting... version: {}", env!("CARGO_PKG_VERSION"));
    let app = tauri::Builder::default()
        .setup(|app| {
            let _tray = gui::tray::get_menu(app)?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            controller::setting::get_setting,
            controller::setting::set_setting,
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    APP_HANDLE.set(app.handle().clone()).unwrap();

    // 读取参数，检测是否为自动重启
    let args: Vec<String> = std::env::args().collect();
    let is_restarted = args.contains(&"--restarted".to_string());

    // 非静默启动时或者是自动重启时，创建窗口
    if !setting::global().load().client.silent_start || is_restarted {
        window::show_or_create(&app.handle())
    }

    app.run(move |_app_handle, event| match event {
        // tauri::RunEvent::Ready { .. } => {
        //     handler::init();
        // }
        tauri::RunEvent::ExitRequested {
            api, code: None, ..
        } => {
            api.prevent_exit();
        }
        // tauri::RunEvent::Exit => {
        //     handler::stop();
        // }
        _ => {}
    });
}

async fn init() {
    #[cfg(debug_assertions)]
    let fmt_layer = fmt::layer()
        .with_level(true)
        // 指定标准控制台输出
        .with_writer(std::io::stdout)
        // 日志等级过滤
        .with_filter(LevelFilter::DEBUG);

    #[cfg(not(debug_assertions))]
    let fmt_layer = {
        // 文件 appender 指定日志文件输出目录和文件名前缀
        // daily 指定生成文件名日期到年月日
        // 如： test-log.2023-08-30
        let file_appender = tracing_appender::rolling::daily("logs", "pacs-upload-tool");
        // 生成非阻塞写入器
        let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
        // 保证文件写入器生命周期
        static GUARD: OnceLock<tracing_appender::non_blocking::WorkerGuard> = OnceLock::new();
        GUARD.get_or_init(|| guard);

        fmt::layer()
            // 移除输出内容中的 颜色或其它格式相关转义字符
            .with_ansi(false)
            .with_writer(non_blocking)
            // 日志等级过滤
            .with_filter(LevelFilter::INFO)
    };

    // 生成注册中心 Registry 绑定多个输出层
    let collector = tracing_subscriber::registry().with(fmt_layer);

    // 订阅者全局注册
    tracing::subscriber::set_global_default(collector).expect("Tracing collect error");

    setting::init().await;
    network::init().expect("network init error");
}
