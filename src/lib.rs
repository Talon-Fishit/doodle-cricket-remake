use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use wry::WebViewBuilder;

#[cfg(target_os = "android")]
use tao::platform::android::EventLoopExtAndroid;

#[cfg_attr(target_os = "android", ndk_glue::main)]
pub fn main() {
    #[cfg(target_os = "android")]
    android_logger::init_once(
        android_logger::Config::default().with_min_level(log::Level::Debug),
    );

    log::info!("Starting Doodle Cricket Android app in Rust");

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Doodle Cricket - Craft Anything")
        .build(&event_loop)
        .unwrap();

    let _webview = WebViewBuilder::new(&window)
        .with_url("file:///android_asset/index.html")
        .build()
        .unwrap();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => {},
        }
    });
}
