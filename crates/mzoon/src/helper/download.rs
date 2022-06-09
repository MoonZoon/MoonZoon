use anyhow::Error;
use fehler::throws;

#[throws]
pub async fn download(url: impl AsRef<str>) -> Vec<u8> {
    let url = url.as_ref();
    again::retry(|| get(url)).await?
}

#[throws]
async fn get(url: &str) -> Vec<u8> {
    reqwest::get(url)
        .await?
        .error_for_status()?
        .bytes()
        .await?
        .to_vec()
}
