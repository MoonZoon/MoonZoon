use async_compression::{Level, futures::write::{GzipEncoder, BrotliEncoder}};
use futures::{AsyncWrite, AsyncWriteExt};
use tokio_util::compat::{TokioAsyncWriteCompatExt, Compat};
use tokio::{fs, task::JoinHandle, spawn};
use anyhow::{Context, Result};
use std::{sync::Arc, path::Path};

pub type BrotliFileCompressor = BrotliEncoder<Compat<fs::File>>;
pub type GzipFileCompressor = GzipEncoder<Compat<fs::File>>;

pub trait FileCompressor where Self: AsyncWrite + Unpin + Send + Sized {
    fn compress_file(
        content: Arc<Vec<u8>>, 
        path: &Path, 
        extension: &str
    ) -> JoinHandle<Result<()>> {
        let mut file_extension = path.extension().unwrap_or_default().to_owned();
        file_extension.push(".");
        file_extension.push(extension);
        let path = path.with_extension(file_extension);

        spawn(async move {
            let file_writer = fs::File::create(&path)
                .await
                .with_context(|| format!("Failed to create the file {:#?}", path))?;

            let mut compressor = Self::with_quality(file_writer.compat_write(), Level::Best);
            compressor.write_all(&content).await?;
            compressor.flush().await?;
            Ok(())
        })
    }

    fn with_quality(inner: Compat<fs::File>, level: Level) -> Self;
}

impl FileCompressor for BrotliFileCompressor {
    fn with_quality(inner: Compat<fs::File>, level: Level) -> Self {
        Self::with_quality(inner, level)
    }
}

impl FileCompressor for GzipFileCompressor {
    fn with_quality(inner: Compat<fs::File>, level: Level) -> Self {
        Self::with_quality(inner, level)
    }
}
