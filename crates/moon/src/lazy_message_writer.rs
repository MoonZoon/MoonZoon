use crate::config::Config;
use local_ip_address::local_ip;
use qrcode::{render::unicode as QrUnicode, QrCode};
use std::{
    io::{self, stdout, Write},
    net::SocketAddr,
};

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
            let url = format!("{protocol}://{ip}:{port}");

            let qr_code = QrCode::new(&url)
                .expect("failed to create a QR code with the server url")
                .render::<QrUnicode::Dense1x2>()
                .dark_color(QrUnicode::Dense1x2::Light)
                .light_color(QrUnicode::Dense1x2::Dark)
                .quiet_zone(false)
                .build();

            writeln!(
                &mut self.0,
                "Server URL on the local network: {url}\n{qr_code}",
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
