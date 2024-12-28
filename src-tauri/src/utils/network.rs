use std::sync::OnceLock;

use reqwest::IntoUrl;
use anyhow::Result;

static CLIENT:OnceLock<reqwest::Client> = OnceLock::new();

pub fn init()->Result<()>{
    if CLIENT.get().is_none(){
        let client = reqwest::Client::builder()
        .user_agent(format!("sing-box-gui/{}", env!("CARGO_PKG_VERSION")))
        .build()?;

        let _ = CLIENT.set(client);
    }
    Ok(())
}

pub fn client() -> Result<&'static reqwest::Client> {
    CLIENT.get().ok_or_else(|| anyhow::anyhow!("Client not initialized"))
}

pub async fn test_network_connection<U: IntoUrl>(url: U) -> Result<()> {
    let resp = client()?.get(url).send().await?;
    resp.error_for_status_ref()?;
    Ok(())
}






