use async_compression::{Level, futures::write::{GzipEncoder, BrotliEncoder}};
use futures::{AsyncWrite, AsyncWriteExt};
use tokio_util::compat::{TokioAsyncWriteCompatExt, Compat};
use tokio::{fs, task::JoinHandle, spawn};
use anyhow::Result;
use std::{sync::Arc, path::Path};

pub type BrotliFileCompressor = BrotliEncoder<Compat<fs::File>>;
pub type GzipFileCompressor = GzipEncoder<Compat<fs::File>>;

pub trait FileCompressor where Self: AsyncWrite + Unpin + Send + Sized {
    fn with_quality(inner: Compat<fs::File>, level: Level) -> Self;

    fn compress_file(
        content: Arc<Vec<u8>>, 
        path: &Path, 
        extension: &str
    ) -> JoinHandle<Result<()>> {
        let mut path = path.to_owned();
        path.push(extension);
        spawn(async move {
            let file_writer = fs::File::create(path).await?;
            Self::with_quality(file_writer.compat_write(), Level::Best).write_all(&content).await?;
            Ok(())
        })
    }
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
