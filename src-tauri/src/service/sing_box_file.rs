use std::{path::PathBuf, sync::LazyLock};

use anyhow::Result;

use crate::{service::DATA_DIR, utils::network::client};

pub async fn download_sing_box_config(url: &str) -> Result<()> {
    static CONFIG_PATH: LazyLock<PathBuf> = LazyLock::new(|| DATA_DIR.join("config.json"));

    let resp = client()?.get(url).send().await?;
    resp.error_for_status_ref()?;
    let content = resp.text().await?;
    tokio::fs::write(&*CONFIG_PATH, content).await?;
    Ok(())
}
