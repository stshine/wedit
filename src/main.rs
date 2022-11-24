extern crate app_units;
extern crate euclid;
extern crate gleam;
extern crate glutin;
extern crate rustybuzz;
extern crate webrender;
extern crate webrender_api;
extern crate winit;

use glutin::{event_loop::EventLoop, window::WindowBuilder, dpi::LogicalSize, event::{self, Event, WindowEvent}};

const WIDTH: f32 = 1024.0;
const HEIGHT: f32 = 768.0;

fn main() {
    let event_loop = EventLoop::new();
    let window_builder = WindowBuilder::new()
        .with_title("Wedit")
        .with_inner_size(LogicalSize::new(WIDTH, HEIGHT));

    let windowed_context = glutin::ContextBuilder::new()
        .with_gl_profile(glutin::GlProfile::Core)
        .build_windowed(window_builder, &event_loop)
        .expect("Failed to create GL context");

    let context = unsafe { windowed_context.make_current().unwrap() };

    event_loop.run(|event, _, control_flow | {
        control_flow.set_wait();

        match event {
            Event::WindowEvent {
                window_id,
                event: WindowEvent::CloseRequested,
            } => {
                println!("Closing...");
                control_flow.set_exit();
            }
            _ => {}
        }
    })
}
