use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowId};

struct State {
    window: Arc<Window>,
    size: winit::dpi::PhysicalSize<u32>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface<'static>,
    surface_format: wgpu::TextureFormat,
}

impl State {
    async fn new(window: Arc<Window>) -> State {
        let size = window.inner_size();
        let instance_descriptor = wgpu::InstanceDescriptor::default();
        let instance = wgpu::Instance::new(&instance_descriptor);
        let request_adapter_options = wgpu::RequestAdapterOptions::default();
        let adapter = instance
            .request_adapter(&request_adapter_options)
            .await
            .unwrap();
        let device_descriptor = wgpu::DeviceDescriptor::default();
        let (device, queue) = adapter.request_device(&device_descriptor).await.unwrap();

        let surface = instance.create_surface(window.clone()).unwrap();
        let surface_capabilities = surface.get_capabilities(&adapter);
        let surface_format = surface_capabilities.formats[0];

        let state = State {
            window,
            device,
            queue,
            size,
            surface,
            surface_format,
        };
        state.configure_surface();

        state
    }

    fn configure_surface(&self) {
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: self.surface_format,
            width: self.size.width,
            height: self.size.height,
            present_mode: wgpu::PresentMode::AutoVsync,
            desired_maximum_frame_latency: 2,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![self.surface_format.add_srgb_suffix()],
        };
        self.surface.configure(&self.device, &surface_config);
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        self.configure_surface();
    }

    fn render(&mut self) {
        let surface_texture = self.surface.get_current_texture().unwrap();
        let texture_view_descriptor = wgpu::TextureViewDescriptor {
            format: Some(self.surface_format.add_srgb_suffix()),
            ..Default::default()
        };
        let texture_view = surface_texture
            .texture
            .create_view(&texture_view_descriptor);

        let mut encoder = self.device.create_command_encoder(&Default::default());

        // Begin the renderpass
        let renderpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &texture_view,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::RED),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        // Draw commands go here

        // End the renderpass
        drop(renderpass);

        self.queue.submit([encoder.finish()]);
        self.window.pre_present_notify();
        surface_texture.present();
    }
}

struct App {
    state: Option<State>,
}

impl App {
    fn new() -> Self {
        Self { state: None }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = Window::default_attributes().with_title("Omnicube");
        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());
        let state = pollster::block_on(State::new(Arc::clone(&window)));
        self.state = Some(state);
        window.request_redraw();
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        if let Some(state) = &mut self.state {
            match event {
                WindowEvent::Resized(new_size) => {
                    state.resize(new_size)
                    // Winit will automatically provide a RedrawRequested event after this event
                }
                WindowEvent::CloseRequested => {
                    self.state = None;
                    event_loop.exit();
                }
                WindowEvent::RedrawRequested => {
                    state.render();
                    state.window.request_redraw();
                }
                _ => {}
            }
        }
    }
}

fn main() {
    let event_loop = EventLoop::new();
    let mut app = App::new();
    event_loop.unwrap().run_app(&mut app).unwrap()
}
