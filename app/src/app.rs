use crate::gfx::Gfx;
use crate::ui::Ui;
use lemon_antbox_core::threaded::ThreadedSimulation;
use std::sync::Arc;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoopWindowTarget;
use winit::window::Window;

pub struct App {
    gfx: Gfx,
    ui: Ui,
    simulation: ThreadedSimulation,
}

impl App {
    pub fn new(window: Arc<Window>, width: u32, height: u32) -> Self {
        Self {
            gfx: Gfx::new(window, width, height),
            ui: Ui::default(),
            simulation: ThreadedSimulation::spawn(height, width),
        }
    }

    pub fn handle_event(&mut self, event: Event<()>, window_target: &EventLoopWindowTarget<()>) {
        match event {
            Event::WindowEvent { event, .. } => {
                let response = self.gfx.on_egui_window_event(&event);
                match event {
                    WindowEvent::CloseRequested => window_target.exit(),
                    WindowEvent::RedrawRequested => self.render(),
                    WindowEvent::Resized(size) if size.width > 0 && size.height > 0 => {
                        self.gfx.resize(size.width, size.height);
                    }
                    WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                        self.gfx.set_pixels_per_point(scale_factor as f32);
                    }
                    WindowEvent::CursorMoved { position, .. } => {
                        self.ui.on_cursor_moved(position);
                    }
                    WindowEvent::MouseInput { state, button, .. } if !response.consumed => {
                        self.ui.on_mouse_input(state, button);
                    }
                    _ => {}
                }
            }
            Event::AboutToWait => {
                self.update();
                self.gfx.request_redraw();
            }
            _ => {}
        }
    }

    fn update(&mut self) {}

    fn render(&mut self) {
        self.simulation.draw(self.gfx.pixels_frame());

        self.gfx.prepare(|ctx| {
            self.ui.draw(ctx);
        });

        self.gfx.render();
    }
}
