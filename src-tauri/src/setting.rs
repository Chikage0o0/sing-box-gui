use std::{
    path::PathBuf,
    sync::{Arc, LazyLock, OnceLock},
};

use anyhow::{anyhow, Result};
use arc_swap::ArcSwap;
use config::Config;
use serde::{Deserialize, Serialize};
use tokio::fs;
use tracing::info;

static CFG: OnceLock<ArcSwap<Setting>> = OnceLock::new();
static CONFIG_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    dirs::config_dir()
        .map(|path| path.join(crate::APP_NAME).join("config.toml"))
        .unwrap_or(
            std::env::current_dir()
                .unwrap()
                .parent()
                .unwrap()
                .join("config.toml"),
        )
});

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct Setting {
    pub server: Server,
    pub client: Client,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Server {
    pub subscribe_url: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Client {
    pub log_level: String,
    pub auto_start: bool,
    pub silent_start: bool,
    pub auto_start_core: bool,
}

impl Default for Client {
    fn default() -> Self {
        Self {
            log_level: "info".to_string(),
            auto_start: false,
            silent_start: false,
            auto_start_core: false,
        }
    }
}

impl Default for Server {
    fn default() -> Self {
        Self {
            subscribe_url: "https://example.com".to_string(),
        }
    }
}

pub async fn init() {
    // 读取配置文件
    let mut cfg = if CONFIG_PATH.exists() {
        let config = Config::builder()
            .add_source(config::File::with_name(CONFIG_PATH.to_str().unwrap()))
            .build()
            .unwrap_or_else(|e| panic!("无法加载配置文件: {},{}", CONFIG_PATH.display(), e));

        // 尝试解析配置文件
        if let Ok(cfg) = config.clone().try_deserialize() {
            cfg
        } else {
            // 迁移可读取的项目
            let cfg = Setting::default();

            cfg
        }
    } else {
        // 文件不存在，创建默认配置
        let cfg = Setting::default();
        cfg.save_to_file().await.unwrap();

        cfg
    };

    let _ = CFG.set(ArcSwap::from_pointee(cfg));
}

pub fn global() -> &'static ArcSwap<Setting> {
    CFG.get().unwrap_or_else(|| panic!("配置文件未初始化"))
}

impl Setting {
    async fn save_to_file(&self) -> Result<()> {
        info!("save config to {}", CONFIG_PATH.display());
        let folder = CONFIG_PATH.as_path().parent().unwrap();
        if !folder.exists() {
            fs::create_dir_all(folder).await?;
        }
        let path = CONFIG_PATH.as_path();

        let toml = toml::to_string(&self).unwrap();

        fs::write(path, toml).await?;

        Ok(())
    }

    pub async fn save(&self) -> Result<()> {
        let old_cfg = global().load();

        // 设置自动启动
        if old_cfg.client.auto_start != self.client.auto_start {
            crate::gui::auto_launch::set(self.client.auto_start).map_err(|_e| anyhow!("设置自启失败，请尝试管理员打开应用并设置"))?;
        }

        // 保存配置
        global().store(Arc::new(self.clone()));
        self.save_to_file().await?;

        Ok(())
    }
}
