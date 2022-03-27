use crate::*;

pub fn window() -> web_sys::Window {
    web_sys::window().unwrap_throw()
}

pub fn document() -> web_sys::Document {
    window().document().unwrap_throw()
}

pub fn history() -> web_sys::History {
    window().history().unwrap_throw()
}

pub fn append_to_head(html: impl AsRef<str>) {
    
}
