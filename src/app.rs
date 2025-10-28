use app_units::Au;
use webrender::Transaction;
use webrender_api::{Epoch, PipelineId, RenderReasons};
use winit::{application::ApplicationHandler, dpi::{LogicalSize, PhysicalPosition}, event::{MouseButton, WindowEvent}, event_loop::ActiveEventLoop};

use crate::{
    compositor::Compositor,
    layout::{
        context::LayoutContext, display_list::DisplayListBuilder, widget::{layout_root, Block, FontCache, Text, TextStyle}, Point, Rect, Size
    }
};

const WIDTH: f32 = 1024.0;
const HEIGHT: f32 = 768.0;

pub struct State {
    compositor: Compositor,
    cursor_position: PhysicalPosition<f64>,
    layout_context: LayoutContext,
    root_pipeline: PipelineId
}

pub struct App {
    state: Option<State>,
    font_cache: FontCache,
    layout_root: Block,
}

impl App {
    pub fn new() -> Self {
        let font_cache = FontCache::new();

        let text = "Hello, world!";
        let text_style = TextStyle {
            font_family: "Fira Code".to_owned(),
            font_size: 16.0,
            font_style: ttf_parser::Style::Normal,
            font_weight: ttf_parser::Weight::Normal,
            line_height: 24.0,
            color: 0x00000000,
        };
        let widget = Text {
            text: text.to_owned(),
            style: text_style,
        };

        let root = Block::new(vec![widget]);

        Self {
            state: None,
            font_cache,
            layout_root: root,
        }
    }

    pub fn run(&mut self) {
    }

    pub fn perform_layout(&mut self) {
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let size = LogicalSize { width: WIDTH, height: HEIGHT };
        let (compositor, api) = Compositor::init(event_loop, size).unwrap();
        let state = State {
            compositor,
            cursor_position: PhysicalPosition::new(0.0, 0.0),
            layout_context: LayoutContext::new(api, Size { width: Au::from_f32_px(WIDTH), height: Au::from_f32_px(HEIGHT) }),
            root_pipeline: PipelineId(0, 0)
        };
        self.state = Some(state);
    }

    fn window_event(
            &mut self,
            event_loop: &ActiveEventLoop,
            window_id: winit::window::WindowId,
            event: winit::event::WindowEvent,
        ) {
            let State {
                compositor,
                cursor_position,
                layout_context,
                root_pipeline
            } = self.state.as_mut().unwrap();
            let size = compositor.device_size();
            let scale_factor = compositor.scale_factor() as f32;
            let viewport_size = Size {
                width: Au::from_px(size.width).scale_by(1.0 / scale_factor),
                height: Au::from_px(size.height).scale_by(1.0 / scale_factor)
            };
            match event {
                WindowEvent::CloseRequested => {
                    println!("Closing...");
                    // compositor.close();
                    event_loop.exit();
                }
                WindowEvent::Resized(size) => {
                    compositor.resize(size);
                }
                WindowEvent::CursorMoved { device_id, position} => {
                    *cursor_position = position;
                }
                WindowEvent::MouseInput { device_id, state, button} => {
                    if button == MouseButton::Left {
                        println!("cursor position: {:?}", *cursor_position);
                    }
                }
                WindowEvent::RedrawRequested => {
                    let root_fragment = layout_root(&self.layout_root, layout_context, viewport_size);
                    let rect = Rect {
                        origin: Point::new(Au(0), Au(0)),
                        size: viewport_size
                    };
                    let mut dl_builder = DisplayListBuilder::new(
                        scale_factor,
                        *root_pipeline,
                        layout_context
                    );
                    dl_builder.wr.begin();

                    root_fragment.build_display_list(&mut dl_builder, rect);

                    let mut txn = Transaction::new();
                    txn.set_display_list(Epoch(0), dl_builder.wr.end());
                    txn.set_root_pipeline(*root_pipeline);
                    txn.generate_frame(0, RenderReasons::empty());

                    layout_context.webrender_api.send_transaction(layout_context.document_id, txn);

                    compositor.update();
                }
                _ => {}
            }
    }
}