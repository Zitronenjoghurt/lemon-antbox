use crate::gfx::Gfx;
use crate::ui::types::draw_mode::DrawMode;
use crate::ui::Ui;
use lemon_antbox_core::simulation::settings::SimulationSettings;
use lemon_antbox_core::threaded::event::SimulationEvent;
use lemon_antbox_core::threaded::ThreadedSimulation;
use std::sync::Arc;
use winit::event::{Event, MouseButton, WindowEvent};
use winit::event_loop::EventLoopWindowTarget;
use winit::window::Window;

pub struct App {
    gfx: Gfx,
    ui: Ui,
    simulation: ThreadedSimulation,
}

impl App {
    pub fn new(window: Arc<Window>, settings: SimulationSettings) -> Self {
        Self {
            gfx: Gfx::new(window, settings.width, settings.height),
            ui: Ui::default(),
            simulation: ThreadedSimulation::spawn(settings),
        }
    }

    pub fn handle_winit_event(
        &mut self,
        event: Event<()>,
        window_target: &EventLoopWindowTarget<()>,
    ) {
        match event {
            Event::WindowEvent { event, .. } => {
                let response = self.gfx.on_egui_window_event(&event);
                match event {
                    WindowEvent::CloseRequested => window_target.exit(),
                    WindowEvent::RedrawRequested => self.render(),
                    WindowEvent::Resized(size) if size.width > 0 && size.height > 0 => {
                        self.gfx.resize(size.width, size.height);
                    }
                    WindowEvent::CursorMoved { position, .. } => {
                        self.ui.on_cursor_moved(position);
                    }
                    WindowEvent::MouseInput { state, button, .. } if !response.consumed => {
                        self.ui.on_mouse_input(state, button);
                    }
                    WindowEvent::KeyboardInput { event, .. } => {
                        self.ui.on_keyboard_input(&self.simulation, &event);
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

    fn update(&mut self) {
        if let Some(button) = self.ui.consume_cursor_pressed()
            && let Some(coords) = self.cursor_coords()
        {
            match button {
                MouseButton::Left => match self.ui.draw_mode() {
                    DrawMode::Ant => {
                        self.simulation
                            .spawn_ant(coords.0, coords.1, self.ui.ant_tribe());
                    }
                    DrawMode::Nest => {
                        self.simulation
                            .spawn_nest(coords.0, coords.1, self.ui.nest_tribe());
                    }
                    DrawMode::Food => {
                        self.simulation
                            .spawn_food(coords.0, coords.1, self.ui.food_amount());
                    }
                },
                MouseButton::Right => {
                    self.simulation.inspect_cell(coords.0, coords.1);
                }
                _ => {}
            }
        }

        if let Some(event) = self.simulation.next_event() {
            self.handle_sim_event(event);
        }
    }

    fn handle_sim_event(&mut self, event: SimulationEvent) {
        match event {
            SimulationEvent::InspectedCell(inspected_cell) => {
                self.ui.set_inspected_cell(*inspected_cell)
            }
        }
    }

    fn render(&mut self) {
        self.gfx.prepare(|ctx| {
            self.ui.draw(ctx, &mut self.simulation);
        });

        self.simulation.draw(self.gfx.pixels_frame());
        self.gfx.render();
    }

    fn cursor_coords(&self) -> Option<(u16, u16)> {
        self.gfx
            .window_pos_to_pixel(self.ui.cursor_pos())
            .map(|(x, y)| (x as u16, y as u16))
    }
}
