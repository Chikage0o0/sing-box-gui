use serde::Deserialize;
use anyhow::Result;

use super::network::client;

#[derive(Debug, Deserialize)]
struct Release {
    tag_name: String,
    prerelease: bool,
    #[serde(rename = "draft")]
    is_draft: bool,
}


/// 获取指定仓库的最新发布版本
#[allow(dead_code)]
pub async fn get_latest_release(owner: &str, repo: &str) -> Result<String> {
    let client = client()?;
    let url = format!("https://api.github.com/repos/{}/{}/releases", owner, repo);
    
    let releases: Vec<Release> = client
        .get(&url)
        .send()
        .await?
        .json()
        .await?;

    // 获取最新的正式版本
    let latest_release = releases.iter()
        .find(|r| !r.prerelease && !r.is_draft)
        .map(|r| r.tag_name.clone());

    latest_release.ok_or_else(|| anyhow::anyhow!("No release found"))
}

/// 获取指定仓库的最新预发布版本
#[allow(dead_code)]
pub async fn get_latest_prerelease(owner: &str, repo: &str) -> Result<String> {
    let client = client()?;
    let url = format!("https://api.github.com/repos/{}/{}/releases", owner, repo);
    
    let releases: Vec<Release> = client
        .get(&url)
        .send()
        .await?
        .json()
        .await?;

    // 获取最新的预发布版本
    let latest_prerelease = releases.iter()
        .find(|r| r.prerelease && !r.is_draft)
        .map(|r| r.tag_name.clone());

    latest_prerelease.ok_or_else(|| anyhow::anyhow!("No prerelease found"))
}

#[cfg(test)]
mod tests {
    use crate::utils::network::init;

    use super::*;

    #[tokio::test]
    async fn test_get_latest_release() {
        init().unwrap();
        let owner = "SagerNet";
        let repo = "sing-box";
        let release = get_latest_release(owner, repo).await.unwrap();
        println!("Latest release: {}", release);
    }

    #[tokio::test]
    async fn test_get_latest_prerelease() {
        init().unwrap();
        let owner = "SagerNet";
        let repo = "sing-box";
        let prerelease = get_latest_prerelease(owner, repo).await.unwrap();
        println!("Latest prerelease: {}", prerelease);
    }
}