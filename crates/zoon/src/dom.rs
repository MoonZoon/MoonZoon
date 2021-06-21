use crate::*;

pub fn window() -> web_sys::Window {
    web_sys::window().expect_throw("window")
}

pub fn document() -> web_sys::Document {
    window().document().expect_throw("document")
}
