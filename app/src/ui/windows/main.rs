use crate::ui::windows::simulation_settings::SimulationSettingsWindow;
use crate::ui::windows::{ToggleableUiWindow, UiWindow};
use egui::{Id, Ui, WidgetText};
use lemon_antbox_core::threaded::ThreadedSimulation;

pub struct MainWindowState {
    pub is_open: bool,
    simulation_settings_open: bool,
}

impl Default for MainWindowState {
    fn default() -> Self {
        Self {
            is_open: true,
            simulation_settings_open: false,
        }
    }
}

pub struct MainWindow<'a> {
    state: &'a mut MainWindowState,
}

impl<'a> MainWindow<'a> {
    pub fn new(state: &'a mut MainWindowState) -> Self {
        Self { state }
    }
}

impl UiWindow for MainWindow<'_> {
    fn id() -> Id {
        Id::new("main_window")
    }

    fn title() -> impl Into<WidgetText> {
        "Ant Box"
    }

    fn is_open(&self) -> bool {
        self.state.is_open
    }

    fn set_open(&mut self, open: bool) {
        self.state.is_open = open;
    }

    fn render_content(&mut self, ui: &mut Ui, sim: &ThreadedSimulation) {
        SimulationSettingsWindow::new(&mut self.state.simulation_settings_open)
            .toggle_button(ui)
            .show(ui.ctx(), sim);
    }
}
