use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowId};

struct App {
    window: Option<Window>,
}

impl App {
    fn new() -> Self {
        Self { window: None }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = Window::default_attributes().with_title("Omnicube");
        self.window = Some(event_loop.create_window(window_attributes).unwrap());
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::Resized(_) => {
                // Resize logic
            }
            WindowEvent::CloseRequested => {
                self.window = None;
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                // Redraw logic
            }
            _ => {}
        }
    }
}

fn main() {
    let event_loop = EventLoop::new();
    let mut app = App::new();
    event_loop.unwrap().run_app(&mut app).unwrap()
}
