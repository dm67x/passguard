use imgui::{im_str, Condition, Window};

mod context;
mod gui;

fn main() {
    let ctx = context::Context::new();
    ctx.run(
        |ui| {
            Window::new(im_str!("Hello world"))
                .size([300.0, 110.0], Condition::FirstUseEver)
                .build(ui, || {
                    ui.text(im_str!("Hello world!"));
                    ui.separator();
                    let mouse_pos = ui.io().mouse_pos;
                    ui.text(format!(
                        "Mouse Position: ({:.1},{:.1})",
                        mouse_pos[0], mouse_pos[1]
                    ));
                });
            true
        },
        |_| true,
    );
}
