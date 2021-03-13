use glium::{
    glutin::{
        self,
        event::{Event, WindowEvent},
        event_loop::ControlFlow,
    },
    Surface,
};
use imgui::{im_str, Condition, Context, FontConfig, FontSource, Ui, Window};
use imgui_glium_renderer::Renderer;
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use std::time::Instant;

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new().with_title("Passguard");
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();
    let mut imgui = Context::create();
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
    let mut renderer = Renderer::init(&mut imgui, &display).expect("Failed to initialize renderer");

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
            if !draw(&mut ui) {
                *control_flow = ControlFlow::Exit;
            }

            let gl_window = display.gl_window();
            let mut target = display.draw();
            target.clear_color_srgb(0.0, 0.0, 0.0, 1.0);
            platform.prepare_render(&ui, gl_window.window());
            let draw_data = ui.render();
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

fn draw(ui: &mut Ui) -> bool {
    Window::new(im_str!("Hello world"))
        .size([300.0, 110.0], Condition::FirstUseEver)
        .build(ui, || {
            ui.text(im_str!("Hello world!"));
            ui.text(im_str!("こんにちは世界！"));
            ui.text(im_str!("This...is...imgui-rs!"));
            ui.separator();
            let mouse_pos = ui.io().mouse_pos;
            ui.text(format!(
                "Mouse Position: ({:.1},{:.1})",
                mouse_pos[0], mouse_pos[1]
            ));
        });
    true
}
