use crate::ui::windows::main::{MainWindow, MainWindowState};
use crate::ui::windows::UiWindow;
use lemon_antbox_core::threaded::ThreadedSimulation;
use winit::dpi::PhysicalPosition;
use winit::event::{ElementState, KeyEvent, MouseButton};
use winit::keyboard::{KeyCode, PhysicalKey};

mod widgets;
mod windows;

#[derive(Default)]
pub struct Ui {
    cursor_pos: (f32, f32),
    cursor_pressed: bool,
    main_window: MainWindowState,
}

impl Ui {
    pub fn draw(&mut self, ctx: &egui::Context, sim: &ThreadedSimulation) {
        MainWindow::new(&mut self.main_window).show(ctx, sim);
    }

    pub fn on_cursor_moved(&mut self, pos: PhysicalPosition<f64>) {
        self.cursor_pos = (pos.x as f32, pos.y as f32)
    }

    pub fn on_mouse_input(&mut self, state: ElementState, button: MouseButton) {
        if button == MouseButton::Left {
            self.cursor_pressed = state == ElementState::Pressed;
        }
    }

    pub fn on_keyboard_input(&mut self, sim: &ThreadedSimulation, event: &KeyEvent) {
        let PhysicalKey::Code(code) = event.physical_key else {
            return;
        };

        match code {
            KeyCode::Escape => {
                if event.state == ElementState::Pressed {
                    self.main_window.is_open = !self.main_window.is_open
                }
            }
            KeyCode::Space => {
                if event.state == ElementState::Pressed {
                    sim.toggle_paused()
                }
            }
            _ => {}
        }
    }

    pub fn cursor_pos(&self) -> (f32, f32) {
        self.cursor_pos
    }

    pub fn cursor_pressed(&self) -> bool {
        self.cursor_pressed
    }
}
