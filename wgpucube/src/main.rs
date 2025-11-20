#![cfg(not(target_os = "android"))]

mod app;
mod cube;
#[cfg(feature = "egui")]
mod egui;

use app::{App, WgpuEvent};
use tracing::info;
use winit::event_loop::EventLoop;

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
    #[cfg_attr(target_arch = "wasm32", expect(unused_mut))]
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
