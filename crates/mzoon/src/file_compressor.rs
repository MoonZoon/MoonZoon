use tokio::{fs, task::JoinHandle, spawn, io::AsyncWriteExt};
use anyhow::{Context, Result};
use std::{sync::Arc, path::Path, io::Read};
use brotli::{CompressorReader as BrotliEncoder, enc::backward_references::BrotliEncoderParams};
use flate2::{bufread::GzEncoder, Compression as GzCompression};

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

            let compressed_content = Self::compress(&content)?;
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
        let params = BrotliEncoderParams::default();
        let mut compressor = BrotliEncoder::with_params(bytes, 0, &params);

        let mut compressed_bytes = Vec::new();
        compressor.read_to_end(&mut compressed_bytes)?;
        Ok(compressed_bytes)
    }
}

pub struct GzipFileCompressor;

impl FileCompressor for GzipFileCompressor {
    fn compress(bytes: &[u8]) -> Result<Vec<u8>> {
        let mut compressor = GzEncoder::new(bytes, GzCompression::best());

        let mut compressed_bytes = Vec::new();
        compressor.read_to_end(&mut compressed_bytes)?;
        Ok(compressed_bytes)
    }
}
