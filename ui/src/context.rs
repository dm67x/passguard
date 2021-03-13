use glium::{
    glutin::{
        self,
        event::{Event, WindowEvent},
        event_loop::ControlFlow,
    },
    Frame, Surface,
};
use glutin::event_loop::EventLoop;
use imgui::{FontConfig, FontSource, Ui};
use imgui_glium_renderer::Renderer;
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use std::time::Instant;

pub struct Context {
    pub event_loop: EventLoop<()>,
    pub display: glium::Display,
    pub imgui: imgui::Context,
    pub platform: WinitPlatform,
    pub renderer: Renderer,
    pub font_size: f32,
}

impl Context {
    pub fn new() -> Self {
        let event_loop = EventLoop::new();
        let wb = glutin::window::WindowBuilder::new().with_title("Passguard");
        let cb = glutin::ContextBuilder::new();
        let display = glium::Display::new(wb, cb, &event_loop).unwrap();
        let mut imgui = imgui::Context::create();
        imgui.set_ini_filename(None);
        let mut platform = WinitPlatform::init(&mut imgui);
        {
            let gl_window = display.gl_window();
            let window = gl_window.window();
            platform.attach_window(imgui.io_mut(), window, HiDpiMode::Rounded);
        }
        let hidpi_factor = platform.hidpi_factor();
        let font_size = (13.0 * hidpi_factor) as f32;
        imgui.fonts().add_font(&[FontSource::DefaultFontData {
            config: Some(FontConfig {
                size_pixels: font_size,
                ..FontConfig::default()
            }),
        }]);
        imgui.io_mut().font_global_scale = (1.0 / hidpi_factor) as f32;
        let renderer = Renderer::init(&mut imgui, &display).expect("Failed to initialize renderer");

        Self {
            event_loop,
            display,
            imgui,
            platform,
            renderer,
            font_size,
        }
    }

    pub fn run<F, G>(self, mut ui_render: F, mut render: G)
    where
        F: FnMut(&mut Ui) -> bool + 'static,
        G: FnMut(&mut Frame) -> bool + 'static,
    {
        let Context {
            event_loop,
            display,
            mut imgui,
            mut platform,
            mut renderer,
            ..
        } = self;

        let mut last_frame = Instant::now();

        event_loop.run(move |event, _, control_flow| match event {
            Event::NewEvents(_) => {
                let now = Instant::now();
                imgui.io_mut().update_delta_time(now - last_frame);
                last_frame = now;
            }
            Event::MainEventsCleared => {
                let gl_window = display.gl_window();
                platform
                    .prepare_frame(imgui.io_mut(), gl_window.window())
                    .expect("Failed to prepare frame");
                gl_window.window().request_redraw();
            }
            Event::RedrawRequested(_) => {
                let mut ui = imgui.frame();
                if !ui_render(&mut ui) {
                    *control_flow = ControlFlow::Exit;
                }

                let gl_window = display.gl_window();
                let mut target = display.draw();
                target.clear_color_srgb(0.0, 0.0, 0.0, 1.0);
                platform.prepare_render(&ui, gl_window.window());
                let draw_data = ui.render();
                if !render(&mut target) {
                    *control_flow = ControlFlow::Exit;
                }
                renderer
                    .render(&mut target, draw_data)
                    .expect("Rendering failed");
                target.finish().expect("Failed to swap buffers");
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            event => {
                let gl_window = display.gl_window();
                platform.handle_event(imgui.io_mut(), gl_window.window(), &event);
            }
        });
    }
}
