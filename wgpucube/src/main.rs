mod cube;

use crate::cube::Cube;
use std::sync::Arc;
use tracing::{error, info, warn};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop, EventLoopProxy};
use winit::window::{Window, WindowId};

#[derive(Debug)]
struct Context {
    device: wgpu::Device,
    queue: wgpu::Queue,
    size: winit::dpi::PhysicalSize<u32>,
    surface: wgpu::Surface<'static>,
    surface_format: wgpu::TextureFormat,
    cube: Cube,
}

impl Context {
    async fn new(window: Arc<Window>) -> Self {
        let instance_descriptor = wgpu::InstanceDescriptor::default();
        let instance = wgpu::Instance::new(&instance_descriptor);

        // Note: Surface creation can only occur after the .resume() call from winit as Android does
        //       not allow surface creation prior to Resume. However, on WebGL backends the surface
        //       must be created before requesting an adapter. This example waits until .resume()
        //       to create the instance and request an adapter, avoiding complications. If the code
        //       is refactored so that instance creation occurs before winit calls .resume() then
        //       additional logic would be required to create the surface at the appropriate point
        //       for different platforms.
        let surface = instance.create_surface(Arc::clone(&window)).unwrap();

        let request_adapter_options = wgpu::RequestAdapterOptions::default();
        let adapter = instance
            .request_adapter(&request_adapter_options)
            .await
            .unwrap();
        info!("Using adapter: {:?}", adapter.get_info().backend);
        let device_descriptor = wgpu::DeviceDescriptor::default();
        let (device, queue) = adapter.request_device(&device_descriptor).await.unwrap();

        // Note: window.inner_size() is only valid after instance.request_adapter() on web
        let size = window.inner_size();

        let surface_capabilities = surface.get_capabilities(&adapter);
        let surface_format = surface_capabilities.formats[0];

        let cube = Cube::new(surface_format, &device);

        let context = Self {
            device,
            queue,
            size,
            surface,
            surface_format,
            cube,
        };
        context.configure_surface();

        context
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
            // TODO: Investigate sRGB surfaces ( surface_format.add_srgb_suffix() )
            view_formats: vec![self.surface_format],
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
            // TODO: Investigate sRGB surfaces ( surface_format.add_srgb_suffix() )
            format: Some(self.surface_format),
            ..Default::default()
        };
        let texture_view = surface_texture
            .texture
            .create_view(&texture_view_descriptor);

        self.cube.render(&texture_view, &self.device, &self.queue);

        surface_texture.present();
    }
}

#[derive(Default)]
#[allow(clippy::large_enum_variant)]
enum State {
    #[default]
    Uninitialized,
    Initializing,
    Resumed {
        window: Arc<Window>,
        context: Context,
    },
}

#[derive(Debug)]
enum WgpuEvent {
    #[cfg_attr(not(target_arch = "wasm32"), allow(unused))]
    Initialized {
        window: Arc<Window>,
        context: Context,
    },
}

struct App {
    #[cfg_attr(not(target_arch = "wasm32"), allow(unused))]
    event_loop_proxy: EventLoopProxy<WgpuEvent>,
    state: State,
    #[cfg(target_os = "ios")]
    request_redraw: bool,
}

impl App {
    fn new(event_loop_proxy: EventLoopProxy<WgpuEvent>) -> Self {
        Self {
            event_loop_proxy,
            state: State::Uninitialized,
            #[cfg(target_os = "ios")]
            request_redraw: false,
        }
    }
}

