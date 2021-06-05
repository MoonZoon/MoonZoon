use anyhow::Error;
use fehler::throws;

#[throws]
pub fn download(url: impl AsRef<str>) -> Vec<u8> {
    attohttpc::get(url)
        .send()?
        .error_for_status()?
        .bytes()?
}
