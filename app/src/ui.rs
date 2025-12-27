use winit::dpi::PhysicalPosition;
use winit::event::{ElementState, MouseButton};

#[derive(Default)]
pub struct Ui {
    cursor_pos: (f64, f64),
    cursor_pressed: bool,
    paused: bool,
}

impl Ui {
    pub fn draw(&mut self, ctx: &egui::Context) {
        egui::Window::new("Tools").show(ctx, |ui| {
            ui.checkbox(&mut self.paused, "Paused");
            if ui.button("Clear").clicked() {}
        });
    }

    pub fn on_cursor_moved(&mut self, pos: PhysicalPosition<f64>) {
        self.cursor_pos = (pos.x, pos.y)
    }

    pub fn on_mouse_input(&mut self, state: ElementState, button: MouseButton) {
        if button == MouseButton::Left {
            self.cursor_pressed = state == ElementState::Pressed;
        }
    }
}
