use std::io::Read;
use anyhow::Result;
use tokio::io::{AsyncRead, AsyncReadExt};
use async_trait::async_trait;

// ------ ReadToVec ------
pub trait ReadToVec: Read {
    fn read_to_vec(&mut self) -> Result<Vec<u8>> {
        let mut vec = Vec::new();
        self.read_to_end(&mut vec)?;
        Ok(vec)
    }
}
impl<T: Read> ReadToVec for T {}

// ------ AsyncReadToVec ------

#[async_trait]
pub trait AsyncReadToVec: AsyncRead where Self: Unpin {
    async fn read_to_vec(&mut self) -> Result<Vec<u8>> {
        let mut vec = Vec::new();
        self.read_to_end(&mut vec).await?;
        Ok(vec)
    }
}
#[async_trait]
impl<T: AsyncRead + Unpin> AsyncReadToVec for T {}
