#[derive(Copy, Clone)]
pub struct Redirect;

impl Redirect {
    pub fn new() -> Self {
        Self
    }

    pub fn enabled(self, enabled: bool) -> Self {
        Self
    }

    pub fn http_to_https(self, http_to_https: bool) -> Self {
        self
    } 

    pub fn port(self, from_port: u16, to_port: u16) -> Self {
        self
    } 
}
