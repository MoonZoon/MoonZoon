use anyhow::Result;

pub fn download(url: impl AsRef<str>) -> Result<Vec<u8>> {
    let bytes = attohttpc::get(url)
        .send()?
        .error_for_status()?
        .bytes()?;
    Ok(bytes)
}
