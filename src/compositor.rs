use std::{ffi::CString, num::NonZeroU32};

use glutin::{
    config::{Api, Config, ConfigTemplateBuilder, GlConfig},
    context::{ContextAttributesBuilder, GlProfile, PossiblyCurrentContext},
    display::GetGlDisplay,
    prelude::{GlDisplay, NotCurrentGlContext, PossiblyCurrentGlContext},
    surface::{GlSurface, Surface, WindowSurface}
};
use glutin_winit::GlWindow;
use webrender::{Renderer, WebRenderOptions, create_webrender_instance, RenderApi, RenderApiSender};
use webrender_api::{ColorF, RenderNotifier, DocumentId, units::DeviceIntSize};
use winit::{
    dpi::{LogicalSize, PhysicalSize},
    event_loop::ActiveEventLoop,
    raw_window_handle::HasWindowHandle,
    window::Window
};

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

    fn new_frame_ready(&self, _: DocumentId, scrolled: bool, composite_needed: bool, frame_publish_id: webrender_api::FramePublishId) {
        self.wake_up(composite_needed);
    }

    fn external_event(&self, _evt: webrender_api::ExternalEvent) {

    }

    fn shut_down(&self) {

    }
}

pub struct Compositor {
    context: PossiblyCurrentContext,
    window: Window,
    renderer: Renderer,
    surface: Surface<WindowSurface>,
    sender: RenderApiSender,
    document_id: DocumentId,
}

impl Compositor {
    pub fn init(event_loop: &ActiveEventLoop, size: LogicalSize<f32>) -> anyhow::Result<(Compositor, RenderApi)> {
        let window_attributes = Window::default_attributes()
            .with_title("Wedit")
            .with_inner_size(size);

        let display_builder = glutin_winit::DisplayBuilder::new()
            .with_preference(glutin_winit::ApiPreference::PreferEgl)
            .with_window_attributes(Some(window_attributes));

        let gl_template = ConfigTemplateBuilder::new()
            .with_api(Api::OPENGL);


        let (window, config) = display_builder.build(
            event_loop,
            gl_template,
            gl_config_picker
        ).unwrap();
        let window = window.unwrap();
        let gl_display = config.display();

        let gl_attributes = ContextAttributesBuilder::new()
            .with_profile(GlProfile::Core)
            .build(window.window_handle().ok().map(|wh|wh.as_raw()));

        let context = unsafe { gl_display.create_context(&config, &gl_attributes)? };


        // Keep this or window will be closed.
        let context = context.treat_as_possibly_current();

        let gl = unsafe {
            gleam::gl::GlFns::load_with(|symbol| {
                let symbol = CString::new(symbol).unwrap();
                gl_display.get_proc_address(&symbol)
            })
        };
        let initial_size = {
            let size = window.inner_size();
            webrender_api::units::DeviceIntSize::new(size.width as i32, size.height as i32)
        };
        let surface_attributes = window.build_surface_attributes(Default::default())?;

        let surface = unsafe {
            gl_display.create_window_surface(&config, &surface_attributes)?
        };
        context.make_current(&surface)?;

        println!("The DPI scale factor is: {}", window.scale_factor());

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
            window,
            surface,
            renderer,
            sender,
            document_id,
        };
        Ok((compositor, api))
    }

    pub fn device_size(&self) -> DeviceIntSize {
        let size = self.window.inner_size();
        DeviceIntSize::new(size.width as i32, size.height as i32)
    }

    pub fn scale_factor(&self) -> f64 {
        self.window.scale_factor()
    }

    pub fn update(&mut self) {
        self.renderer.update();
        self.renderer.render(self.device_size(), 0).unwrap();
        let _ = self.renderer.flush_pipeline_info();
        self.surface.swap_buffers(&self.context).unwrap();
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        let width = NonZeroU32::new(size.width).unwrap();
        let height = NonZeroU32::new(size.height).unwrap();
        self.surface.resize(&self.context, width, height);
        self.renderer.update();
    }

    pub fn close(self) {
        self.renderer.deinit();
    }
}

// Find the config with the maximum number of samples, so our triangle will be
// smooth.
pub fn gl_config_picker(configs: Box<dyn Iterator<Item = Config> + '_>) -> Config {
    configs
        .reduce(|accum, config| {
            let transparency_check = config.supports_transparency().unwrap_or(false)
                & !accum.supports_transparency().unwrap_or(false);

            if transparency_check || config.num_samples() > accum.num_samples() {
                config
            } else {
                accum
            }
        })
        .unwrap()
}