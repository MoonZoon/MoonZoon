use crate::config::Config;
use std::io::{self, stdout, Write};
use std::net::SocketAddr;
use local_ip_address::local_ip;

pub struct LazyMessageWriter(Vec<u8>);

impl LazyMessageWriter {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn write_all(&self) -> io::Result<()> {
        stdout().write_all(&self.0)
    }

    pub fn server_is_running(&mut self, address: &SocketAddr, config: &Config) -> io::Result<()> {
        let protocol = if config.https { "https" } else { "http" };
        let port = config.port;
        writeln!(
            &mut self.0,
            "Server is running on {protocol}://{address} [{protocol}://localhost:{port}]",
        )?;
        if let Ok(ip) = local_ip() {
            writeln!(
                &mut self.0,
                "Server URL on the local network: {protocol}://{ip}:{port}",
            )?;
        }
        Ok(())
    }

    pub fn redirect_from(&mut self, address: &SocketAddr, config: &Config) -> io::Result<()> {
        let port = config.port;
        writeln!(
            &mut self.0,
            "Redirect from http://{address} [http://localhost:{port}]",
        )
    }
}
