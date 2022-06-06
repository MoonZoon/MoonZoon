use crate::config::Config;

pub fn localhost_url(config: &Config) -> String {
    format!(
        "{protocol}://localhost:{port}",
        protocol = if config.https { "https" } else { "http" },
        port = config.port
    )
}
