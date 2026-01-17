use crate::ui::types::draw_mode::DrawMode;
use crate::ui::windows::main::{MainWindow, MainWindowState};
use crate::ui::windows::UiWindow;
use lemon_antbox_core::threaded::event::InspectedCell;
use lemon_antbox_core::threaded::ThreadedSimulation;
use winit::dpi::PhysicalPosition;
use winit::event::{ElementState, KeyEvent, MouseButton};
use winit::keyboard::{KeyCode, PhysicalKey};

pub mod types;
mod widgets;
mod windows;

#[derive(Default)]
pub struct Ui {
    cursor_pos: (f32, f32),
    cursor_pressed: Option<MouseButton>,
    main_window: MainWindowState,
}

impl Ui {
    pub fn draw(&mut self, ctx: &egui::Context, sim: &mut ThreadedSimulation) {
        MainWindow::new(&mut self.main_window, sim).show(ctx);
    }

    pub fn on_cursor_moved(&mut self, pos: PhysicalPosition<f64>) {
        self.cursor_pos = (pos.x as f32, pos.y as f32)
    }

    pub fn on_mouse_input(&mut self, state: ElementState, button: MouseButton) {
        if state == ElementState::Pressed {
            self.cursor_pressed = Some(button);
        } else {
            self.cursor_pressed = None;
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

    pub fn consume_cursor_pressed(&mut self) -> Option<MouseButton> {
        // The left mouse button will allow for being kept pressed
        if let Some(MouseButton::Left) = self.cursor_pressed {
            return Some(MouseButton::Left);
        }
        self.cursor_pressed.take()
    }

    pub fn set_inspected_cell(&mut self, inspected_cell: InspectedCell) {
        self.main_window
            .cell_inspector_window_state
            .inspect(inspected_cell);
    }
}

// Window state helpers
impl Ui {
    pub fn draw_mode(&self) -> DrawMode {
        self.main_window.draw_settings.draw_mode
    }

    pub fn ant_tribe(&self) -> u8 {
        self.main_window.draw_settings.ant_tribe
    }

    pub fn nest_tribe(&self) -> u8 {
        self.main_window.draw_settings.nest_tribe
    }

    pub fn food_amount(&self) -> u8 {
        self.main_window.draw_settings.food_amount
    }
}
