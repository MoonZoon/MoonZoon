use crate::config::Config;
use std::net::SocketAddr;
use std::io::{self, Write, stdout};

pub struct LazyMessageWriter(Vec<u8>);

impl LazyMessageWriter {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn write_all(&self) -> io::Result<()> {
        stdout().write_all(&self.0)
    }

    pub fn server_is_running(&mut self, address: &SocketAddr, config: &Config) -> io::Result<()> {
        writeln!(
            &mut self.0, 
            "Server is running on {protocol}://{address} [{protocol}://localhost:{port}]",
            address = address,
            protocol = if config.https { "https" } else { "http" },
            port = config.port
        )
    }
    
    pub fn redirect_from(&mut self, address: &SocketAddr, config: &Config) -> io::Result<()> {
        writeln!(
            &mut self.0,
            "Redirect from http://{address} [http://localhost:{port}]",
            address = address,
            port = config.redirect_server.port
        )
    }
}
