use glutin::{event_loop::EventLoop, dpi::{LogicalSize, PhysicalSize}, window::{WindowBuilder, Window}, ContextWrapper, PossiblyCurrent};
use webrender::{Renderer, WebRenderOptions, create_webrender_instance, RenderApi, RenderApiSender};
use webrender_api::{ColorF, RenderNotifier, DocumentId, units::DeviceIntSize};

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
    sender: RenderApiSender,
    document_id: DocumentId,
}

impl Compositor {
    pub fn init(event_loop: &EventLoop<()>, size: LogicalSize<f32>) -> (Compositor, RenderApi) {
        let window_builder = WindowBuilder::new()
            .with_title("Wedit")
            .with_inner_size(size);

        let windowed_context = glutin::ContextBuilder::new()
            .with_gl_profile(glutin::GlProfile::Core)
            .build_windowed(window_builder, event_loop)
            .expect("Failed to create GL context");


        // Keep this or window will be closed.
        let context = unsafe { windowed_context.make_current().unwrap() };

        let gl = unsafe {
            gleam::gl::GlFns::load_with(|symbol| context.get_proc_address(symbol))
        };

        let initial_size = {
            let size = context.window().inner_size();
            webrender_api::units::DeviceIntSize::new(size.width as i32, size.height as i32)
        };

        println!("The DPI scale factor is: {}", context.window().scale_factor());

        let webrender_options = WebRenderOptions {
            enable_aa: true,
            enable_subpixel_aa: true,
            clear_color: ColorF::new(1.0, 1.0, 1.0, 0.0),
            ..Default::default()
        };

        let notifier = Box::new(Notifier {});

        let (renderer, sender) = create_webrender_instance(gl, notifier, webrender_options, None).unwrap();
        let api = sender.create_api();

        let document_id = api.add_document(initial_size);

        let compositor = Compositor {
            context,
            renderer,
            sender,
            document_id,
        };
        (compositor, api)
    }

    pub fn device_size(&self) -> DeviceIntSize {
        let size = self.context.window().inner_size();
        DeviceIntSize::new(size.width as i32, size.height as i32)
    }

    pub fn scale_factor(&self) -> f64 {
        self.context.window().scale_factor()
    }

    pub fn update(&mut self) {
        self.renderer.update();
        self.renderer.render(self.device_size(), 0).unwrap();
        let _ = self.renderer.flush_pipeline_info();
        self.context.swap_buffers().unwrap();
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.context.resize(size);
        self.renderer.update();
    }

    pub fn close(self) {
        self.renderer.deinit();
    }
}
