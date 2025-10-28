extern crate app_units;
extern crate euclid;
extern crate gleam;
extern crate glutin;
extern crate rustybuzz;
extern crate webrender;
extern crate webrender_api;
extern crate winit;

mod app;
mod compositor;
mod layout;
// mod layout_thread;
// mod widget;

use winit::event_loop::EventLoop;

use crate::app::App;

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let mut app = App::new();

    event_loop.run_app(&mut app).unwrap();
}