impl ApplicationHandler<WgpuEvent> for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // TODO: Be tolerante of multiple calls to .resumed()
        // Winit documentation states that .resumed() can be called multiple times on some
        // platforms. This should be refactored to be tolerante of multiple calls. Currently only
        // wasm platforms utilize State::Initializing so other platforms will only need to handle
        // the case where .resumed() is called when State::Resumed is already true. Need to
        // determine appropriate behavior in this case.
        match self.state {
            State::Uninitialized => self.state = State::Initializing,
            State::Initializing => panic!("Call to .resumed() but state is State::Initializing"),
            State::Resumed { .. } => panic!("Call to .resumed() but state is State::Resumed"),
        }

        let mut window_attributes = Window::default_attributes();
        window_attributes = window_attributes.with_title("wgpucube");

        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::JsCast;
            use winit::platform::web::WindowAttributesExtWebSys;

            let canvas = web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .get_element_by_id("wgpucube-canvas")
                .unwrap()
                .dyn_into::<web_sys::HtmlCanvasElement>()
                .unwrap();
            window_attributes = window_attributes.with_canvas(Some(canvas));
        }

        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());

        cfg_if::cfg_if! {
            if #[cfg(target_arch = "wasm32")] {
                let event_loop_proxy = self.event_loop_proxy.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let context = Context::new(Arc::clone(&window)).await;
                    event_loop_proxy.send_event(WgpuEvent::Initialized { window, context }).unwrap();
                });
            } else {
                let context = pollster::block_on(Context::new(Arc::clone(&window)));
                self.state = State::Resumed {
                    window: Arc::clone(&window),
                    context,
                };
                window.request_redraw();
            }
        }
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, event: WgpuEvent) {
        match event {
            WgpuEvent::Initialized { context, window } => {
                // TODO: Is it safe to assume state will be initializing?
                assert!(matches!(self.state, State::Initializing));
                self.state = State::Resumed {
                    context,
                    window: Arc::clone(&window),
                };
                window.request_redraw();
            }
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match &mut self.state {
            State::Uninitialized => {
                error!("Received WindowEvent in State::Uninitialized: {:?}", event);
            }
            State::Initializing => {
                // TODO: Should these events be queued up and processed when initialization is done?
                warn!("Dropped event during initialization: {:?}", event);
            }
            State::Resumed { window, context } => {
                match event {
                    WindowEvent::Resized(new_size) => {
                        // TODO: If cube stays in context then should context call cube.resize?
                        context.cube.resize(new_size, &context.queue);
                        context.resize(new_size)
                        // Winit will automatically provide a RedrawRequested event after this event
                    }
                    WindowEvent::CloseRequested => {
                        // TODO: Could drop resources here for cleanup
                        event_loop.exit();
                    }
                    WindowEvent::RedrawRequested => {
                        // TODO: Is this correct order for pre_present_notify and render?
                        window.pre_present_notify();
                        context.render();
                        // Calling window.request_redraw() during a WindowEvent::RedrawRequested
                        // does not work properly on iOS. As a workaround, the request_redraw flag
                        // is set. The about_to_wait() method checks this flag and calls
                        // window.request_redraw(), which appears to work.
                        //
                        // Issue: https://github.com/rust-windowing/winit/issues/3406
                        cfg_if::cfg_if! {
                            if #[cfg(target_os = "ios")] {
                                self.request_redraw = true;
                            } else {
                                window.request_redraw();
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    #[cfg(target_os = "ios")]
    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let State::Resumed { window, .. } = &self.state {
            // See the comment above about request_redraw workaround on iOS
            if std::mem::take(&mut self.request_redraw) {
                window.request_redraw();
            }
        }
    }
}

#[cfg(not(target_os = "android"))]
fn main() {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            wasm_tracing::set_as_global_default();
        } else {
            let subscriber = tracing_subscriber::FmtSubscriber::new();
            tracing::subscriber::set_global_default(subscriber).unwrap();
        }
    }

    info!("Starting wgpucube");

    let event_loop = EventLoop::<WgpuEvent>::with_user_event().build().unwrap();
    let event_loop_proxy = event_loop.create_proxy();
    #[cfg_attr(target_arch = "wasm32", allow(unused))]
    let mut app = App::new(event_loop_proxy);

    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            use winit::platform::web::EventLoopExtWebSys;
            event_loop.spawn_app(app);
        } else {
            event_loop.run_app(&mut app).unwrap();
        }
    }
}

// Android entry point
// TODO: Refactor to move this to lib.rs to silence warning about using same file for lib and bin

#[cfg(target_os = "android")]
use winit::platform::android::activity::AndroidApp;

#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
fn android_main(android_app: AndroidApp) {
    use winit::platform::android::EventLoopBuilderExtAndroid;
    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::Level::Trace.to_level_filter()),
    );

    let event_loop = EventLoop::<WgpuEvent>::with_user_event()
        .with_android_app(android_app)
        .build()
        .unwrap();
    let event_loop_proxy = event_loop.create_proxy();
    let mut app = App::new(event_loop_proxy);
    event_loop.run_app(&mut app).unwrap();
}
