extern crate app_units;
extern crate euclid;
extern crate gleam;
extern crate glutin;
extern crate rustybuzz;
extern crate webrender;
extern crate webrender_api;
extern crate winit;

mod compositor;

use compositor::Compositor;
use glutin::{event_loop::EventLoop, event::{Event, WindowEvent}};

fn main() {
    let event_loop = EventLoop::new();
    let compositor = Compositor::init(&event_loop);
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
