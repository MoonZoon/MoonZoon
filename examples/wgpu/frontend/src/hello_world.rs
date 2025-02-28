// Based on:
// - hello_triangle.rs example
// - https://github.com/grovesNL/glyphon/blob/main/examples/hello-world.rs

use std::future::Future;
use std::rc::Rc;
use std::sync::Arc;

use zoon::wasm_bindgen::throw_str;
use zoon::*;

use glyphon::{
    fontdb, Attrs, Buffer, Cache, Color, Family, FontSystem, Metrics, Resolution, Shaping,
    SwashCache, TextArea, TextAtlas, TextBounds, TextRenderer, Viewport,
};

use wgpu::{Device, MultisampleState, Queue, Surface, SurfaceConfiguration};
use winit::platform::web::WindowAttributesExtWebSys;
use winit::{
    application::ApplicationHandler,
    dpi::{LogicalSize, PhysicalSize},
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop, EventLoopProxy},
    window::{Window, WindowId},
};

pub fn run(canvas: zoon::web_sys::HtmlCanvasElement) {
    let event_loop = EventLoop::with_user_event().build().unwrap_throw();
    let mut app = Application::new(&event_loop, canvas);
    event_loop.run_app(&mut app).unwrap_throw();
}

fn create_graphics(
    event_loop: &ActiveEventLoop,
    canvas: zoon::web_sys::HtmlCanvasElement,
) -> impl Future<Output = Graphics> + 'static {
    let window_attrs = Window::default_attributes()
        .with_max_inner_size(LogicalSize::new(super::CANVAS_WIDTH, super::CANVAS_HEIGHT))
        // NOTE: It has to be set to make it work in Firefox
        .with_inner_size(LogicalSize::new(super::CANVAS_WIDTH, super::CANVAS_HEIGHT))
        .with_canvas(Some(canvas));

    let window = Rc::new(event_loop.create_window(window_attrs).unwrap_throw());
    let instance = wgpu::Instance::default();
    let surface = instance
        .create_surface(window.clone())
        .unwrap_or_else(|e| throw_str(&format!("{e:#?}")));

    async move {
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                compatible_surface: Some(&surface),
                power_preference: wgpu::PowerPreference::None,
                force_fallback_adapter: false,
            })
            .await
            .unwrap_throw();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("WGPU Device"),
                    memory_hints: wgpu::MemoryHints::default(),
                    required_features: wgpu::Features::default(),
                    #[cfg(feature = "webgpu")]
                    required_limits: wgpu::Limits::default().using_resolution(adapter.limits()),
                    #[cfg(feature = "webgl")]
                    required_limits: wgpu::Limits::downlevel_webgl2_defaults()
                        .using_resolution(adapter.limits()),
                },
                None,
            )
            .await
            .unwrap_throw();

        let physical_size = window.inner_size();

        let surface_config = surface
            .get_default_config(&adapter, physical_size.width, physical_size.height)
            .unwrap_throw();

        let swapchain_capabilities = surface.get_capabilities(&adapter);
        let swapchain_format = swapchain_capabilities.formats[0];

        // Set up text renderer
        let mut font_system = {
            // NOTE: Smaller and compressed font would be probably better
            let font_data = include_bytes!("../fonts/FiraCode-Regular.ttf");
            FontSystem::new_with_fonts([fontdb::Source::Binary(Arc::new(font_data))])
        };
        let swash_cache = SwashCache::new();
        let cache = Cache::new(&device);
        let viewport = Viewport::new(&device, &cache);
        let mut atlas = TextAtlas::new(&device, &queue, &cache, swapchain_format);
        let text_renderer =
            TextRenderer::new(&mut atlas, &device, MultisampleState::default(), None);
        let mut text_buffer = Buffer::new(&mut font_system, Metrics::new(30.0, 42.0));

        text_buffer.set_text(
            &mut font_system,
            "Hello world!",
            Attrs::new().family(Family::Monospace),
            Shaping::Advanced,
        );
        text_buffer.shape_until_scroll(&mut font_system, false);

        Graphics {
            device,
            queue,
            surface,
            surface_config,

            font_system,
            swash_cache,
            viewport,
            atlas,
            text_renderer,
            text_buffer,

            window,
        }
    }
}

