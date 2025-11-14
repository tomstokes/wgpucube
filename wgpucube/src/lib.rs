#![cfg(target_os = "android")]

mod app;
mod cube;
mod egui;

use app::{App, WgpuEvent};
use winit::event_loop::EventLoop;
use winit::platform::android::activity::AndroidApp;

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
