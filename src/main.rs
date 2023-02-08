extern crate app_units;
extern crate euclid;
extern crate gleam;
extern crate glutin;
extern crate rustybuzz;
extern crate webrender;
extern crate webrender_api;
extern crate winit;

// mod app;
mod compositor;
mod layout;
// mod layout_thread;
// mod widget;

use app_units::Au;
use compositor::Compositor;
use glutin::{event_loop::EventLoop, event::{Event, WindowEvent, MouseButton}, dpi::LogicalSize};
use layout::{widget::{TextStyle, Text, Block, layout_root}, display_list::DisplayListBuilder, Rect, Point, Size, context::LayoutContext};
use webrender::Transaction;
use webrender_api::{PipelineId, Epoch, ColorF, RenderReasons};

const WIDTH: f32 = 1024.0;
const HEIGHT: f32 = 768.0;

fn main() {
    let event_loop = EventLoop::new();
    let compositor_size = LogicalSize::new(WIDTH, HEIGHT);
    let (mut compositor, api) = Compositor::init(&event_loop, compositor_size);
    let mut cursor_position = Default::default();

    let text = "Hello, world!";

    let text_style = TextStyle {
        font_family: "Fira Code".to_owned(),
        font_size: 16.0,
        font_style: ttf_parser::Style::Normal,
        font_weight: ttf_parser::Weight::Normal,
        line_height: 24.0,
    };

    let widget = Text {
        text: text.to_owned(),
        style: text_style,
    };

    let root = Block::new(vec![widget]);

    let viewport_size = Size { width: Au::from_f32_px(WIDTH), height: Au::from_f32_px(HEIGHT) };
    let scale_factor = compositor.scale_factor() as f32;

    let root_pipeline = PipelineId(0, 0);
    let mut layout_context = LayoutContext::new(api, viewport_size);

    event_loop.run( move |event, _, control_flow | {
        control_flow.set_wait();

        match event {
            Event::WindowEvent {
                window_id,
                event,
            } => {
                match event {
                    WindowEvent::CloseRequested => {
                        println!("Closing...");
                        // compositor.close();
                        control_flow.set_exit();
                    }
                    WindowEvent::Resized(size) => {
                        compositor.resize(size);
                    }
                    WindowEvent::CursorMoved { device_id, position, modifiers } => {
                        cursor_position = position;
                    }
                    WindowEvent::MouseInput { device_id, state, button, modifiers } => {
                        if button == MouseButton::Left {
                            println!("cursor position: {:?}", cursor_position);
                        }
                    }
                    _ => {}
                }
            }
            Event::RedrawRequested(_) => {
                let root_fragment = layout_root(&root, &mut layout_context, viewport_size);
                let rect = Rect {
                    origin: Point::new(Au(0), Au(0)),
                    size: viewport_size
                };
                let mut dl_builder = DisplayListBuilder::new(scale_factor, root_pipeline, &mut layout_context);
                dl_builder.wr.begin();

                root_fragment.build_display_list(&mut dl_builder, rect);

                let mut txn = Transaction::new();
                txn.set_display_list(
                    Epoch(0),
                    Some(ColorF { r: 0.1, g: 0.1, b: 0.1, a: 1.0 }),
                    viewport_size.to_layout(scale_factor),
                    dl_builder.wr.end(),
                );
                txn.set_root_pipeline(root_pipeline);
                txn.generate_frame(0, RenderReasons::empty());

                layout_context.webrender_api.send_transaction(layout_context.document_id, txn);

                compositor.update();

            }
            _ => {}
        }
    })
}