#[allow(dead_code)]
struct Graphics {
    device: Device,
    queue: Queue,
    surface: Surface<'static>,
    surface_config: SurfaceConfiguration,

    font_system: FontSystem,
    swash_cache: SwashCache,
    viewport: glyphon::Viewport,
    atlas: glyphon::TextAtlas,
    text_renderer: glyphon::TextRenderer,
    text_buffer: glyphon::Buffer,

    // Make sure that the winit window is last in the struct so that
    // it is dropped after the wgpu surface is dropped, otherwise the
    // program may crash when closed. This is probably a bug in wgpu.
    window: Rc<Window>,
}

struct GraphicsBuilder {
    event_loop_proxy: Option<EventLoopProxy<Graphics>>,
    canvas: zoon::web_sys::HtmlCanvasElement,
}

impl GraphicsBuilder {
    fn new(
        event_loop_proxy: EventLoopProxy<Graphics>,
        canvas: zoon::web_sys::HtmlCanvasElement,
    ) -> Self {
        Self {
            event_loop_proxy: Some(event_loop_proxy),
            canvas,
        }
    }

    fn build_and_send(&mut self, event_loop: &ActiveEventLoop) {
        let Some(event_loop_proxy) = self.event_loop_proxy.take() else {
            // event_loop_proxy is already spent - we already constructed Graphics
            return;
        };

        let gfx_fut = create_graphics(event_loop, self.canvas.clone());
        wasm_bindgen_futures::spawn_local(async move {
            let gfx = gfx_fut.await;
            assert!(event_loop_proxy.send_event(gfx).is_ok());
        });
    }
}

enum MaybeGraphics {
    Builder(GraphicsBuilder),
    Graphics(Graphics),
}

struct Application {
    graphics: MaybeGraphics,
}

impl Application {
    fn new(event_loop: &EventLoop<Graphics>, canvas: zoon::web_sys::HtmlCanvasElement) -> Self {
        Self {
            graphics: MaybeGraphics::Builder(GraphicsBuilder::new(
                event_loop.create_proxy(),
                canvas,
            )),
        }
    }

    fn draw(&mut self) {
        let MaybeGraphics::Graphics(gfx) = &mut self.graphics else {
            // draw call rejected because graphics doesn't exist yet
            return;
        };

        gfx.viewport.update(
            &gfx.queue,
            Resolution {
                width: gfx.surface_config.width,
                height: gfx.surface_config.height,
            },
        );

        gfx.text_renderer
            .prepare(
                &gfx.device,
                &gfx.queue,
                &mut gfx.font_system,
                &mut gfx.atlas,
                &gfx.viewport,
                [TextArea {
                    buffer: &gfx.text_buffer,
                    left: 10.0,
                    top: 10.0,
                    scale: 1.0,
                    bounds: TextBounds {
                        left: 0,
                        top: 0,
                        right: 600,
                        bottom: 160,
                    },
                    default_color: Color::rgb(255, 255, 255),
                    custom_glyphs: &[],
                }],
                &mut gfx.swash_cache,
            )
            .unwrap();

        let frame = gfx.surface.get_current_texture().unwrap_throw();
        let view = frame.texture.create_view(&Default::default());
        let mut encoder = gfx.device.create_command_encoder(&Default::default());

        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            });
            gfx.text_renderer
                .render(&gfx.atlas, &gfx.viewport, &mut rpass)
                .unwrap();
        }

        let command_buffer = encoder.finish();
        gfx.queue.submit([command_buffer]);
        frame.present();

        gfx.atlas.trim();
    }

    fn resized(&mut self, size: PhysicalSize<u32>) {
        let MaybeGraphics::Graphics(gfx) = &mut self.graphics else {
            return;
        };
        gfx.surface_config.width = size.width;
        gfx.surface_config.height = size.height;
        gfx.surface.configure(&gfx.device, &gfx.surface_config);
    }
}

impl ApplicationHandler<Graphics> for Application {
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::Resized(size) => self.resized(size),
            WindowEvent::RedrawRequested => self.draw(),
            WindowEvent::CloseRequested => event_loop.exit(),
            _ => (),
        }
    }

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if let MaybeGraphics::Builder(builder) = &mut self.graphics {
            builder.build_and_send(event_loop);
        }
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, graphics: Graphics) {
        self.graphics = MaybeGraphics::Graphics(graphics);
    }
}
