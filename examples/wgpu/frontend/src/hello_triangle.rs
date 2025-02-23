use std::{borrow::Cow, future::Future};

#[allow(unused_imports)]
use zoon::wasm_bindgen::{prelude::wasm_bindgen, throw_str, JsCast, UnwrapThrowExt};

use zoon::*;

use wgpu::{Adapter, Device, Instance, Queue, RenderPipeline, Surface, SurfaceConfiguration};
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop, EventLoopProxy},
    window::{Window, WindowId},
};

#[cfg(target_arch = "wasm32")]
type Rc<T> = std::rc::Rc<T>;

#[cfg(not(target_arch = "wasm32"))]
type Rc<T> = std::sync::Arc<T>;

#[cfg(target_arch = "wasm32")]
const CANVAS_ID: &str = "wgpu-canvas";

pub async fn run_with_canvas(canvas: zoon::web_sys::HtmlCanvasElement) {

}

pub fn run() {
    let event_loop = EventLoop::with_user_event().build().unwrap_throw();
    let mut app = Application::new(&event_loop);
    event_loop.run_app(&mut app).unwrap_throw();
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn run_web() {
    let window = web_sys::window().unwrap_throw();
    let document = window.document().unwrap_throw();

    let canvas = document.create_element("canvas").unwrap_throw();
    canvas.set_id(CANVAS_ID);
    canvas.set_attribute("width", "500").unwrap_throw();
    canvas.set_attribute("height", "500").unwrap_throw();

    let body = document
        .get_elements_by_tag_name("body")
        .item(0)
        .unwrap_throw();
    body.append_with_node_1(canvas.unchecked_ref())
        .unwrap_throw();

    run();
}

fn create_graphics(event_loop: &ActiveEventLoop) -> impl Future<Output = Graphics> + 'static {
    #[allow(unused_mut)]
    let mut window_attrs = Window::default_attributes();

    #[cfg(target_arch = "wasm32")]
    {
        use web_sys::wasm_bindgen::JsCast;
        use winit::platform::web::WindowAttributesExtWebSys;

        let window = web_sys::window().unwrap_throw();
        let document = window.document().unwrap_throw();
        let canvas = document.get_element_by_id(CANVAS_ID).unwrap_throw();
        let html_canvas_element = canvas.unchecked_into();
        window_attrs = window_attrs.with_canvas(Some(html_canvas_element));
    }

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

        let size = window.inner_size();
        let surface_config = surface
            .get_default_config(&adapter, size.width, size.height)
            .unwrap_throw();

        #[cfg(not(target_arch = "wasm32"))]
        {
            surface.configure(&device, &surface_config);
        }

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });
        let swapchain_capabilities = surface.get_capabilities(&adapter);
        let swapchain_format = swapchain_capabilities.formats[0];

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed("
                    @vertex
                    fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> @builtin(position) vec4<f32> {
                        let x = f32(i32(in_vertex_index) - 1);
                        let y = f32(i32(in_vertex_index & 1u) * 2 - 1);
                        return vec4<f32>(x, y, 0.0, 1.0);
                    }

                    @fragment
                    fn fs_main() -> @location(0) vec4<f32> {
                        return vec4<f32>(1.0, 0.0, 0.0, 1.0);
                    }
            "))
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                compilation_options: Default::default(),
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                compilation_options: Default::default(),
                targets: &[Some(swapchain_format.into())],
            }),
            primitive: Default::default(),
            depth_stencil: None,
            multisample: Default::default(),
            multiview: None,
            cache: None,
        });

        Graphics {
            window,
            instance,
            surface,
            surface_config,
            adapter,
            device,
            queue,
            render_pipeline,
        }
    }
}

#[allow(dead_code)]
struct Graphics {
    window: Rc<Window>,
    instance: Instance,
    surface: Surface<'static>,
    surface_config: SurfaceConfiguration,
    adapter: Adapter,
    device: Device,
    queue: Queue,
    render_pipeline: RenderPipeline,
}

struct GraphicsBuilder {
    event_loop_proxy: Option<EventLoopProxy<Graphics>>,
}

impl GraphicsBuilder {
    fn new(event_loop_proxy: EventLoopProxy<Graphics>) -> Self {
        Self {
            event_loop_proxy: Some(event_loop_proxy),
        }
    }

    fn build_and_send(&mut self, event_loop: &ActiveEventLoop) {
        let Some(event_loop_proxy) = self.event_loop_proxy.take() else {
            // event_loop_proxy is already spent - we already constructed Graphics
            return;
        };

        #[cfg(target_arch = "wasm32")]
        {
            let gfx_fut = create_graphics(event_loop);
            wasm_bindgen_futures::spawn_local(async move {
                let gfx = gfx_fut.await;
                assert!(event_loop_proxy.send_event(gfx).is_ok());
            });
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            let gfx = pollster::block_on(create_graphics(event_loop));
            assert!(event_loop_proxy.send_event(gfx).is_ok());
        }
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
    fn new(event_loop: &EventLoop<Graphics>) -> Self {
        Self {
            graphics: MaybeGraphics::Builder(GraphicsBuilder::new(event_loop.create_proxy())),
        }
    }

    fn draw(&mut self) {
        let MaybeGraphics::Graphics(gfx) = &mut self.graphics else {
            // draw call rejected because graphics doesn't exist yet
            return;
        };

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
            rpass.set_pipeline(&gfx.render_pipeline);
            rpass.draw(0..3, 0..1);
        }

        let command_buffer = encoder.finish();
        gfx.queue.submit([command_buffer]);
        frame.present();
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
