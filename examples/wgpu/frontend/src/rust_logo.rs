// Based on:
// - https://github.com/nical/lyon/tree/main/examples/wgpu

use std::future::Future;
use std::rc::Rc;
use std::sync::Arc;

use zoon::wasm_bindgen::throw_str;
use zoon::*;

use glyphon::{
    Attrs, Buffer, Cache, Color, Family, FontSystem, Metrics, Resolution, Shaping, SwashCache,
    TextArea, TextAtlas, TextBounds, TextRenderer, Viewport, fontdb
};

use wgpu::{Device, Queue, Surface, SurfaceConfiguration, MultisampleState};
use winit::{
    application::ApplicationHandler,
    dpi::{PhysicalSize, LogicalSize},
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop, EventLoopProxy},
    window::{Window, WindowId},
};
use winit::platform::web::WindowAttributesExtWebSys;

pub fn run(canvas: zoon::web_sys::HtmlCanvasElement) {
    todo!()
}
