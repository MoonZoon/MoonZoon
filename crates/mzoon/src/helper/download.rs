use anyhow::Error;
use fehler::throws;

#[throws]
pub async fn download(url: impl AsRef<str>) -> Vec<u8> {
    reqwest::get(url.as_ref())
        .await?
        .error_for_status()?
        .bytes()
        .await?
        .to_vec()
}
