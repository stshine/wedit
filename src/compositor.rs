use glutin::{event_loop::EventLoop, dpi::LogicalSize, window::{WindowBuilder, Window}, ContextWrapper, PossiblyCurrent};
use webrender::{Renderer, WebRenderOptions, create_webrender_instance};
use webrender_api::{ColorF, RenderNotifier};

const WIDTH: f32 = 1024.0;
const HEIGHT: f32 = 768.0;

struct Notifier {}

impl RenderNotifier for Notifier {
    fn wake_up(
            &self,
            composite_needed: bool,
        ) {

    }

    fn clone(&self) -> Box<dyn RenderNotifier> {
        Box::new(Notifier {})
    }

    fn new_frame_ready(&self, _: webrender_api::DocumentId, scrolled: bool, composite_needed: bool) {
        self.wake_up(composite_needed);
    }

    fn external_event(&self, _evt: webrender_api::ExternalEvent) {

    }

    fn shut_down(&self) {

    }
}

pub struct Compositor {
    context: ContextWrapper<PossiblyCurrent, Window>,
    renderer: Renderer,
}

impl Compositor {
    pub fn init(event_loop: &EventLoop<()>) -> Self {
        let window_builder = WindowBuilder::new()
            .with_title("Wedit")
            .with_inner_size(LogicalSize::new(WIDTH, HEIGHT));

        let windowed_context = glutin::ContextBuilder::new()
            .with_gl_profile(glutin::GlProfile::Core)
            .build_windowed(window_builder, event_loop)
            .expect("Failed to create GL context");


        // Keep this or window will be closed.
        let context = unsafe { windowed_context.make_current().unwrap() };

        let gl = unsafe {
            gleam::gl::GlFns::load_with(|symbol| context.get_proc_address(symbol))
        };

        let webrender_options = WebRenderOptions {
            enable_aa: true,
            enable_subpixel_aa: true,
            clear_color: ColorF::new(1.0, 1.0, 1.0, 0.0),
            ..Default::default()
        };

        let notifier = Box::new(Notifier {});

        let (renderer, sender) = create_webrender_instance(gl, notifier, webrender_options, None).unwrap();

        Compositor {
            context,
            renderer,
        }
    }
}