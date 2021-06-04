use tokio::{fs, task::{JoinHandle, spawn, spawn_blocking},io::AsyncWriteExt};
use anyhow::{Context, Result};
use std::{sync::Arc, path::Path};
use brotli::{CompressorReader as BrotliEncoder, enc::backward_references::BrotliEncoderParams};
use flate2::{bufread::GzEncoder, Compression as GzCompression};
use crate::helper::ReadToVec;

pub trait FileCompressor {
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
            let mut file_writer = fs::File::create(&path)
                .await
                .with_context(|| format!("Failed to create the file {:#?}", path))?;

            let compressed_content = spawn_blocking(move || {
                Self::compress(&content)
            }).await??;

            file_writer.write_all(&compressed_content).await?;
            file_writer.flush().await?;
            Ok(())
        })
    }

    fn compress(bytes: &[u8]) -> Result<Vec<u8>>;
}

pub struct BrotliFileCompressor;

impl FileCompressor for BrotliFileCompressor {
    fn compress(bytes: &[u8]) -> Result<Vec<u8>> {
        BrotliEncoder::with_params(
            bytes, 0, &BrotliEncoderParams::default()
        ).read_to_vec()
    }
}

pub struct GzipFileCompressor;

impl FileCompressor for GzipFileCompressor {
    fn compress(bytes: &[u8]) -> Result<Vec<u8>> {
        GzEncoder::new(bytes, GzCompression::best()).read_to_vec()
    }
}
